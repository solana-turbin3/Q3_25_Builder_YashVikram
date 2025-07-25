use anchor_lang::prelude::*;

// the escrow pda actually itself becomes the vault, do we don't have to use a vault explicitly here
#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}

/*
    - this escrow will take the mint_a from maker and store them.
    - when the mint_b is transferred from the taker_ata_b to maker_ata_b, alongside mint_a will be          transferred directly from vault to taker_ata_a
    - this is why we do not need taker account here, because tldr:
        it takes tokens from maker, stores them, and transfers to taker. does not take tokens from taker, hence, do need for it

    - we need a seed because we want the maker to be able to have multiple escrow accounts so that he can transfer tokens whenever and however he wants
*/