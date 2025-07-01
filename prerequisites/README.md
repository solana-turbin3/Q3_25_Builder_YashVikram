 # Prerequisites Assignment for Solana Turbine

This folder contains the prerequisite assignments for the Solana Turbine program, implemented in both **Rust** and **TypeScript**. The assignments cover essential Solana wallet operations, airdrops, transfers, and program interactions, and are designed to familiarize you with Solana development workflows.

## Folder Structure

```
prerequisites/
  ├── rust/
  │   ├── Cargo.toml
  │   ├── Cargo.lock
  │   ├── Rust_Prerequisites_Tutorial.pdf
  │   ├── dev_wallet.json
  │   ├── Turbin3-wallet.json
  │   └── src/
  │       └── lib.rs
  └── typescript/
      ├── airdrop.ts
      ├── base58ToWallet.ts
      ├── emptyWallet.ts
      ├── enroll.ts
      ├── enroll2.ts
      ├── findPda.ts
      ├── keygen.ts
      ├── transfer.ts
      ├── dev-wallet.json
      ├── Turbin3-wallet.json
      ├── package.json
      ├── package-lock.json
      ├── tsconfig.json
      ├── yarn.lock
      ├── Prerequisites_Tutorial.docx.pdf
      └── programs/
          └── Turbin3_prereq.ts
```

---

## Rust Prerequisites

### Overview

The Rust project (`rust/`) contains a set of test functions in `src/lib.rs` that demonstrate key Solana wallet and transaction operations using the Solana Rust SDK.

### Key Files

- **Cargo.toml**: Project manifest, lists dependencies like `solana-sdk`, `solana-client`, `bs58`.
- **dev_wallet.json** / **Turbin3-wallet.json**: Example keypair files for wallet operations.
- **Rust_Prerequisites_Tutorial.pdf**: Assignment instructions (binary, not viewable here).
- **src/lib.rs**: Main logic, with the following test functions:
  - `keygen`: Generates a new Solana wallet and prints the public key and secret key array.
  - `print_pubkey`: Reads a keypair from file and prints the public key.
  - `base58_to_wallet`: Converts a base58-encoded private key to a wallet array.
  - `wallet_to_base58`: Converts a wallet array to a base58-encoded private key.
  - `airdrop`: Requests an airdrop of SOL to the dev wallet.
  - `check_balance`: Prints the wallet's SOL balance.
  - `transfer_sol`: Signs and sends a transfer transaction, and verifies the signature.
  - `empty_devnet_wallet`: Transfers the entire wallet balance (minus fees) to a specified address.
  - `submit_turbin3`: (Partial) Submits a transaction to the Turbin3 program.

### Usage

1. **Install Rust and dependencies** (see [rustup.rs](https://rustup.rs/)).
2. **Configure your wallet**: Place your keypair as `dev_wallet.json` in the project root.
3. **Run tests**:
   ```sh
   cargo test -- --nocapture
   ```
   This will execute all the test functions, printing output for each operation.

---

## TypeScript Prerequisites

### Overview

The TypeScript project (`typescript/`) provides scripts for similar wallet and transaction operations, using the Solana Web3.js and Anchor frameworks.

### Key Files

- **package.json**: Lists dependencies (`@solana/web3.js`, `@coral-xyz/anchor`, `bs58`, etc.) and scripts.
- **dev-wallet.json** / **Turbin3-wallet.json**: Example keypair files.
- **Prerquisites_Tutorial.docx.pdf**: Assignment instructions (binary, not viewable here).
- **programs/Turbin3_prereq.ts**: Anchor IDL for the Turbin3 prerequisite program.

#### Scripts

- **keygen.ts**: Generates a new Solana wallet and prints the public key and secret key array.
- **airdrop.ts**: Requests an airdrop of 2 SOL to the dev wallet.
- **transfer.ts**: Transfers 0.01 SOL from the dev wallet to a specified address.
- **emptyWallet.ts**: Transfers the entire balance (minus fees) from the dev wallet to a specified address.
- **base58ToWallet.ts**: Converts a base58-encoded private key to a wallet array.
- **findPda.ts**: Computes the Program Derived Address (PDA) for the Turbin3 program and a given wallet.
- **enroll.ts**: Calls the `initialize` method on the Turbin3 program to enroll a user (with a GitHub username).
- **enroll2.ts**: Calls the `submitTs` method on the Turbin3 program to submit the TypeScript prerequisite.

### Usage

1. **Install dependencies**:
   ```sh
   cd prerequisites/typescript
   yarn install
   # or
   npm install
   ```
2. **Configure your wallet**: Place your keypair as `dev-wallet.json` or `Turbin3-wallet.json`.
3. **Run scripts**:
   ```sh
   yarn keygen
   yarn airdrop
   yarn transfer
   yarn enroll
   yarn enroll2
   # Or use ts-node directly:
   npx ts-node airdrop.ts
   ```

### Notes

- All scripts use the Solana Devnet.
- Update the public keys in scripts as needed for your assignment.
- The Anchor program IDL (`programs/Turbin3_prereq.ts`) describes the on-chain program's instructions and account structure.

---

## Assignment Goals

- **Understand Solana wallet management**: Key generation, import/export, and base58 encoding.
- **Interact with the Solana Devnet**: Request airdrops, check balances, and transfer SOL.
- **Work with Solana programs**: Enroll and submit assignments to the Turbin3 program using both Rust and TypeScript clients.
- **Familiarize with Anchor and Web3.js**: Learn how to use these frameworks for Solana development.

---

## References

- [Solana Docs](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Framework](https://book.anchor-lang.com/)
- [Web3.js](https://solana-labs.github.io/solana-web3.js/)

---

## License

MIT

---

Let me know if you want to add more details, usage examples, or specific instructions for any script!
