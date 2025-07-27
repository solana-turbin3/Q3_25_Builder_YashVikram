use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use solana_program::{sysvar::instructions::load_instruction_at_checked, ed25519_program, hash::hash};

use crate::{state::Bet, error::DiceError};


pub const HOUSE_EDGE: u16 = 150; // 1.5% House edge

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(
        mut
    )]
    ///CHECK: This is safe
    pub player: UncheckedAccount<'info>, //The player's account. It's the destination for any potential winnings and receives the rent back when the bet account is closed
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>, // The game's treasury, which will be the source of the payout.
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>, //  The specific Bet account being resolved. It's marked close = player so it gets deleted after this instruction, and the rent is returned to the player
    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    /// CHECK: This is safe
    pub instruction_sysvar: AccountInfo<'info>, // This is a special, read-only Solana account that contains a list of all instructions included in the current transaction. It's the key to the randomness mechanism.
    pub system_program: Program<'info, System>
}

impl<'info> ResolveBet<'info> {

    // This function checks a secret, off-chain signature provided by the house to ensure it's authentic and tied specifically to the player's bet. This is the source of randomness.
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // Get the Ed25519 signature instruction 
        let ix = load_instruction_at_checked(
            0, 
            &self.instruction_sysvar.to_account_info()
        )?;
        // Make sure the instruction is addressed to the ed25519 program
        require_keys_eq!(ix.program_id, ed25519_program::ID, DiceError::Ed25519Program);
        // Make sure there are no accounts present
        require_eq!(ix.accounts.len(), 0, DiceError::Ed25519Accounts);
        
        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, DiceError::Ed25519DataLength);
        let signature = &signatures[0];

        // Make sure all the data is present to verify the signature
        require!(signature.is_verifiable, DiceError::Ed25519Header);
        
        // Ensure public keys match
        require_keys_eq!(signature.public_key.ok_or(DiceError::Ed25519Pubkey)?, self.house.key(), DiceError::Ed25519Pubkey);

        // Ensure signatures match
        require!(&signature.signature.ok_or(DiceError::Ed25519Signature)?.eq(sig), DiceError::Ed25519Signature);

        // Ensure messages match
        require!(&signature.message.as_ref().ok_or(DiceError::Ed25519Signature)?.eq(&self.bet.to_slice()), DiceError::Ed25519Signature);

        Ok(())
    }

    // This function uses the verified signature to generate a random dice roll (1-100). If the player wins, it calculates the payout and transfers the winnings from the vault to the player.
    pub fn resolve_bet(&mut self, bumps: &ResolveBetBumps, sig: &[u8]) -> Result<()> {
        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8;16] = [0;16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);
        
        let roll = lower
            .wrapping_add(upper)
            .wrapping_rem(100) as u8 + 1;

        if self.bet.roll > roll {

            // Payout minus house edge
            let payout = (self.bet.amount as u128)
            .checked_mul(10000 - HOUSE_EDGE as u128).ok_or(DiceError::Overflow)?
            .checked_div(self.bet.roll as u128 - 1).ok_or(DiceError::Overflow)?
            .checked_div(100).ok_or(DiceError::Overflow)? as u64;

            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info()
            };

            let seeds = [b"vault", &self.house.key().to_bytes()[..], &[bumps.vault]];
            let signer_seeds = &[&seeds[..]][..];
    
            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds
            );
            transfer(ctx, payout)?;
        }
        Ok(())
    }
}