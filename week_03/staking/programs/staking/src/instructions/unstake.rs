use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi,
            ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount,
        Metadata,
        MetadataAccount,
    },
    token::{ Revoke, revoke, Mint, Token, TokenAccount },
};

use crate::{ state::{ StakeAccount, StakeConfig }, UserAccount };
use crate::{ error::StakeError };
/*
   - this instruction will allow the user to stake nfts.
   - accounts:
        - user
        - mint
        - collection mint
        - metadata
        - edition
        - config
        - user_account
        - stake_account
        - user_mint_ata
        - the three programs
*/

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() ==
        collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [b"user",user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump = stake_account.bump,
        close = user
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/*
    Steps:
    (1) calculate the date/ time and check if freeze_period has passed (even though the user can unstake their nft anytime, they cannot do it before the freezing period is over)
    (2) thaw (unfreeze) the frozen NFT
    (3) remove delegate approval so user gets full control
    (4) update the amount of tokens staked by the user (decrease it)
*/
impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        // ensure freeze period has passed
        let now = Clock::get()?.unix_timestamp;

        let staked_at = self.stake_account.staked_at as i64;
        let days = (now - staked_at) / 86_400;
        require!(days >= (self.config.freeze_period as i64), StakeError::FreezePeriodNotPassed);

        // thaw the frozen NFT via Metadata CPI
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.user_mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = self.mint.to_account_info();
        let token_program = self.token_program.to_account_info();
        let metadata_program = self.metadata_program.to_account_info();

        let mint_key = self.mint.key();
        let config_key = self.config.key();

        let seeds = &[b"stake", mint_key.as_ref(), config_key.as_ref(), &[self.stake_account.bump]];
        let signer_seeds = &[&seeds[..]];

        ThawDelegatedAccountCpi::new(&metadata_program, ThawDelegatedAccountCpiAccounts {
            delegate: delegate,
            token_account: token_account,
            edition: edition,
            mint: &mint,
            token_program: &token_program,
        }).invoke_signed(signer_seeds)?;

        // Revoke delegate approval so user regains full control
        let cpi_accounts = Revoke {
            source: self.user_mint_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        revoke(cpi_ctx)?;

        // update on-chain state
        self.user_account.amount_staked = self.user_account.amount_staked.saturating_sub(1);

        Ok(())
    }
}