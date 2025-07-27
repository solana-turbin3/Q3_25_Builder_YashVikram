use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet{
    pub player: Pubkey, // The public key (wallet address) of the person who placed the bet.
    pub amount: u64, // The amount of tokens or SOL the player has wagered.
    pub slot: u64, // The blockchain slot number when the bet was placed. This is often used to ensure a recent, unpredictable value for randomness.
    pub seed: u128, // A random seed provided by the player. This is combined with on-chain data to help generate the final roll, preventing the house from predicting the outcome.
    pub roll: u8, // The number the player is betting on 
    pub bump: u8,
}

// This function is a helper used for generating a random number. It takes all the important fields of the bet (player address, amount, slot, seed, and roll) and converts them into a sequence of bytes (Vec<u8>)
// This sequence of bytes can then be fed into a hashing algorithm (like SHA-256) to produce a "random" outcome for the dice roll in a deterministic way. This is a common pattern for on-chain randomness.
impl Bet{
    pub fn to_slice(&self) -> Vec<u8> {
        let mut s = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.amount.to_be_bytes());
        s.extend_from_slice(&self.slot.to_be_bytes());
        s.extend_from_slice(&self.seed.to_be_bytes());
        s.extend_from_slice(&self.roll.to_be_bytes());

        s
    }
}