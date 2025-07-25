use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ mint_to, Mint, MintTo, Token, TokenAccount },
};

use crate::{ StakeConfig, UserAccount };

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user".as_ref(),user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [b"rewards".as_ref(),config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub rewards_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"config".as_ref()], 
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
/*
    Steps:
    (1) we will mint the rewards token to user ata
    (2) set user points to 0
*/
impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let seeds = &[b"config".as_ref(), &[self.config.bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = MintTo {
            authority: self.config.to_account_info(),
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Calculate the amount to mint, accounting for decimals
        let amount_to_mint = self.user_account.points as u64
            * 10_u64.pow(self.rewards_mint.decimals as u32);

        // CPI to mint the tokens
        mint_to(cpi_context, amount_to_mint)?;

        // CRITICAL FIX: Reset the user's points to 0
        self.user_account.points = 0;

        Ok(())
    }
}