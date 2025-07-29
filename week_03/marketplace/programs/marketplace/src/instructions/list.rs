use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{mpl_token_metadata, MasterEditionAccount, MetadataAccount},
    // Import from token_interface for full compatibility
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked},
};
use mpl_token_metadata::accounts::Metadata; // Import the Metadata struct for the program type

use crate::{
    error::MarketplaceError, marketplace_account, state::{ListingAccount, MarketplaceAccount}
};

#[derive(Accounts)]
pub struct List<'info> {

    #[account(mut)]
    pub seller: Signer<'info>,

    // The seller's token account containing the NFT
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
        associated_token::token_program = token_program,
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>,

    // The NFT mint account to be listed
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = seller,
        space = 8 + ListingAccount::INIT_SPACE,
        seeds = [
            b"listing",
            marketplace_account.key().as_ref(),
            seller.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
    )]
    pub listing: Account<'info, ListingAccount>,

    #[account(
        init,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program,
    )]
    pub listing_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace".as_ref()],
        bump = marketplace_account.marketplace_bump,
    )]
    pub marketplace_account: Account<'info, MarketplaceAccount>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            mpl_token_metadata::ID.as_ref(), // Use the static program ID
            mint.key().as_ref(),
        ],
        seeds::program = mpl_token_metadata::ID,
        bump,
        // Combined constraints for clarity
        constraint = metadata.collection.is_some() 
            && metadata.collection.as_ref().unwrap().verified 
            && metadata.collection.as_ref().unwrap().key == collection_mint.key(),
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata", 
            mpl_token_metadata::ID.as_ref(), 
            mint.key().as_ref(), 
            b"edition"
        ],
        seeds::program = mpl_token_metadata::ID,
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub clock: Sysvar<'info, Clock>
}

impl <'info> List <'info>{

    pub fn initialize_listing(&mut self, price: u64, bumps: &ListBumps)->Result<()>{
        
        require!(price > 0, MarketplaceError::InvalidPrice);
        require!(self.seller_ata.amount > 0, MarketplaceError::NoToken);

        self.listing.set_inner(ListingAccount { 
            seller: self.seller.key(), 
            mint: self.mint.key(), 
            price, 
            created_at: self.clock.unix_timestamp, 
            listing_bump: bumps.listing  
        });

        Ok(())
    }

    pub fn transfer_nft(&mut self)->Result<()>{

        let program = self.token_program.to_account_info();
        let accounts = TransferChecked{
            from: self.seller_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.listing_ata.to_account_info(),
            authority: self.seller.to_account_info(),
        };

        let ctx = CpiContext::new(program, accounts);

        transfer_checked(ctx, 1,self.mint.decimals)?;

        Ok(())
    }
}