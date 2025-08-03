use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Dao {
    #[max_len(500)]
    pub name: String,
    pub authority: Pubkey,
    pub proposal_count: u64,
    pub bump: u8,
}

