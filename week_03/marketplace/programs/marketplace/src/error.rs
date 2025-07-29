use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("The price must be greater than 0")]
    InvalidPrice,
    #[msg("You must have at least one token.")]
    NoToken,
}
