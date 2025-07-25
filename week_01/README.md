# Solana Starter

A starter template for Solana development using **TypeScript**. This project includes scripts for working with wallets, SPL tokens, NFTs, and vaults on the Solana Devnet. It is ideal for learning, prototyping, or bootstrapping Solana projects.

---

## Folder Structure

```
solana-starter/
  ├── ts/
  │   ├── cluster1/
  │   │   ├── nft_image.ts              # Upload or manage NFT images
  │   │   ├── nft_metadata.ts           # Create or update NFT metadata
  │   │   ├── nft_mint.ts               # Mint new NFTs
  │   │   ├── spl_init.ts               # Create a new SPL token mint
  │   │   ├── spl_metadata.ts           # Manage SPL token metadata
  │   │   ├── spl_mint.ts               # Mint SPL tokens to an account
  │   │   ├── spl_transfer.ts           # Transfer SPL tokens between accounts
  │   │   ├── vault_close.ts            # Close a vault account
  │   │   ├── vault_deposit.ts          # Deposit SOL into a vault
  │   │   ├── vault_deposit_nft.ts      # Deposit NFTs into a vault
  │   │   ├── vault_deposit_spl.ts      # Deposit SPL tokens into a vault
  │   │   ├── vault_init.ts             # Initialize a new vault
  │   │   ├── vault_withdraw.ts         # Withdraw SOL from a vault
  │   │   ├── vault_withdraw_nft.ts     # Withdraw NFTs from a vault
  │   │   ├── vault_withdraw_spl.ts     # Withdraw SPL tokens from a vault
  │   ├── tools/                        # Utility scripts (not shown)
  │   ├── wba-wallet.json               # Example wallet file (keep this secret!)
  │   ├── package.json                  # NPM dependencies and scripts
  │   ├── tsconfig.json                 # TypeScript configuration
  │   └── ...                           # Other config and lock files
  ├── generug.png                       # Example image asset
  ├── README.md                         # Project documentation
  └── ...                               # Root config and lock files
```

---

## Getting Started

### 1. Install Dependencies

```sh
cd ts
npm install
# or
yarn install
```

### 2. Wallet Setup

- Place your Solana wallet keypair as `wba-wallet.json` in the `ts/` directory.
- You can generate one with:
  ```sh
  solana-keygen new -o wba-wallet.json
  ```
- Fund your wallet with Devnet SOL:
  ```sh
  solana airdrop 2 --keypair wba-wallet.json
  ```

---

## Script Usage

All scripts are run from the `ts/` directory using `ts-node`:

```sh
npx ts-node cluster1/<script_name>.ts
```

### SPL Token Scripts

- `spl_init.ts` – Create a new SPL token mint.
- `spl_mint.ts` – Mint tokens to your associated token account.
- `spl_transfer.ts` – Transfer tokens to another account.
- `spl_metadata.ts` – Manage SPL token metadata.

### NFT Scripts

- `nft_image.ts` – Upload or manage NFT images.
- `nft_metadata.ts` – Create or update NFT metadata.
- `nft_mint.ts` – Mint new NFTs.

### Vault Scripts

- `vault_init.ts` – Initialize a new vault.
- `vault_deposit.ts` – Deposit SOL into a vault.
- `vault_deposit_spl.ts` – Deposit SPL tokens into a vault.
- `vault_deposit_nft.ts` – Deposit NFTs into a vault.
- `vault_withdraw.ts` – Withdraw SOL from a vault.
- `vault_withdraw_spl.ts` – Withdraw SPL tokens from a vault.
- `vault_withdraw_nft.ts` – Withdraw NFTs from a vault.
- `vault_close.ts` – Close a vault account.

---

## Notes

- All scripts use the Solana Devnet.
- Make sure your wallet has enough SOL for transaction fees.
- Update script parameters (mint addresses, recipient addresses, etc.) as needed in each script.

---

## Resources

- [Solana Docs](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [@solana/web3.js](https://solana-labs.github.io/solana-web3.js/)
- [@solana/spl-token](https://spl.solana.com/token)

---

## License

MIT