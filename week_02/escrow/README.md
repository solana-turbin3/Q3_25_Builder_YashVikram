# Solana Escrow Program

A decentralized escrow system built on Solana using the Anchor framework. This program allows users to create, execute, and refund token-based escrow transactions in a trustless manner.

## Overview

The escrow program enables atomic token swaps between two parties:
- **Maker**: Creates an escrow by depositing tokens (mint_a) and specifying desired tokens (mint_b)
- **Taker**: Executes the escrow by providing the required tokens (mint_b) to receive the deposited tokens (mint_a)
- **Refund**: Allows the maker to reclaim their deposited tokens if the escrow is not executed

## Program Architecture

```
04_escrow/
├── programs/
│   └── escrow/
│       ├── src/
│       │   ├── lib.rs              # Main program entry point
│       │   ├── instructions/       # Program instructions
│       │   │   ├── make.rs         # Create escrow and deposit tokens
│       │   │   ├── transfer.rs     # Execute escrow transaction
│       │   │   ├── refund.rs       # Refund deposited tokens
│       │   │   └── mod.rs          # Instruction module
│       │   ├── state/
│       │   │   └── mod.rs          # Account state structures
│       │   ├── error.rs            # Custom error types
│       │   └── constants.rs        # Program constants
│       └── Cargo.toml              # Program dependencies
├── tests/
│   └── escrow.ts                   # Integration tests
├── migrations/
│   └── deploy.ts                   # Deployment script
├── Anchor.toml                     # Anchor configuration
└── package.json                    # Project dependencies
```

## Program Instructions

### 1. Make Instruction
Creates a new escrow and deposits tokens from the maker.

**Parameters:**
- `seed`: Unique identifier for the escrow
- `deposit_amount`: Amount of mint_a tokens to deposit
- `receive`: Amount of mint_b tokens expected in return

**Accounts:**
- `maker`: The escrow creator (signer)
- `mint_a`: Token mint being deposited
- `mint_b`: Token mint expected in return
- `maker_ata_a`: Maker's associated token account for mint_a
- `escrow`: PDA escrow account
- `escrow_vault`: Token account owned by escrow PDA
- System programs (System, Token, Associated Token)

### 2. Transfer Instruction
Executes the escrow by transferring tokens between parties.

**Parameters:**
- `seed`: Escrow identifier

**Accounts:**
- `maker`: Escrow creator (non-signer)
- `taker`: Escrow executor (signer)
- `mint_a`, `mint_b`: Token mints
- `maker_ata_b`: Maker's account for mint_b (receives tokens)
- `taker_ata_a`: Taker's account for mint_a (receives tokens)
- `taker_ata_b`: Taker's account for mint_b (provides tokens)
- `escrow`: Escrow account (closed after execution)
- `vault`: Escrow's token vault (closed after execution)
- System programs

### 3. Refund Instruction
Allows the maker to reclaim deposited tokens.

**Parameters:**
- `seed`: Escrow identifier

**Accounts:**
- `maker`: Escrow creator (signer)
- `mint_a`: Token mint being refunded
- `maker_ata_a`: Maker's account for mint_a (receives refund)
- `escrow`: Escrow account (closed after refund)
- `vault`: Escrow's token vault (closed after refund)
- System programs

## State Structure

### Escrow Account
```rust
pub struct Escrow {
    pub seed: u64,           // Unique escrow identifier
    pub maker: Pubkey,       // Escrow creator
    pub mint_a: Pubkey,      // Token being deposited
    pub mint_b: Pubkey,      // Token expected in return
    pub receive: u64,        // Amount of mint_b expected
    pub bump: u8,            // PDA bump seed
}
```

## Error Types

- `InvalidAmount`: Amount must be greater than zero
- `InsufficientFunds`: Taker doesn't have enough tokens
- `EmptyVault`: No tokens in escrow vault

## Getting Started

### Prerequisites
- Rust and Cargo
- Node.js and Yarn
- Solana CLI tools
- Anchor CLI

### Installation

1. **Clone and install dependencies:**
   ```bash
   cd 04_escrow
   yarn install
   ```

2. **Build the program:**
   ```bash
   anchor build
   ```

3. **Run tests:**
   ```bash
   anchor test
   ```

### Deployment

1. **Configure your wallet in Anchor.toml**
2. **Deploy to localnet:**
   ```bash
   anchor deploy
   ```

3. **Or deploy to devnet:**
   ```bash
   anchor deploy --provider.cluster devnet
   ```

## Usage Examples

### Creating an Escrow
```typescript
const tx = await program.methods
  .make(seed, depositAmount, receiveAmount)
  .accounts({
    maker: maker.publicKey,
    mintA: mintA,
    mintB: mintB,
    makerAtaA: makerAtaA,
    escrow: escrowPda,
    escrowVault: escrowVault,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  })
  .signers([maker])
  .rpc();
```

### Executing an Escrow
```typescript
const tx = await program.methods
  .transfer(seed)
  .accounts({
    maker: maker.publicKey,
    taker: taker.publicKey,
    mintA: mintA,
    mintB: mintB,
    makerAtaB: makerAtaB,
    takerAtaA: takerAtaA,
    takerAtaB: takerAtaB,
    escrow: escrowPda,
    vault: escrowVault,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  })
  .signers([taker])
  .rpc();
```

### Refunding an Escrow
```typescript
const tx = await program.methods
  .refund(seed)
  .accounts({
    maker: maker.publicKey,
    mintA: mintA,
    makerAtaA: makerAtaA,
    escrow: escrowPda,
    vault: escrowVault,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  })
  .signers([maker])
  .rpc();
```

## Security Features

- **PDA-based vault**: Escrow tokens are stored in a Program Derived Address
- **Atomic transactions**: All token transfers happen in a single transaction
- **Access control**: Only the maker can refund, only the taker can execute
- **Validation**: Comprehensive checks for amounts, account ownership, and state

## Testing

The project includes comprehensive integration tests covering:
- Successful escrow creation and execution
- Error handling for invalid amounts
- Refund functionality
- Account validation

Run tests with:
```bash
anchor test
```

## Program ID

- **Localnet**: `FZDK4T9zznUeC3xVDq8qZqmd1WerJvYmGwWPtXBuRgv6`

## Dependencies

- **Anchor**: `^0.31.1`
- **SPL Token**: `^0.4.13`
- **TypeScript**: `^5.7.3`

## License

ISC

## Resources

- [Anchor Documentation](https://book.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Program](https://spl.solana.com/token) 