use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{error::DiceError, Bet};

#[derive(Accounts)]
pub struct RefundBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    ///CHECK: This is safe
    pub house: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>
}

impl<'info> RefundBet<'info> {
    pub fn refund_bet(&mut self, bumps: &RefundBetBumps) -> Result<()> {

        let slot = Clock::get()?.slot;
        
        // The require! macro ensures that the transaction will fail unless at least 1000 slots have passed. On Solana, a slot is roughly 400-600 milliseconds, so this is a timeout of about 6-10 minutes.
        require!((self.bet.slot - slot) > 1000, DiceError::TimeoutNotReached);
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info()
        };

        let seeds = [b"vault", &self.house.key().to_bytes()[..], &[bumps.vault]];
        let signer_seeds = &[&seeds[..]][..];
    

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(ctx, self.bet.amount)
    }
}