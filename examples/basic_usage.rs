use openocean_sdk::{OpenoceanClient, OpenoceanConfig, Chain};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = OpenoceanClient::new(OpenoceanConfig::default())?;
    
    println!("=== OpenOcean SDK Basic Usage Example ===\n");
    
    // Get BSC chain gas prices
    println!("Getting BSC chain gas prices...");
    match client.get_price(Chain::Bsc).await {
        Ok(gas_response) => {
            println!("✅ Gas prices retrieved successfully:");
            println!("  Standard: {} Gwei", gas_response.data.standard);
            println!("  Fast: {} Gwei", gas_response.data.fast);
            println!("  Instant: {} Gwei", gas_response.data.instant);
        }
        Err(e) => {
            println!("❌ Failed to get gas prices: {}", e);
        }
    }
    
    println!();
    
    // Get BSC chain token list
    println!("Getting BSC chain token list...");
    match client.get_token_list(Chain::Bsc).await {
        Ok(token_list) => {
            println!("✅ Token list retrieved successfully, {} tokens found", token_list.data.len());
            
            // Display first 5 tokens
            println!("First 5 tokens:");
            for (i, token) in token_list.data.iter().take(5).enumerate() {
                println!("  {}. {} ({})", i + 1, token.name, token.symbol);
                println!("     Address: {}", token.address);
                println!("     Decimals: {}", token.decimals);
                println!();
            }
        }
        Err(e) => {
            println!("❌ Failed to get token list: {}", e);
        }
    }
    
    Ok(())
}
