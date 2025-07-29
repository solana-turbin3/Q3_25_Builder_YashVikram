#![allow(unexpected_cfgs)]
use anchor_lang::{ prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, close_account, Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount}};

use crate::{ListingAccount, MarketplaceAccount};

#[derive(Accounts)]
pub struct Delist<'info>{

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = seller,
        associated_token::token_program = token_program,
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = seller,
        has_one = mint,
        has_one = seller,
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
        mut,
        // close = seller, 
        associated_token::authority = listing,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
    )]
    pub listing_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace".as_ref()],
        bump = marketplace_account.marketplace_bump
    )]
    pub marketplace_account: Account<'info, MarketplaceAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info,TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl <'info> Delist <'info> {
    
    pub fn return_nft(&mut self)->Result<()>{
        
        let program = self.token_program.to_account_info();
        let accounts = TransferChecked{
            from: self.listing_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.seller_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let marketplace_binding = self.marketplace_account.key();
        let seller_binding =self.seller.key();
        let mint_binding = self.mint.key();

        let seeds = &[
            b"listing",
            marketplace_binding.as_ref(),
            seller_binding.as_ref(),
            mint_binding.as_ref(),
            &[self.listing.listing_bump]
        ];

        let signer_seeds = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);
        transfer_checked(ctx, 1, self.mint.decimals)?;

        // Step:2 Close the listing_ata manually
        let close_cpi_accounts = CloseAccount {
            account: self.listing_ata.to_account_info(),
            destination: self.seller.to_account_info(), // Send rent to seller
            authority: self.listing.to_account_info(), // The PDA is the authority
        };
        let close_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_cpi_accounts,
            signer_seeds // The same seeds must sign to close the account
        );
        close_account(close_cpi_ctx)?;

        Ok(())
        
    }
}

