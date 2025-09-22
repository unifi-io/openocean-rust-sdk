# openocean-sdk

[![Crates.io](https://img.shields.io/crates/v/openocean-sdk.svg)](https://crates.io/crates/openocean-sdk)
[![Documentation](https://docs.rs/openocean-sdk/badge.svg)](https://docs.rs/openocean-sdk)
[![License](https://img.shields.io/crates/l/openocean-sdk.svg)](https://github.com/unifi-io/openocean-rust-sdk/blob/main/LICENSE)

Async Rust SDK for [OpenOcean](https://openocean.finance/) APIs. OpenOcean is a decentralized aggregation trading platform that provides cross-chain trading and liquidity aggregation services.

## Features

- 🚀 **Async Support** - High-performance async HTTP client based on `tokio` and `reqwest`
- 🔗 **Multi-chain Support** - Support for Ethereum, BSC, Arbitrum, Polygon and other major blockchains
- 🛡️ **Type Safety** - Full Rust type system support with compile-time error checking
- ⚡ **Flexible Configuration** - Customizable timeout, user agent, base URL and other settings
- 🔧 **TLS Options** - Support for both `rustls` and `native-tls` backends
- 📦 **Lightweight** - Minimal dependencies, focused on core functionality

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
openocean-sdk = "0.1.0"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

### Feature Selection

The SDK provides two TLS backend options (defaults to `rustls-tls`):

```toml
# Use rustls (recommended, pure Rust implementation)
openocean-sdk = { version = "0.1.0", default-features = false, features = ["rustls-tls"] }

# Or use native-tls (system native TLS)
openocean-sdk = { version = "0.1.0", default-features = false, features = ["native-tls"] }
```

## Quickstart

### Basic Usage

```rust
use openocean_sdk::{OpenoceanClient, OpenoceanConfig, Chain};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = OpenoceanClient::new(OpenoceanConfig::default())?;
    
    // Get BSC chain gas prices
    let gas_response = client.get_price(Chain::Bsc).await?;
    println!("Standard gas: {} Gwei", gas_response.data.standard);
    println!("Fast gas: {} Gwei", gas_response.data.fast);
    println!("Instant gas: {} Gwei", gas_response.data.instant);
    
    // Get token list
    let token_list = client.get_token_list(Chain::Bsc).await?;
    println!("Token count: {}", token_list.data.len());
    
    Ok(())
}
```

### Custom Configuration

```rust
use openocean_sdk::{OpenoceanClient, OpenoceanConfig, Chain};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use builder pattern for custom configuration
    let config = OpenoceanConfig::builder()
        .timeout(Duration::from_secs(60))
        .user_agent("my-app/1.0.0")
        .build();
    
    let client = OpenoceanClient::new(config)?;
    
    // Use client...
    Ok(())
}
```

## Supported Blockchains

The SDK supports a wide range of blockchain networks, including both EVM-compatible and non-EVM chains:

### EVM-Compatible Chains

- **Ethereum** (`Chain::Eth`) - Ethereum mainnet
- **BSC** (`Chain::Bsc`) - Binance Smart Chain
- **Arbitrum** (`Chain::Arbitrum`) - Arbitrum One
- **Polygon** (`Chain::Polygon`) - Polygon network
- **Base** (`Chain::Base`) - Base network
- **Linea** (`Chain::Linea`) - Linea network
- **Fantom** (`Chain::Fantom`) - Fantom Opera
- **Avalanche** (`Chain::Avalanche`) - Avalanche C-Chain
- **Optimism** (`Chain::Optimism`) - Optimism
- **Moonriver** (`Chain::Moonriver`) - Moonriver
- **Aurora** (`Chain::Aurora`) - Aurora
- **Cronos** (`Chain::Cronos`) - Cronos
- **Harmony** (`Chain::Harmony`) - Harmony
- **Kava** (`Chain::Kava`) - Kava
- **Metis Andromeda** (`Chain::MetisAndromeda`) - Metis Andromeda
- **Celo** (`Chain::Celo`) - Celo
- **Telos** (`Chain::Telos`) - Telos EVM
- **Polygon zkEVM** (`Chain::PolygonZkEVM`) - Polygon zkEVM
- **Gnosis** (`Chain::Gnosis`) - Gnosis Chain
- **OpBNB** (`Chain::OpBNB`) - OpBNB
- **Mantle** (`Chain::Mantle`) - Mantle
- **Manta** (`Chain::Manta`) - Manta Network
- **Scroll** (`Chain::Scroll`) - Scroll
- **Blast** (`Chain::Blast`) - Blast
- **Mode** (`Chain::Mode`) - Mode
- **Rootstock** (`Chain::Rootstock`) - Rootstock
- **Sei** (`Chain::Sei`) - Sei
- **Gravity** (`Chain::Gravity`) - Gravity
- **Apechain** (`Chain::Apechain`) - Apechain
- **Sonic** (`Chain::Sonic`) - Sonic
- **Berachain** (`Chain::Berachain`) - Berachain
- **Monad Testnet** (`Chain::MonadTestnet`) - Monad Testnet
- **UniChain** (`Chain::UniChain`) - UniChain
- **Flare** (`Chain::Flare`) - Flare
- **Swell** (`Chain::Swell`) - Swell
- **HyperEVM** (`Chain::HyperEVM`) - HyperEVM
- **Plume** (`Chain::Plume`) - Plume
- **TAC** (`Chain::TAC`) - TAC
- **zkSync Era** (`Chain::ZkSyncEra`) - zkSync Era

### Non-EVM Chains

- **Solana** (`Chain::Solana`) - Solana
- **Ontology** (`Chain::Ontology`) - Ontology
- **Near** (`Chain::Near`) - NEAR Protocol
- **Starknet** (`Chain::Starknet`) - Starknet

## API Reference

### Client Methods

#### `get_price(chain: Chain) -> Result<OpenoceanGasResponse, OpenoceanError>`

Get gas price information for the specified chain.

```rust
let gas_response = client.get_price(Chain::Bsc).await?;
// gas_response.data.standard  - Standard gas price
// gas_response.data.fast      - Fast gas price  
// gas_response.data.instant   - Instant gas price
```

#### `get_token_list(chain: Chain) -> Result<OpenoceanBaseResponse<Vec<OpenoceanToken>>, OpenoceanError>`

Get token list for the specified chain.

```rust
let token_list = client.get_token_list(Chain::Bsc).await?;
for token in &token_list.data {
    println!("Token: {} ({})", token.name, token.symbol);
    println!("Address: {}", token.address);
    println!("Decimals: {}", token.decimals);
}
```

### Data Structures

#### `OpenoceanToken`

Token information structure:

```rust
pub struct OpenoceanToken {
    pub id: i32,                    // Token ID
    pub code: String,               // Token code
    pub name: String,               // Token name
    pub address: String,            // Contract address
    pub decimals: u8,               // Decimals
    pub symbol: String,             // Token symbol
    pub icon: String,               // Icon URL
    pub chain: String,              // Chain name
    pub create_time: String,        // Creation time
    pub chain_id: Option<i32>,      // Chain ID
    pub custom_symbol: Option<String>,    // Custom symbol
    pub custom_address: Option<String>,   // Custom address
}
```

#### `OpenoceanGasPrice`

Gas price information:

```rust
pub struct OpenoceanGasPrice {
    pub standard: f64,    // Standard gas price
    pub fast: f64,        // Fast gas price
    pub instant: f64,     // Instant gas price
}
```

## Error Handling

The SDK uses `thiserror` to provide detailed error types:

```rust
use openocean_sdk::OpenoceanError;

match client.get_price(Chain::Bsc).await {
    Ok(response) => println!("Gas price: {:?}", response),
    Err(OpenoceanError::Network(msg)) => eprintln!("Network error: {}", msg),
    Err(OpenoceanError::Http { status, body }) => eprintln!("HTTP error: {} - {}", status, body),
    Err(OpenoceanError::Parse(msg)) => eprintln!("Parse error: {}", msg),
    Err(OpenoceanError::Internal(msg)) => eprintln!("Internal error: {}", msg),
}
```

## Examples

Check the `examples/` directory for more usage examples:

### Running Examples

```bash
# Run basic usage example
cargo run --example basic_usage

# Run multi-chain example
cargo run --example multi_chain

# Run error handling example
cargo run --example error_handling
```

### Available Examples

- **`basic_usage`** - Demonstrates basic SDK usage with gas prices and token lists
- **`multi_chain`** - Shows how to work with multiple blockchain networks
- **`error_handling`** - Comprehensive error handling examples and best practices

## Contributing

Contributions are welcome! Please check the [GitHub repository](https://github.com/unifi-io/openocean-rust-sdk) to learn how to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Related Links

- [OpenOcean Website](https://openocean.finance/)
- [OpenOcean API Documentation](https://openocean.finance/docs)
- [Rust Documentation](https://docs.rs/openocean-sdk)