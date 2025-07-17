use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config{
    pub seed: u64, // allows us to have multiple amms pools
    pub authority: Option<Pubkey>, //authority is optional if we wanna unlock the pool, so authority is set to null
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}

/*
- seed: u64 - A unique identifier used for generating Program Derived Addresses (PDAs). This ensures each pool has a unique address and prevents address collisions when creating multiple pools.

- authority: Option<Pubkey> - The account that has administrative control over the pool. It's optional (Option) because:

When Some(pubkey), that account can modify pool parameters, pause trading, etc.
When None, the pool becomes "unlocked" or decentralized - no single entity can control it

- mint_x: Pubkey & mint_y: Pubkey - The two token mints that make up the trading pair. For example, if this is a SOL/USDC pool, one would be the SOL mint and the other the USDC mint.

- fee: u16 - The trading fee charged on swaps, typically stored in basis points (e.g., 30 = 0.3%). This generates revenue for liquidity providers and/or the protocol.

- locked: bool - A safety mechanism that can pause all trading activity. When true, swaps are disabled but liquidity operations might still work.

- config_bump: u8 & lp_bump: u8 - These store the "bump seeds" used to generate PDAs for the config account itself and the LP (liquidity provider) token mint. Storing these saves computation on subsequent operations since you don't need to derive them again.
*/