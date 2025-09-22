# openocean-sdk

Async Rust SDK for [OpenOcean](https://openocean.finance/) APIs.

## Quickstart

```rust,no_run
use openocean_sdk::{OpenoceanProvider, OpenoceanConfig, Chain};

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let provider = OpenoceanProvider::new(
    OpenoceanConfig::builder().chain(Chain::Bsc).build()
)?;
let gas = provider.gas_price().await?;
println!("fast gas: {}", gas.data.fast);
# Ok(()) }
```