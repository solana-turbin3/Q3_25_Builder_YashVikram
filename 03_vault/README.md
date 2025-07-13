# Solana Vault Program

A secure vault system built on Solana using the Anchor framework. This program allows users to create, deposit, withdraw, and close vault accounts with proper access control and rent management.

## Overview

The vault program provides a secure way to store SOL tokens in Program Derived Addresses (PDAs) with the following features:
- **Secure storage**: Tokens are stored in PDAs that only the program can control
- **Access control**: Only the vault owner can withdraw or close the vault
- **Rent management**: Automatically maintains rent-exempt status
- **Atomic operations**: All vault operations are atomic transactions

## Program Architecture

```
03_vault/
├── programs/
│   └── vault/
│       ├── src/
│       │   └── lib.rs              # Main program with all instructions
│       └── Cargo.toml              # Program dependencies
├── tests/
│   └── vault.ts                    # Integration tests
├── migrations/
│   └── deploy.ts                   # Deployment script
├── Anchor.toml                     # Anchor configuration
└── package.json                    # Project dependencies
```

## Program Instructions

### 1. Initialize Instruction
Creates a new vault and vault state account for the signer.

**Parameters:** None

**Accounts:**
- `signer`: The vault owner (signer)
- `vault`: PDA vault account that holds SOL
- `vault_state`: PDA state account that stores vault metadata
- `system_program`: System program for account creation and transfers

**What it does:**
- Creates a vault state account with bump seeds
- Creates a vault account (PDA) that can hold SOL
- Funds the vault with minimum rent-exempt balance

### 2. Deposit Instruction
Deposits SOL from the signer into the vault.

**Parameters:**
- `amount`: Amount of SOL to deposit (in lamports)

**Accounts:**
- `signer`: The vault owner (signer)
- `vault`: PDA vault account (receives SOL)
- `vault_state`: PDA state account
- `system_program`: System program for transfers

### 3. Withdraw Instruction
Withdraws SOL from the vault to the signer.

**Parameters:**
- `amount`: Amount of SOL to withdraw (in lamports)

**Accounts:**
- `signer`: The vault owner (signer)
- `vault`: PDA vault account (provides SOL)
- `vault_state`: PDA state account
- `system_program`: System program for transfers

**Security checks:**
- Ensures vault has sufficient balance after maintaining rent exemption
- Only the vault owner can withdraw

### 4. Close Instruction
Closes the vault and returns all SOL to the owner.

**Parameters:** None

**Accounts:**
- `signer`: The vault owner (signer)
- `vault`: PDA vault account (closed)
- `vault_state`: PDA state account (closed)
- `system_program`: System program for transfers

**What it does:**
- Transfers all SOL from vault to the owner
- Closes both vault and vault_state accounts
- Returns rent to the owner

## State Structure

### VaultState Account
```rust
pub struct VaultState {
    pub vault_bump: u8,    // Bump seed for the vault PDA
    pub state_bump: u8,    // Bump seed for the state PDA
}
```

## PDA Derivation

The program uses two PDAs:

1. **Vault State PDA**: `["state", signer.key()]`
   - Stores vault metadata and bump seeds
   - Owned by the program

2. **Vault PDA**: `["vault", vault_state.key()]`
   - Holds the actual SOL tokens
   - Owned by the program
   - Can sign transactions using the vault_state bump

## Error Types

- `InsufficientFunds`: Vault doesn't have enough SOL for withdrawal while maintaining rent exemption

## Getting Started

### Prerequisites
- Rust and Cargo
- Node.js and Yarn
- Solana CLI tools
- Anchor CLI

### Installation

1. **Clone and install dependencies:**
   ```bash
   cd 03_vault
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

### Initializing a Vault
```typescript
const [vaultState] = PublicKey.findProgramAddressSync(
  [Buffer.from("state"), signer.publicKey.toBuffer()],
  program.programId
);

const [vault] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), vaultState.toBuffer()],
  program.programId
);

const tx = await program.methods
  .initialize()
  .accounts({
    signer: signer.publicKey,
    vault,
    vaultState,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Depositing SOL
```typescript
const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL); // 1 SOL

const tx = await program.methods
  .deposit(amount)
  .accounts({
    signer: signer.publicKey,
    vault,
    vaultState,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Withdrawing SOL
```typescript
const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 2); // 0.5 SOL

const tx = await program.methods
  .withdraw(amount)
  .accounts({
    signer: signer.publicKey,
    vault,
    vaultState,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Closing the Vault
```typescript
const tx = await program.methods
  .close()
  .accounts({
    signer: signer.publicKey,
    vault,
    vaultState,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

## Security Features

- **PDA-based storage**: SOL is stored in Program Derived Addresses
- **Access control**: Only the vault owner can withdraw or close
- **Rent management**: Automatically maintains rent-exempt status
- **Atomic operations**: All vault operations are single transactions
- **Bump seed validation**: Proper PDA derivation and validation

## Testing

The project includes comprehensive integration tests covering:
- Vault initialization
- SOL deposits and withdrawals
- Account closure
- Error handling

Run tests with:
```bash
anchor test
```

## Program ID

- **Localnet**: `EodSMyDA8eeKfUaVqNy6y8F7wF2JB2fgHjofphnff1Gy`

## Dependencies

- **Anchor**: `^0.31.1`
- **TypeScript**: `^5.7.3`

## License

ISC

## Resources

- [Anchor Documentation](https://book.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Program Derived Addresses](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses) 