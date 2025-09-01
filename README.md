# Linketh - Arbitrum Stylus Smart Contract

> ⚠️ **WARNING: This code is not production ready yet.** This is an experimental implementation and should not be used in production environments without thorough security audits and testing.

A decentralized, Linktree-like DID (Decentralized Identity) profiles solution built as an Arbitrum Stylus WASM smart contract. Linketh allows users to create decentralized profiles linked to their ENS (Ethereum Name Service) domains, storing profile data on IPFS while maintaining on-chain verification and quick links.

## Features

- **ENS-Verified Profiles**: Profile creation requires ENS domain ownership verification
- **IPFS Integration**: Profile data and avatars stored on IPFS using Content Identifiers (CIDs)
- **Quick Links**: Up to 5 customizable quick links per profile (social media, websites, etc.)
- **Profile Management**: Create, update, delete, and transfer profiles between addresses
- **Gas Efficient**: Built on Arbitrum Stylus for low-cost WASM execution
- **Modular Architecture**: Clean separation of ENS utilities and contract logic

## Architecture

- **Language**: Rust compiled to WASM via Arbitrum Stylus
- **Storage**: On-chain profile metadata, IPFS for content
- **ENS Integration**: Verifies domain ownership via ENS registry
- **Events**: Comprehensive event logging for profile operations

## Project Structure

```
src/
├── lib.rs          # Main contract implementation
├── ens.rs          # ENS utilities and namehash implementation  
├── tests.rs        # Comprehensive unit tests
└── main.rs         # Binary entry point
```

## Prerequisites

