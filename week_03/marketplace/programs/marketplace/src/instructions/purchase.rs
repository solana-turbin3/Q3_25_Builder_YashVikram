#![allow(unexpected_cfgs)]

use anchor_lang::{ prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, close_account, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{ListingAccount, MarketplaceAccount};

#[derive(Accounts)]
pub struct Purchase<'info>{

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we are only using this account to receive SOL, and the address is validated by being a seed in the listing PDA.
    #[account(mut)]
    pub seller: AccountInfo<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = seller,
        seeds = [
            b"listing",
            marketplace_account.key().as_ref(),
            seller.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump = listing.listing_bump
    )]
    pub listing: Account<'info, ListingAccount>,

    #[account(
        mut,
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

    /// CHECK: This is safe because we are only using it to receive SOL payments.
    #[account(
        mut,
        constraint = treasury.key() == marketplace_account.treasury
    )]
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl <'info> Purchase <'info>{

    pub fn process_payment(&mut self)->Result<()>{

        let fee_basis_points = self.marketplace_account.fee_basis_points;
        let price = self.listing.price;

        let fee = (price as u128)
                        .checked_mul(fee_basis_points as u128)
                        .unwrap()
                        .checked_div(10000)
                        .unwrap() as u64;

        let program = self.system_program.to_account_info();
        let accounts = Transfer{
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let ctx = CpiContext::new(program, accounts);
        transfer(ctx, fee)?;

        let amount = price - fee;

        let ctx_program = self.system_program.to_account_info();
        let ctx_accounts = Transfer{
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };

        let ctx = CpiContext::new(ctx_program, ctx_accounts);
        transfer(ctx, amount)?;

        Ok(())
    }

    pub fn transfer_nft(&mut self) ->Result<()>{

        let program = self.token_program.to_account_info();
        let accounts = TransferChecked{
            from: self.listing_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.buyer_ata.to_account_info(),
            authority: self.listing.to_account_info()
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

/*
SAFETY REASONS:

Even though they use the "unsafe" AccountInfo type, we have added our own powerful, programmatic checks that guarantee security.

# How the seller Account is Kept Safe:
The seller account's safety comes from the way we validate the listing PDA.
The seeds for the listing account include seller.key(). This means that for the transaction to succeed, the client must provide the public key of the original seller who created the listing. If an attacker tries to substitute a different wallet for the seller, the PDA validation on the listing account will fail, and the entire transaction will be rejected. This implicitly guarantees the right seller gets paid.

# How the treasury Account is Kept Safe:
The treasury account's safety comes from the constraint attribute.
The check constraint = treasury.key() == marketplace_account.treasury programmatically verifies that the address of the treasury account passed into the instruction is identical to the official treasury address stored in the global marketplace_account. This makes it impossible for anyone to divert fees to a different wallet.
*/