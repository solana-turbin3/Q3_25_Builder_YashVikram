## 1. Core Program Structure

### Vault Program

**Responsibilities:**

- Manages user crypto deposits and withdrawals
- Implements collateral safety mechanisms
- Controls access to deposited funds
- Calculates loan-to-value ratios

**Key Instructions:**

- `initialize_vault()` - Creates user vault PDA
- `deposit_collateral()` - Accepts crypto deposits
- `withdraw_collateral()` - Processes withdrawals
- `calculate_spending_power()` - Determines available credit

### Spending Program

**Responsibilities:**

- Authorizes purchase transactions
- Validates sufficient collateral backing
- Updates user spending balances
- Maintains transaction history

**Key Instructions:**

- `authorize_purchase()` - Validates and approves spending
- `update_balance()` - Adjusts available credit
- `check_collateral_ratio()` - Ensures safe lending ratios

### Yield Program

**Responsibilities:**

- Integrates with DeFi protocols for earning
- Auto-compounds user returns
- Tracks individual user earnings
- Manages yield strategy optimization

**Key Instructions:**

- `stake_collateral()` - Deploys funds to earning protocols
- `compound_earnings()` - Reinvests generated yields
- `calculate_user_share()` - Tracks individual returns

### Liquidation Program

**Responsibilities:**

- Monitors collateral value in real-time
- Executes emergency selling when needed
- Protects platform from undercollateralization
- Notifies users of liquidation events

**Key Instructions:**

- `monitor_prices()` - Continuous price surveillance
- `trigger_liquidation()` - Executes emergency sales
- `notify_user()` - Sends liquidation alerts

### Payment Program

**Responsibilities:**

- Processes merchant settlements
- Converts crypto value to fiat
- Integrates with traditional payment rails
- Manages escrow for pending transactions

**Key Instructions:**

- `process_merchant_payment()` - Settles with merchants
- `convert_crypto_to_fiat()` - Handles currency conversion
- `update_merchant_balance()` - Tracks merchant earnings

## 2. Account Architecture

### Program Derived Addresses (PDAs)

### User Vault PDA

- **Seeds:** `[user_pubkey, "vault", bump]`
- **Data:** Collateral amounts, deposit timestamps, withdrawal permissions
- **Owner:** Vault Program

### Spending Account PDA

- **Seeds:** `[user_pubkey, "spending", bump]`
- **Data:** Available credit, transaction history, spending limits
- **Owner:** Spending Program

### Yield Account PDA

- **Seeds:** `[user_pubkey, "yield", bump]`
- **Data:** Staked amounts, earned rewards, strategy allocations
- **Owner:** Yield Program

### Merchant Account PDA

- **Seeds:** `[merchant_pubkey, "merchant", bump]`
- **Data:** Payment history, settlement preferences, fees earned
- **Owner:** Payment Program

### Token Accounts

- **User Token Account:** Holds user's original crypto before deposit
- **Vault Token Account:** Stores collateral securely within protocol
- **Yield Token Account:** Represents staked positions in DeFi protocols
- **Merchant Token Account:** Escrow for pending merchant payments