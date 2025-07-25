use anchor_lang::prelude::*;

use crate::UserAccount;
/*
    we are initializing the user accounts here:
     - user: Signer<'info>
     - user_account: UserAccount of the user
     - system_program
*/
#[derive(Accounts)]
pub struct InitializeUser<'info>{

    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()], // we want one to many behaviour, i.e. one user account is related to multiple stake accounts
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info>{

    pub fn initialize_user(&mut self, bumps: &InitializeUserBumps) ->Result<()>{
        self.user_account.set_inner(UserAccount{
            points : 0,
            amount_staked : 0,
            bump : bumps.user_account,
        });

        Ok(())
    }
}