pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("A3JHytf7fmzPu68yvhFnzaHJq1MzrYBJPMj82rvjMUXe");

#[program]
pub mod quadratic_voting {
    use super::*;

    pub fn init_dao(ctx: Context<InitDao>, name: String) -> Result<()> {
        ctx.accounts.init_dao(name, &ctx.bumps)?;
        Ok(())
    }

    pub fn init_proposal(ctx: Context<InitProposal>, metadata: String) -> Result<()> {
        ctx.accounts.init_proposal(metadata, &ctx.bumps)?;
        Ok(())
    }

    pub fn cast_votes(ctx: Context<CastVotes>, vote_type: u8) -> Result<()> {
        ctx.accounts.cast_votes(vote_type, &ctx.bumps)?;
        Ok(())
    }
}
