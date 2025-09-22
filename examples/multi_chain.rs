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
    
    println!("=== OpenOcean SDK Multi-Chain Example ===\n");
    
    let chains = vec![
        (Chain::Eth, "Ethereum"),
        (Chain::Bsc, "Binance Smart Chain"), 
        (Chain::ZkSyncEra, "zkSync Era"),
        (Chain::Polygon, "Polygon"),
        (Chain::Base, "Base"),
        (Chain::Linea, "Linea"),
        (Chain::Fantom, "Fantom"),
        (Chain::Avalanche, "Avalanche"),
        (Chain::Arbitrum, "Arbitrum"),
        (Chain::Optimism, "Optimism"),
        (Chain::Moonriver, "Moonriver"),
        (Chain::Aurora, "Aurora"),
        (Chain::Cronos, "Cronos"),
        (Chain::Harmony, "Harmony"),
        (Chain::Kava, "Kava"),
        (Chain::MetisAndromeda, "Metis Andromeda"),
        (Chain::Celo, "Celo"),
    ];
    
    // Get gas prices for all supported chains
    println!("Getting gas prices for all supported chains...\n");
    
    for (chain, name) in chains {
        println!("--- {} ---", name);
        
        match client.get_price(chain).await {
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
    
    let chains_for_tokens = vec![
        (Chain::Eth, "Ethereum"),
        (Chain::Bsc, "Binance Smart Chain"),
        (Chain::Arbitrum, "Arbitrum One"),
        (Chain::Polygon, "Polygon"),
    ];
    
    for (chain, name) in chains_for_tokens {
        println!("--- {} Token Count ---", name);
        
        match client.get_token_list(chain).await {
            Ok(token_list) => {
                println!("✅ Total tokens: {}", token_list.data.len());
                
                // Show some popular tokens if available
                let popular_tokens: Vec<_> = token_list.data
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
    
    Ok(())
}
