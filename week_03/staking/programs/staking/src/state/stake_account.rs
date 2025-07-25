use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount{
    pub owner: Pubkey, // public key of user wallet who staked the token
    pub mint: Pubkey, // mint address of the staked token
    pub staked_at: i64, // time at which it is staked (before 1970, unix time is negative)
    pub bump: u8,
}