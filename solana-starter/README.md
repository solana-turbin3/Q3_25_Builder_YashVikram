# Solana Starter

This repository provides a comprehensive starter template for Solana development in both **TypeScript** and **Rust**. It includes scripts and programs for wallet management, airdrops, transfers, SPL tokens, NFTs, and on-chain program interaction, making it ideal for learning, prototyping, or bootstrapping Solana projects.

---

## Folder Structure

```
solana-starter/
  ├── ts/                # TypeScript client scripts and tools
  │   ├── prereqs/       # Prerequisite scripts (wallet, airdrop, transfer, enroll)
  │   ├── programs/      # Anchor IDL for on-chain programs
  │   ├── tools/         # Utility scripts (base58, wallet conversion, airdrop)
  │   ├── cluster1/      # SPL token, NFT, and vault scripts
  │   ├── wba-wallet.json# Example wallet file
  │   ├── package.json   # NPM dependencies and scripts
  │   └── ...            # Config and lock files
  ├── rs/                # Rust client and program code
  │   ├── src/           # Rust source files
  │   ├── Cargo.toml     # Rust dependencies and metadata
  │   └── ...            # Build and config files
  └── ...
```

---

## TypeScript Client

### Overview

The `ts/` directory contains scripts for:

- **Wallet management**: Generate, import, and export Solana wallets.
- **Airdrop and transfer**: Request SOL from the Devnet faucet and transfer SOL.
- **Program interaction**: Enroll and update accounts on the WBA Prereq program.
- **SPL tokens & NFTs**: Mint, transfer, and manage SPL tokens and NFTs.
- **Vault operations**: Deposit, withdraw, and manage assets in a vault program.

### Key Scripts

#### Prerequisite Scripts (`ts/prereqs/`)

- `keygen.ts`: Generate a new Solana wallet and print the public/secret key.
- `airdrop.ts`: Request 2 SOL from the Devnet faucet to your wallet.
- `transfer.ts`: Transfer your wallet's entire balance (minus fees) to a specified address.
- `enroll.ts`: Enroll your wallet in the WBA Prereq program (on-chain).

#### Tools (`ts/tools/`)

- `base58_to_wallet.ts`: Convert a base58 private key to a wallet array.
- `wallet_to_base58.ts`: Convert a wallet array to a base58 private key.
- `airdrop_to_wallet.ts`: Request an airdrop to a specified wallet.

#### SPL, NFT, and Vault Scripts (`ts/cluster1/`)

- `spl_init.ts`, `spl_mint.ts`, `spl_transfer.ts`, `spl_metadata.ts`: SPL token operations.
- `nft_image.ts`, `nft_metadata.ts`, `nft_mint.ts`: NFT minting and metadata.
- `vault_init.ts`, `vault_deposit.ts`, `vault_withdraw.ts`, `vault_close.ts`, etc.: Vault program operations for SOL, SPL, and NFTs.

#### Program IDL (`ts/programs/`)

- `wba_prereq.ts`: Anchor IDL for the WBA Prereq program, which supports `complete` and `update` instructions for on-chain enrollment.

### Usage

1. **Install dependencies**:
   ```sh
   cd solana-starter/ts
   yarn install
   # or
   npm install
   ```

2. **Configure your wallet**: Place your keypair as `wba-wallet.json` or `dev-wallet.json` in the `ts/` directory.

3. **Run scripts**:
   ```sh
   yarn keygen
   yarn airdrop
   yarn transfer
   yarn enroll
   # Or use ts-node directly:
   npx ts-node prereqs/airdrop.ts
   ```

4. **SPL, NFT, and Vault operations**:
   ```sh
   yarn spl_init
   yarn spl_mint
   yarn nft_mint
   yarn vault_init
   # ...and more (see package.json scripts)
   ```

---

## Rust Client

### Overview

The `rs/` directory contains Rust code for:

- **Wallet management**: Key generation, base58 conversion.
- **Airdrop and transfer**: Request SOL and transfer funds.
- **Program interaction**: Enroll and update accounts on the WBA Prereq program.

### Key Files

- `src/prereqs.rs`: Contains test functions for wallet, airdrop, transfer, and program interaction.
- `Cargo.toml`: Lists dependencies (`solana-sdk`, `solana-client`, `bs58`, etc.).

### Usage

1. **Install Rust and dependencies** (see [rustup.rs](https://rustup.rs/)).
2. **Configure your wallet**: Place your keypair as `dev-wallet.json` or `wba-wallet.json` in the `rs/` directory.
3. **Run tests**:
   ```sh
   cargo test -- --nocapture
   ```
   This will execute all the test functions, printing output for each operation.

---

## On-Chain Program: WBA Prereq

- The WBA Prereq program allows users to enroll and update their on-chain account with a GitHub username.
- The Anchor IDL is provided in `ts/programs/wba_prereq.ts`.
- The program expects a PDA derived from `["prereq", user_pubkey]`.

---

## Assignment Goals

- **Learn Solana wallet management**: Key generation, import/export, and base58 encoding.
- **Interact with the Solana Devnet**: Request airdrops, check balances, and transfer SOL.
- **Work with Solana programs**: Enroll and update accounts using both Rust and TypeScript clients.
- **Explore SPL tokens, NFTs, and vaults**: Mint, transfer, and manage assets on Solana.

---

## References

- [Solana Docs](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Framework](https://book.anchor-lang.com/)
- [Web3.js](https://solana-labs.github.io/solana-web3.js/)

---

## License

MIT 