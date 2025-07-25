use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount{
    pub points: u32, // reward earned by the user on the initial amount staked
    pub amount_staked: u8, // initial amount staked by the user
    pub bump: u8,
}