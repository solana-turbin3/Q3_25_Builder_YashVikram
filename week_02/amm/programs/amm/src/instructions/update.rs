use anchor_lang::prelude::*;

use crate::{error::AmmError, Config};

// this instruction can be used to lock or unlock amm pools
/* 
    accounts required:
    - user
    - config
*/
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[b"config",&config.seed.to_le_bytes().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,
}

impl<'info> Update<'info> {
    pub fn lock(&mut self) -> Result<()> {
        require!(!self.config.locked, AmmError::PoolLocked);
        require!(self.config.authority == Some(self.user.key()), AmmError::InvalidAuthority);

        self.config.locked = true;
        Ok(())
    }

    pub fn unlock(&mut self) -> Result<()> {
        
        require!(self.config.locked, AmmError::PoolUnlocked);
        require!(self.config.authority == Some(self.user.key()), AmmError::InvalidAuthority);

        self.config.locked = false;
        Ok(())
    }
}
