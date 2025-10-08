use openocean_sdk::{OpenoceanClient, OpenoceanConfig, Chain};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with custom configuration
    let config = OpenoceanConfig::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("multi-chain-example/1.0.0")
        .build();
    
    let client = OpenoceanClient::new(config)?;
    let swap = Swap::new(&client);
    
    println!("=== OpenOcean SDK Multi-Chain Example ===\n");
    
    // EVM-Compatible chains
    let evm_chains = vec![
        (Chain::Eth, "Ethereum"),
        (Chain::Bsc, "Binance Smart Chain"),
        (Chain::Arbitrum, "Arbitrum One"),
        (Chain::Polygon, "Polygon"),
        (Chain::Base, "Base"),
        (Chain::Linea, "Linea"),
        (Chain::Fantom, "Fantom"),
        (Chain::Avalanche, "Avalanche"),
        (Chain::Optimism, "Optimism"),
        (Chain::Moonriver, "Moonriver"),
        (Chain::Aurora, "Aurora"),
        (Chain::Cronos, "Cronos"),
        (Chain::Harmony, "Harmony"),
        (Chain::Kava, "Kava"),
        (Chain::MetisAndromeda, "Metis Andromeda"),
        (Chain::Celo, "Celo"),
        (Chain::ZkSyncEra, "zkSync Era"),
        (Chain::Telos, "Telos EVM"),
        (Chain::PolygonZkEVM, "Polygon zkEVM"),
        (Chain::Gnosis, "Gnosis Chain"),
        (Chain::OpBNB, "OpBNB"),
        (Chain::Mantle, "Mantle"),
        (Chain::Manta, "Manta Network"),
        (Chain::Scroll, "Scroll"),
        (Chain::Blast, "Blast"),
        (Chain::Mode, "Mode"),
        (Chain::Rootstock, "Rootstock"),
        (Chain::Sei, "Sei"),
        (Chain::Gravity, "Gravity"),
        (Chain::Apechain, "Apechain"),
        (Chain::Sonic, "Sonic"),
        (Chain::Berachain, "Berachain"),
        (Chain::MonadTestnet, "Monad Testnet"),
        (Chain::UniChain, "UniChain"),
        (Chain::Flare, "Flare"),
        (Chain::Swell, "Swell"),
        (Chain::HyperEVM, "HyperEVM"),
        (Chain::Plume, "Plume"),
        (Chain::TAC, "TAC"),
    ];
    
    // Non-EVM chains
    let non_evm_chains = vec![
        (Chain::Solana, "Solana"),
        (Chain::Ontology, "Ontology"),
        (Chain::Near, "NEAR Protocol"),
        (Chain::Starknet, "Starknet"),
    ];
    
    // Get gas prices for all supported chains
    println!("Getting gas prices for all supported chains...\n");
    println!("Note: This example will test all {} EVM-compatible chains", evm_chains.len());
    println!("For demonstration purposes, we'll limit to first 10 chains to avoid overwhelming output.\n");
    
    for (chain, name) in evm_chains.iter().take(10) {
        println!("--- {} ---", name);
        
        match swap.get_price(*chain).await {
            Ok(gas_response) => {
                println!("✅ Gas prices:");
                println!("  Standard: {:.2} Gwei", gas_response.data.standard);
                println!("  Fast: {:.2} Gwei", gas_response.data.fast);
                println!("  Instant: {:.2} Gwei", gas_response.data.instant);
            }
            Err(e) => {
                println!("❌ Failed to get gas prices: {}", e);
            }
        }
        
        println!();
    }
    
    // Get token count for each chain
    println!("Getting token counts for all supported chains...\n");
    
    for (chain, name) in evm_chains.iter().take(5) {
        println!("--- {} Token Count ---", name);
        
        match swap.get_token_list(*chain).await {
            Ok(token_list) => {
                let token_list = token_list.data.unwrap();
                println!("✅ Total tokens: {}", token_list.len());
                
                // Show some popular tokens if available
                let popular_tokens: Vec<_> = token_list
                    .iter()
                    .filter(|token| {
                        let symbol = token.symbol.to_lowercase();
                        symbol == "eth" || symbol == "btc" || symbol == "usdt" || 
                        symbol == "usdc" || symbol == "bnb" || symbol == "matic"
                    })
                    .take(3)
                    .collect();
                
                if !popular_tokens.is_empty() {
                    println!("  Popular tokens found:");
                    for token in popular_tokens {
                        println!("    - {} ({})", token.name, token.symbol);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to get token list: {}", e);
            }
        }
        
        println!();
    }
    
    // Summary of all supported chains
    println!("=== Summary ===");
    println!("Total EVM-compatible chains supported: {}", evm_chains.len());
    println!("Total non-EVM chains supported: {}", non_evm_chains.len());
    println!("Total chains supported: {}", evm_chains.len() + non_evm_chains.len());
    
    println!("\nAll supported chains:");
    println!("EVM-Compatible:");
    for (_, name) in &evm_chains {
        println!("  - {}", name);
    }
    
    println!("\nNon-EVM:");
    for (_, name) in &non_evm_chains {
        println!("  - {}", name);
    }
    
    Ok(())
}
