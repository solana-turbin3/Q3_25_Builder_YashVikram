#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use crate::MarketplaceAccount;

#[derive(Accounts)]
pub struct Initialize <'info>{

    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + MarketplaceAccount::INIT_SPACE,
        seeds = [b"marketplace".as_ref()],
        bump
    )]
    pub marketplace_account: Account<'info, MarketplaceAccount>,

    // #[account(
    //     seeds = [b"treasury", marketplace_account.key().as_ref()],
    //     bump
    // )]
    // pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {

    pub fn initialize_marketplace(
        &mut self, 
        fee_basis_points:u16, 
        treasury: Pubkey,
        bumps: &InitializeBumps
        ) -> Result<()> {
        
        self.marketplace_account.set_inner(MarketplaceAccount { 
            authority: self.authority.key(), 
            fee_basis_points: fee_basis_points,  
            marketplace_bump: bumps.marketplace_account ,
            // treasury_bump: bumps.treasury,
            treasury
        });

        Ok(())
    }
}