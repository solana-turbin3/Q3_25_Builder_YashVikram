# Solana AMM (Automated Market Maker)

This project implements an Automated Market Maker (AMM) smart contract for the Solana blockchain using the [Anchor](https://project-serum.github.io/anchor/) framework.

## Features
- **Pool Initialization**: Create a new AMM pool with custom parameters.
- **Liquidity Provision**: Deposit tokens to provide liquidity and receive LP tokens.
- **Token Swaps**: Swap between two tokens using a constant product curve.
- **Withdrawals**: Remove liquidity and redeem LP tokens for underlying assets.
- **Pool Management**: Lock/unlock pools for admin control.

## Directory Structure
```
05_amm/
├── Anchor.toml           # Anchor configuration
├── Cargo.toml            # Rust dependencies
├── programs/
│   └── amm/
│       ├── src/
│       │   ├── constants.rs
│       │   ├── error.rs
│       │   ├── instructions/
│       │   │   ├── deposit.rs
│       │   │   ├── initialize.rs
│       │   │   ├── swap.rs
│       │   │   ├── update.rs
│       │   │   ├── withdraw.rs
│       │   │   └── mod.rs
│       │   ├── lib.rs
│       │   └── state/
│       │       └── mod.rs
│       └── Cargo.toml
├── tests/
│   └── amm.ts            # Anchor Mocha tests
├── migrations/
│   └── deploy.ts         # Anchor deployment script
└── README.md
```

## Build & Test

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://project-serum.github.io/anchor/getting-started/installation.html)
- Node.js & Yarn

### Build the Program
```sh
anchor build
```

### Run Tests
```sh
anchor test
```

## Usage Example

You can interact with the AMM program using Anchor's TypeScript client. Example (see `tests/amm.ts`):

```typescript
const tx = await program.methods
  .initialize(seed, fee, authority)
  .accounts({ /* ... */ })
  .rpc();
```

## License
MIT 