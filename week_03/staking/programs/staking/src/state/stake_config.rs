use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig{
    pub points_per_stake:u8, // base reward rate
    pub max_stake: u8, // global limit on number of tokens a single user can stake
    pub freeze_period:u32, // mandatory lock-up period
    pub rewards_bump:u8,
    pub bump:u8
}

/*
    Stake config the global rulebook
*/