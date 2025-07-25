#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{TokenAccount, Mint, TokenInterface, transfer_checked, TransferChecked}
};

use crate::Escrow;

/*
    accounts required:
        - maker
        - mint_a
        - mint_b
        - maker_ata_a
        - escrow
        - vault
        - other three accounts
*/

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::token_program = token_program,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    // Token account owned by the escrow PDA
    // ATAs use built-in deterministic addressing, while custom PDAs need explicit seeds for derivation, which means that ATAs do not require seeds to be derived, unlike PDAs
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub escrow_vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Make<'info> {
    pub fn init_escrow_and_deposit(
        &mut self,
        seed: u64,
        deposit_amount: u64,
        receive: u64,
        bumps: &MakeBumps,
    ) -> Result<()> {
        // Initialize the escrow account
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive,
            bump: bumps.escrow,
        });

        // Transfer tokens from maker to escrow vault
        let transfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.escrow_vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let ctx = CpiContext::new(
            self.token_program.to_account_info(),
            transfer_accounts,
        );

        transfer_checked(ctx, deposit_amount, self.mint_a.decimals)?;

        Ok(())
    }
}

