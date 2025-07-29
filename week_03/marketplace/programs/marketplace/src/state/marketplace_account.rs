use anchor_lang::{prelude::*};

#[account]
#[derive(InitSpace)]
pub struct MarketplaceAccount{

    pub authority: Pubkey, // this is the pubkey of the authority of the marketplace
    pub fee_basis_points: u16, // the fee basis points for the royality to fee_basis_points
    pub marketplace_bump: u8 ,
   // pub treasury_bump: u8, // bump for the treasury to store the fee basis points
    pub treasury: Pubkey,
}