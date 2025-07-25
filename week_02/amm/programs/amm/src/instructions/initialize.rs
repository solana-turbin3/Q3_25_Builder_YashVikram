use anchor_lang::{prelude::*, solana_program::address_lookup_table::state::ProgramState};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use crate::state::Config;

// this instruction is for the initializer (whoever starts the amm pool and sets the rule)
/*
    Account context data structure:
    - initializer
    - mint_x
    - mint_y
    - mint_lp
    - config
    - vault_x
    - vault_y
    - the three accounts
*/

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info>{

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub mint_x: Account<'info,Mint>, // we are just reading tokens, not initializing or updating them, so we don't need to mention #[account()] constraint here
    pub mint_y: Account<'info,Mint>,

    #[account(
        init,
        payer = initializer,
        seeds = [b"lp",config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp: Account<'info,Mint>, // lp tokens to be given to the users

    #[account(
        init,
        payer = initializer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump
    )]
    pub config: Account<'info,Config>, // unique config account which controls each unique amm pool

    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = config
    )]
    pub vault_x: Account<'info,TokenAccount>, // associated token account to store mint_x, notice, we do not need to provide seeds when we initialize atas

     #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = config
    )]
    pub vault_y: Account<'info,TokenAccount>,
    
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
    pub associated_token_program: Program<'info,AssociatedToken>,
}

impl <'info> Initialize<'info> {

    pub fn init(&mut self, seed:u64,authority: Option<Pubkey>, fee:u16, bumps: &InitializeBumps ) -> Result<()>{
        
        self.config.set_inner(Config { 
            seed, 
            authority, 
            mint_x: self.mint_x.key(), 
            mint_y: self.mint_y.key(), 
            fee, 
            locked: false, 
            config_bump: bumps.config, 
            lp_bump: bumps.mint_lp 
        });

        Ok(())
    }
}