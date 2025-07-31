# Solana AMM (Automated Market Maker)

A decentralized Automated Market Maker (AMM) built on Solana using the Anchor framework. This AMM allows users to create liquidity pools, provide liquidity, and swap tokens with constant product pricing.

## 🚀 Features

- **Initialize Pool**: Create new AMM pools with custom parameters
- **Add Liquidity**: Deposit tokens to earn trading fees
- **Remove Liquidity**: Withdraw tokens from pools
- **Token Swaps**: Exchange tokens using constant product formula
- **Fee Management**: Configurable trading fees for pool creators

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- [Node.js](https://nodejs.org/) (v18+ recommended)
- [Rust](https://rustup.rs/) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.31.1)

## 🛠️ Installation

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd amm
   ```

2. **Install dependencies**
   ```bash
   cd tests
   npm install
   ```

3. **Configure Solana CLI**
   ```bash
   # Set to localnet for development
   solana config set --url localhost
   
   # Generate a new keypair (if needed)
   solana-keygen new
   ```

## 🔧 Building

Build the Anchor program:

```bash
anchor build
```

## 🧪 Testing

### Local Testing

1. **Start local validator**
   ```bash
   solana-test-validator
   ```

2. **Run tests**
   ```bash
   anchor test
   # or
   cd tests && npm run test:local
   ```

### Network Testing

```bash
# Test on devnet
npm run test:devnet

# Test on mainnet (use with caution)
npm run test:mainnet
```

## 🚀 Deployment

### Local Deployment
```bash
anchor deploy
# or
npm run deploy:local
```

### Devnet Deployment
```bash
npm run deploy:devnet
```

### Mainnet Deployment
```bash
npm run deploy:mainnet
```

## 📖 Usage

### Core Instructions

#### 1. Initialize Pool
Creates a new AMM pool with specified parameters.

```typescript
await program.methods
  .initialize(seed, fee, authority)
  .accounts({
    // ... accounts
  })
  .rpc();
```

**Parameters:**
- `seed`: Unique identifier for the pool
- `fee`: Trading fee (in basis points, e.g., 30 = 0.3%)
- `authority`: Optional pool authority

#### 2. Add Liquidity (Deposit)
Provides liquidity to the pool in exchange for LP tokens.

```typescript
await program.methods
  .deposite(amount, maxX, maxY)
  .accounts({
    // ... accounts
  })
  .rpc();
```

**Parameters:**
- `amount`: Amount of LP tokens to mint
- `maxX`: Maximum amount of token X to deposit
- `maxY`: Maximum amount of token Y to deposit

#### 3. Remove Liquidity (Withdraw)
Burns LP tokens to withdraw underlying assets.

```typescript
await program.methods
  .withdraw(amount, minX, minY)
  .accounts({
    // ... accounts
  })
  .rpc();
```

**Parameters:**
- `amount`: Amount of LP tokens to burn
- `minX`: Minimum amount of token X to receive
- `minY`: Minimum amount of token Y to receive

#### 4. Swap Tokens
Exchanges one token for another using the constant product formula.

```typescript
await program.methods
  .swap(isX, amountIn, minAmountOut)
  .accounts({
    // ... accounts
  })
  .rpc();
```

**Parameters:**
- `isX`: `true` to swap X for Y, `false` to swap Y for X
- `amountIn`: Amount of input tokens
- `minAmountOut`: Minimum amount of output tokens (slippage protection)

## 📁 Project Structure

```
amm/
├── Anchor.toml                 # Anchor configuration
├── Cargo.toml                  # Rust workspace configuration
├── programs/amm/               # Main program directory
│   ├── src/
│   │   ├── lib.rs             # Program entry point
│   │   ├── instructions/      # Instruction handlers
│   │   │   ├── initialize.rs  # Pool initialization
│   │   │   ├── deposite.rs    # Liquidity deposit
│   │   │   ├── withdraw.rs    # Liquidity withdrawal
│   │   │   └── swap.rs        # Token swapping
│   │   ├── states/            # Program state definitions
│   │   │   └── config.rs      # Pool configuration
│   │   ├── errors.rs          # Custom error definitions
│   │   └── constant.rs        # Program constants
└── tests/                     # TypeScript tests
    ├── ammTest.ts            # Main test suite
    ├── package.json          # Test dependencies
    └── tsconfig.json         # TypeScript configuration
```

## 🔧 Configuration

### Anchor.toml
- **Anchor Version**: 0.31.1
- **Default Cluster**: Localnet

### Environment Setup

1. Update `Anchor.toml` with your desired cluster
2. Configure wallet path in `Anchor.toml`
3. Ensure sufficient SOL balance for deployments

## 🧪 Example Test Flow

The test suite demonstrates a complete AMM workflow:

1. **Setup**: Create token mints and accounts
2. **Initialize**: Create AMM pool with 0.3% fee
3. **Deposit**: Add initial liquidity
4. **Swap**: Execute token swaps
5. **Withdraw**: Remove liquidity

## ⚠️ Important Notes

- This is a **development version** - audit before mainnet use
- Contains a typo: `deposite` should be `deposit` (consistent in codebase)
- Always test thoroughly on devnet before mainnet deployment
- Ensure proper slippage protection in production

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `anchor test`
5. Submit a pull request

## 📄 License

This project is licensed under the ISC License.

## 🔗 Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Documentation](https://spl.solana.com/token)

---

**⚡ Quick Start:**
```bash
anchor build && anchor test
```
