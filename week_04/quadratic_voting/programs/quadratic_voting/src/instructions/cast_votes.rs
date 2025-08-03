use anchor_lang::{prelude::*};
use anchor_spl::token::TokenAccount;

use crate::{Dao, Proposal, Vote};

#[derive(Accounts)]
pub struct CastVotes <'info> {

    #[account(mut)]
    pub voter: Signer<'info>,

    pub dao: Account<'info, Dao>,
    pub proposal: Account<'info, Proposal>,

    #[account(
        init,
        payer = voter,
        space = 8 + Vote::INIT_SPACE,
        seeds = [b"vote", voter.key().as_ref(), proposal.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,

    #[account(
        token::authority = voter,
    )]
    pub creator_ata : Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

impl <'info> CastVotes <'info> {
    
    pub fn cast_votes(&mut self, vote_type: u8, bumps: &CastVotesBumps)-> Result<()>{

        let vote_account = &self.vote;
        let proposal_account = &self.proposal;

        let voting_credit = (self.creator_ata.amount as f64).sqrt() as u64;

        self.vote.set_inner(Vote { 
            authority: self.voter.key(), 
            vote_type, 
            vote_credits: voting_credit, 
            bump: bumps.vote  
        });
        
        Ok(())
    }
}
