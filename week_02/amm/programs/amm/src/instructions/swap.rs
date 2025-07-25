use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer, TransferChecked}, token_2022::spl_token_2022::extension::cpi_guard::CpiGuard,
};
use constant_product_curve::{ConstantProduct,LiquidityPair};

use crate::{error::AmmError, state::Config};

// this instruction is for users, in order to swap their tokens 
/*
    accounts used:
    - user
    - mint_x, mint_y, mint_lp
    - config
    - vault_x, vault_y,
    - user_x, user_y
    - three instructions
*/
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"lp",config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        has_one = mint_x, // here has_one puts the check that this mint_x is the same one as mentioned in the config account struct
        has_one = mint_y,
        seeds =[b"config",config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>, //ata for mint_x
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info, TokenAccount>, //ata for mint_y
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Account<'info, TokenAccount>, //ata for mint_x for user

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Account<'info, TokenAccount>, //ata for mint_y for user

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    
    pub fn swap(&mut self, amount: u64, is_x:bool , min:u64) -> Result<()>{
        // here min is the minimum amount of tokens the user expects in return, this helps us to prevent user from taking losses due to slippage
        
        require!(self.config.locked==false,AmmError::PoolLocked);
        require!(amount>0, AmmError::InvalidAmount);

        // This creates a constant product curve (x × y = k)
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            self.config.fee,
            None
        ).map_err(|_| AmmError::CurveError)?;

        // Determines which token is being sold (X or Y)
        let pair = match is_x { 
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        // Calculates the swap using the constant product formula, min parameter provides slippage protection
        let res = curve.swap(pair, amount, min).map_err(|_| AmmError::SwapError)?;

        require!(res.deposit != 0 && res.withdraw != 0, AmmError::InvalidAmount);

        // Transfers tokens from user to vault (what they're selling)
        self.deposit_tokens_being_sold(is_x, res.deposit)?;
        // Transfers tokens from vault to user (what they're buying)
        self.withdraw_tokens_being_bought(is_x, res.withdraw)?;

        Ok(())
    }

    pub fn deposit_tokens_being_sold(&mut self, is_x:bool, amount: u64)->Result<()>{

        let (from,to) = match is_x{
            true => (self.user_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer{
            from,
            to,
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn withdraw_tokens_being_bought(&mut self, is_x: bool, amount: u64) -> Result<()> {
        
        // If is_x is true (user sold X), they now buy/withdraw Y.
        // If is_x is false (user sold Y), they now buy/withdraw X.
        let (from, to) = match is_x {
            true => (self.vault_y.to_account_info(), self.user_y.to_account_info()),
            false => (self.vault_x.to_account_info(), self.user_x.to_account_info()),
        };
    
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.config.to_account_info(),
        };
    
        let seeds = &[&b"config"[..], &self.config.seed.to_le_bytes(), &[self.config.config_bump]];
        let signer_seeds = &[&seeds[..]];
    
        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    
        transfer(ctx, amount)?;
        Ok(())
    }
}
