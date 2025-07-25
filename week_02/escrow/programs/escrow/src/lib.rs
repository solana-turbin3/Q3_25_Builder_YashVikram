pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("FZDK4T9zznUeC3xVDq8qZqmd1WerJvYmGwWPtXBuRgv6");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit_amount: u64,
        receive: u64,) 
        -> Result<()> {
        
        require!(deposit_amount > 0, EscrowError::InvalidAmount);
        require!(receive > 0, EscrowError::InvalidAmount);

        ctx.accounts.init_escrow_and_deposit(seed, deposit_amount, receive, &ctx.bumps)
        
    }

    pub fn refund(ctx: Context<Refund>,seed: u64)->Result<()>{
        ctx.accounts.refund_and_close_vault(seed)
        
    }

    pub fn transfer(ctx: Context<Transfer>,seed: u64)->Result<()>{
        ctx.accounts.transfer_and_close_vault(seed)   
    }
}
