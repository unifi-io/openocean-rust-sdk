use openocean_sdk::{OpenoceanClient, OpenoceanConfig, Chain, OpenoceanError};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenOcean SDK Error Handling Example ===\n");
    
    // Create client with very short timeout to demonstrate error handling
    let config = OpenoceanConfig::builder()
        .timeout(Duration::from_millis(100)) // Very short timeout
        .user_agent("error-handling-example/1.0.0")
        .build();
    
    let client = OpenoceanClient::new(config)?;
    
    // Example 1: Successful request
    println!("1. Successful request:");
    match client.get_price(Chain::Bsc).await {
        Ok(gas_response) => {
            println!("✅ Success! Gas prices retrieved:");
            println!("  Standard: {} Gwei", gas_response.data.standard);
            println!("  Fast: {} Gwei", gas_response.data.fast);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            print_error_details(&e);
        }
    }
    
    println!();
    
    // Example 2: Error handling with different error types
    println!("2. Error handling examples:");
    
    // Try to get token list
    match client.get_token_list(Chain::Bsc).await {
        Ok(token_list) => {
            println!("✅ Token list retrieved: {} tokens", token_list.data.len());
        }
        Err(e) => {
            println!("❌ Error getting token list: {}", e);
            print_error_details(&e);
        }
    }
    
    println!();
    
    // Example 3: Demonstrating different error types
    println!("3. Error type demonstration:");
    demonstrate_error_types().await;
    
    Ok(())
}

fn print_error_details(error: &OpenoceanError) {
    match error {
        OpenoceanError::Network(msg) => {
            println!("  📡 Network Error: {}", msg);
            println!("  💡 This could be due to connection issues, timeouts, or DNS problems.");
        }
        OpenoceanError::Http { status, body } => {
            println!("  🌐 HTTP Error: Status {}, Body: {}", status, body);
            match *status {
                400 => println!("  💡 Bad Request: Check your request parameters."),
                401 => println!("  💡 Unauthorized: Check your API credentials."),
                403 => println!("  💡 Forbidden: You don't have permission to access this resource."),
                404 => println!("  💡 Not Found: The requested resource doesn't exist."),
                429 => println!("  💡 Rate Limited: You're making too many requests."),
                500..=599 => println!("  💡 Server Error: The API server is having issues."),
                _ => println!("  💡 Unknown HTTP error."),
            }
        }
        OpenoceanError::Parse(msg) => {
            println!("  🔍 Parse Error: {}", msg);
            println!("  💡 This could be due to unexpected response format or JSON parsing issues.");
        }
        OpenoceanError::Internal(msg) => {
            println!("  ⚙️ Internal Error: {}", msg);
            println!("  💡 This is an internal SDK error. Please report this issue.");
        }
    }
}

async fn demonstrate_error_types() {
    // Create a client with invalid base URL to demonstrate different error types
    let invalid_config = OpenoceanConfig::builder()
        .base_url("https://invalid-url-that-does-not-exist.com")
        .timeout(Duration::from_secs(5))
        .build();
    
    if let Ok(client) = OpenoceanClient::new(invalid_config) {
        match client.get_price(Chain::Bsc).await {
            Err(OpenoceanError::Network(msg)) => {
                println!("✅ Network error caught: {}", msg);
            }
            Err(e) => {
                println!("❌ Unexpected error type: {}", e);
            }
            Ok(_) => {
                println!("❌ Unexpected success with invalid URL");
            }
        }
    } else {
        println!("❌ Failed to create client with invalid URL");
    }
}
