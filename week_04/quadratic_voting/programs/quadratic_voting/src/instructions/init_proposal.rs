use anchor_lang::prelude::*;

use crate::{dao, proposal, Dao, Proposal};

#[derive(Accounts)]
pub struct InitProposal <'info> {

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub dao: Account<'info, Dao>,

    #[account(
        init,
        payer = creator,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", dao.key().as_ref(), dao.proposal_count.to_le_bytes().as_ref() ],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}

impl <'info> InitProposal <'info> {
    
    pub fn init_proposal(&mut self, metadata: String, bumps: &InitProposalBumps)-> Result<()>{

        let proposal = &self.proposal;

        self.dao.proposal_count += 1;

        self.proposal.set_inner(Proposal { 
            authority: self.creator.key(), 
            metadata, 
            yes_vote_count: 0, 
            no_vote_count: 0, 
            bump: bumps.proposal
        });

        Ok(())
    }
}
