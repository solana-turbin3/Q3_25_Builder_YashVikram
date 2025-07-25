use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Freaze period not passed")]
    FreezePeriodNotPassed,
    #[msg("Max stake reached")]
    MaxStakeReached,
}