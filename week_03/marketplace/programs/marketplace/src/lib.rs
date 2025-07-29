pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ALAywgGpzDGo25uYh53aUvMmyu5PqEgWahTBw3xukUUY");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<Initialize>, fee_basis_points: u16, treasury: Pubkey) -> Result<()>{
        ctx.accounts.initialize_marketplace(fee_basis_points, treasury, &ctx.bumps)?;
        Ok(())
    }
    
    pub fn list_nft(ctx:Context<List>,price: u64) -> Result<()>{
        ctx.accounts.initialize_listing(price, &ctx.bumps)?;
        ctx.accounts.transfer_nft()?;
        Ok(())
    }

    pub fn delist_nft(ctx:Context<Delist>)-> Result<()>{
        ctx.accounts.return_nft()?;
        Ok(())
    }

    pub fn purchase_nft(ctx:Context<Purchase>) -> Result<()>{
        ctx.accounts.process_payment()?;
        ctx.accounts.transfer_nft()?;
        Ok(())
    }
}
