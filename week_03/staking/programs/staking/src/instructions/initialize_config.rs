use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::StakeConfig;

/*
    accounts:
        - admin (initializes the config)
        - config (StakeConfig account)
        - rewards_mint (Mint for reward of staking)
        - system program & token program
*/

// we initialize these accounts here, and we may not use it here, but these are initialized over here. and also, these are initialised only once
#[derive(Accounts)]
pub struct InitializeConfig<'info> {

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + StakeConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info,StakeConfig>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"rewards",config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config, // this means that config is the "money printing machine", that is, only it can create reward tokens
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub system_program : Program<'info, System>,
    pub token_program: Program<'info,Token>,
}

impl <'info> InitializeConfig<'info> {
    pub fn initialize_config(&mut self,points_per_stake: u8, max_stake:u8, freeze_period: u32, bumps:&InitializeConfigBumps)->Result<()>{
        self.config.set_inner(StakeConfig{
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });

        Ok(())
    }
}
