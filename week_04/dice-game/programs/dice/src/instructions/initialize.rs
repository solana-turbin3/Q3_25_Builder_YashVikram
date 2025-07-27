#![allow(unexpected_cfgs)]
use anchor_lang::{
    prelude::*, system_program::{transfer,Transfer},
};

// This code defines the initialize instruction for your game. Its one and only job is to set up the game's treasury.

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub house: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault",house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>, //This is the game's treasury. It's a special account that will hold all the SOL for paying out winning bets
 
    pub system_program: Program<'info,System>,
}

impl <'info> Initialize <'info> {
    // Its job is to move SOL from the house to the vault. It does this using a Cross-Program Invocation (CPI).
    pub fn init(&mut self,amount: u64) -> Result<()>{

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.house.to_account_info(),
            to: self.vault.to_account_info(),
        };
  
        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(ctx, amount)
    }
}