- **Rust**: Install via [rustup.rs](https://rustup.rs/)
- **Stylus SDK**: Arbitrum's Rust SDK for WASM contracts
- **Cargo Stylus**: CLI tool for Stylus development

### Installation

1. Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

2. Add WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Install Cargo Stylus:
```bash
cargo install --force cargo-stylus cargo-stylus-check
```

## Development

### Building the Contract

```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Check contract compatibility with Stylus
cargo stylus check
```

### Running Tests

The project includes comprehensive unit tests covering:
- ENS namehash algorithm validation
- Profile storage operations
- Quick links management
- Edge cases and error handling

```bash
# Run all tests
cargo test

# Run specific test module
cargo test --lib ens::tests
cargo test --lib tests::tests

# Run with verbose output
cargo test -- --nocapture
```

### Test Coverage

- **ENS Module**: 9 tests validating namehash implementation
- **Main Contract**: 11 tests covering profile operations
- **Total**: 20 comprehensive unit tests

## Configuration

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
# Arbitrum RPC endpoint
RPC_URL=https://arb1.arbitrum.io/rpc

# Deployed contract address (after deployment)
STYLUS_CONTRACT_ADDRESS=

# Private key file path for deployment
PRIV_KEY_PATH=

# ENS Registry address (mainnet: 0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e)
ENS_REGISTRY_ADDRESS=0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e
```

## Deployment

### 1. Local Testing

```bash
# Check contract compatibility
cargo stylus check

# Estimate deployment gas
cargo stylus deploy --private-key-path=<PRIVKEY_PATH> --estimate-gas
```

### 2. Testnet Deployment

```bash
# Deploy to Arbitrum Sepolia
cargo stylus deploy \
  --private-key-path=<PRIVKEY_PATH> \
  --rpc-url=https://sepolia-rollup.arbitrum.io/rpc
```

### 3. Mainnet Deployment

```bash
# Deploy to Arbitrum One
cargo stylus deploy \
  --private-key-path=<PRIVKEY_PATH> \
  --rpc-url=https://arb1.arbitrum.io/rpc
```

### 4. Contract Initialization

After deployment, initialize the contract with ENS registry:

```bash
# Initialize ENS registry address
cast send <CONTRACT_ADDRESS> "init(address)" <ENS_REGISTRY_ADDRESS> \
  --private-key <PRIVATE_KEY> \
  --rpc-url <RPC_URL>
```

## Usage

### Contract Interface

#### Profile Management
```rust
// Initialize contract with ENS registry
fn init(ens_registry_address: Address)

// Create profile (requires ENS ownership)
fn create_profile(ens_name: String, cid: String, display: String, avatar: String)

// Update profile fields
fn update_profile(cid: String, display: String, avatar: String)

// Delete profile and all associated data
fn delete_profile()

// Transfer profile to new address
fn transfer_profile(new_owner: Address)
```

#### Quick Links
```rust
// Set quick links (max 5)
fn set_quick_links(links: Vec<(String, String)>)

// Get user's quick links
fn get_quick_links(owner: Address) -> Vec<(String, String)>
```

#### Profile Queries
```rust
// Get profile data
fn get_profile(owner: Address) -> (String, String, String, String)
// Returns: (ens, cid, display_name, avatar_cid)
```

### Events

```rust
event ProfileCreated(address indexed owner, string ens, string cid, string display_name, string avatar_cid);
event ProfileUpdated(address indexed owner, string cid, string display_name, string avatar_cid);
event ProfileDeleted(address indexed owner);
event QuickLinksUpdated(address indexed owner);
```

## Example Usage

### Creating a Profile

1. **Own an ENS domain** (e.g., `yourname.eth`)
2. **Store content on IPFS** and get CID
3. **Call create_profile**:

```javascript
// Using ethers.js or similar
await contract.create_profile(
  "yourname.eth",
  "QmYourContentCID...",
  "Your Display Name",
  "QmYourAvatarCID..."
);
```

### Setting Quick Links

```javascript
const links = [
  ["Twitter", "https://twitter.com/yourhandle"],
  ["GitHub", "https://github.com/yourusername"],
  ["Website", "https://yourwebsite.com"]
];

await contract.set_quick_links(links);
```

### ABI Export

Export the Solidity ABI for frontend integration:

```bash
cargo stylus export-abi
```

## Docker Support

The project includes a multi-stage Dockerfile for containerized builds and deployment:

### Docker Build Stages

1. **Runtime Stage** (for development/testing):
```bash
# Build development image
docker build --target runtime -t linketh:dev .

# Run tests in container
docker run --rm linketh:dev cargo test --lib

# Check Stylus compatibility
docker run --rm linketh:dev cargo stylus check
```

2. **WASM Stage** (production-ready binary):
```bash
# Build minimal WASM image
docker build --target wasm -t linketh:wasm .

# Extract WASM binary
docker create --name temp linketh:wasm
docker cp temp:/linketh.wasm ./linketh.wasm
docker rm temp
```

### Docker Compose (Optional)

For development environments, you can create a `docker-compose.yml`:

```yaml
version: '3.8'
services:
  linketh-dev:
    build:
      context: .
      target: runtime
    volumes:
      - .:/app
      - cargo-cache:/usr/local/cargo/registry
    working_dir: /app
    command: cargo stylus check

volumes:
  cargo-cache:
```

## CI/CD Pipeline

The project includes comprehensive GitHub Actions workflows:

### Main CI Pipeline (`.github/workflows/ci.yml`)

**Triggered on**: Push/PR to `main` and `develop` branches

**Jobs**:
- 🔍 **Code Validation**: Formatting, linting, security audit
- 🏗️ **Build and Test**: Multi-version Rust builds with test coverage  
- 🛡️ **Stylus Check**: **CRITICAL** - Ensures WASM compatibility
- 🐳 **Docker Build**: Container build verification
- 📚 **Documentation**: Docs and README validation
- 🔗 **Integration Check**: Full deployment pipeline simulation
- 📦 **Release Artifacts**: WASM and ABI generation (main branch only)

### Security Pipeline (`.github/workflows/security.yml`)

**Triggered on**: Push, PR, daily schedule

**Jobs**:
- 🔐 **Security Audit**: Dependency vulnerabilities, license compliance
- 🔬 **Supply Chain Check**: Unsafe code analysis, proc macro audit

### Branch Protection

**Main Branch Protection**:
- ✅ Requires PR reviews (minimum 1)
- ✅ Requires status checks: `CI Success`, `Stylus Compatibility Check`
- ✅ Requires branches to be up to date
- ❌ No force pushes or deletions allowed
- ✅ Includes administrators

**Develop Branch Protection**:
- ✅ Requires PR reviews (minimum 1)  
- ✅ Requires status checks: `CI Success`, `Stylus Compatibility Check`
- ❌ No force pushes or deletions allowed

> 📋 See [`.github/BRANCH_PROTECTION.md`](.github/BRANCH_PROTECTION.md) for detailed configuration

### Critical Status Checks

**🚨 Stylus Compatibility Check** - This check is **MANDATORY** for merging:
- Validates WASM compilation for `wasm32-unknown-unknown`
- Runs `cargo stylus check` to ensure Arbitrum compatibility  
- Enforces 128KB WASM size limit
- **Failure blocks all merges to protected branches**

### Using CI/CD

```bash
# Check CI status locally
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --lib
cargo stylus check

# Build Docker image locally
docker build --target runtime -t linketh:test .
docker run --rm linketh:test cargo stylus check
```

## Current TODOs & Limitations

### High Priority TODOs
- [ ] **L1 ENS Integration**: Implement L1 ENS contract queries using Arbitrum's `getStorageAtL1` for proper ENS ownership verification
- [ ] **Enhanced Testing**: Create better integration tests with ENS mocking and end-to-end scenarios
- [ ] **Security Audit**: Professional security review required before production use
- [ ] **Gas Optimization**: Further optimize WASM contract size and execution costs

### Future Enhancements
- [ ] **Frontend Interface**: Web interface for profile management
- [ ] **IPFS Pinning**: Integration with IPFS pinning services
- [ ] **Profile Templates**: Predefined profile templates and themes
- [ ] **Social Graph**: Following/followers functionality
- [ ] **Profile Analytics**: View statistics and engagement metrics

### Current Limitations
- ENS ownership verification currently requires proper L1 ENS registry access implementation
- Profile data stored on IPFS may have availability concerns without pinning
- No built-in IPFS pinning service integration
- Limited to 5 quick links per profile
- No admin controls or upgrade mechanisms

## Security Considerations

⚠️ **This contract is experimental and not audited**

### Key Security Points
- ENS ownership verification is critical for profile authenticity
- IPFS content addressing provides integrity but not guaranteed availability
- Profile transfers are irreversible operations
- No admin controls or emergency pause mechanisms
- Smart contract interactions should be carefully validated

### Recommended Security Practices
- Always verify ENS ownership before profile creation
- Use reputable IPFS gateways and consider pinning services
- Test all operations on testnets before mainnet use
- Implement proper access controls in frontend applications

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add comprehensive tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Check Stylus compatibility: `cargo stylus check`
6. Submit a pull request

### Development Guidelines

- Follow Rust best practices and idioms
- Maintain comprehensive test coverage (currently 20 tests)
- Use clear, descriptive commit messages
- Update documentation for new features
- Ensure modular code organization

### Code Structure
- **`src/lib.rs`**: Main contract logic and storage definitions
- **`src/ens.rs`**: ENS-specific utilities with comprehensive tests
- **`src/tests.rs`**: Unit tests for contract functionality
- **`src/main.rs`**: Binary entry point for examples

## Testing

The project includes a comprehensive test suite:

```bash
# Run all 20 tests
cargo test --lib

# Test specific modules
cargo test --lib ens::tests      # ENS namehash tests
cargo test --lib tests::tests    # Main contract tests

# View test details
cargo test --lib -- --nocapture
```

### Test Coverage
- ✅ ENS namehash algorithm validation (9 tests)
- ✅ Profile storage and retrieval (11 tests)
- ✅ Edge cases and error handling
- ✅ Multi-user independence
- ✅ Storage operations and data integrity

## License

MIT License - see LICENSE file for details.

## Resources

- [Arbitrum Stylus Documentation](https://docs.arbitrum.io/stylus/stylus-gentle-introduction)
- [Stylus SDK GitHub](https://github.com/OffchainLabs/stylus-sdk-rs)
- [ENS Documentation](https://docs.ens.domains/)
- [IPFS Documentation](https://docs.ipfs.io/)
- [Cargo Stylus CLI](https://github.com/OffchainLabs/cargo-stylus)

## Support

For questions, issues, or contributions:
- Open an issue on GitHub
- Join the Arbitrum Discord for Stylus discussions
- Review the Stylus documentation and examples
- Check the test suite for usage examples

---

**Built with ❤️ using Arbitrum Stylus, Rust, and decentralized technologies**

*Linketh enables truly decentralized identity profiles with ENS verification and IPFS storage, powered by efficient WASM execution on Arbitrum.*