#![allow(unexpected_cfgs)]
use anchor_lang::{
    accounts::account, prelude::*, system_program::{transfer,Transfer}
};

use crate::Bet;

#[derive(Accounts)]
#[instruction(seed:u128)]
pub struct PlaceBet<'info> {

    #[account(mut)]
    pub player: Signer<'info>, // The user who is placing the bet

    /// Check: This is safe
    pub house: UncheckedAccount<'info>, // The public key of the game owner

    #[account(
        init,
        payer = player,
        space = 8 + Bet::INIT_SPACE,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info,Bet>, // This is the new account that will be created to store the bet's details

    #[account(
        mut,
        seeds = [b"vault",house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>, // The game's treasury. This instruction finds the exact same vault that was created by the initialize instruction

    pub system_program: Program<'info,System>,
}

impl <'info> PlaceBet <'info> {

    // Purpose: To fill in the details of the bet.
    pub fn create_bet(&mut self, bumps: &PlaceBetBumps, seed: u128, roll: u8, amount: u64)->Result<()>{
        self.bet.set_inner(Bet { player: self.player.key(), amount, slot: Clock::get()?.slot, seed, roll, bump : bumps.bet});
        Ok(())
    }

    // Purpose: To handle the money transfer.
    pub fn deposit(&mut self, amount: u64)->Result<()>{

        let program = self.system_program.to_account_info();
        let accounts = Transfer{
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let ctx = CpiContext::new(program, accounts);

        transfer(ctx, amount)
    }
}
