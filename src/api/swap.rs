use std::collections::HashMap;

use crate::{models::swap::{DecodeInputDataResponse, GasPriceResponse, GasResponse, GetDexListResponse, GetTokenListResponse, GetTransactionResponse, QuoteParams, QuoteResponse, ReverseQuoteParams, ReverseQuoteResponse, SwapQuoteParams, SwapQuoteResponse}, Chain, OpenoceanClient, OpenoceanError};






#[derive(Clone)]
pub struct Swap<'a>{
    client: &'a OpenoceanClient,
}


// #[async_trait]
impl<'a> Swap<'a> {
    pub fn new(client: &'a OpenoceanClient) -> Self {
        Self { client }
    }

    pub async fn quote(&self, chain: Chain, params: &QuoteParams) -> Result<QuoteResponse, OpenoceanError> {
        let path = format!("/v4/{}/quote", chain);
        let res: QuoteResponse = self.client.get_json_with_query(&path, params).await?;
        Ok(res)
    }

    pub async fn get_token_list(&self, chain: Chain) -> Result<GetTokenListResponse, OpenoceanError> {
        let path = format!("/v4/{}/tokenList", chain);
        self.client.get_json(&path).await
    }

    pub async fn get_price(&self, chain: Chain) -> Result<GasResponse, OpenoceanError> {
        let path = format!("/v4/{}/gasPrice", chain);
        self.client.get_json(&path).await
    }

    pub async fn reverse_quote(&self, chain: Chain, parmas: &ReverseQuoteParams) -> Result<ReverseQuoteResponse, OpenoceanError> {
        let path = format!("/v4/{}/reverseQuote", chain);
        self.client.get_json_with_query(&path, parmas).await
    }

    pub async fn swap_quote(&self, chain: Chain, params: &SwapQuoteParams) -> Result<SwapQuoteResponse, OpenoceanError> {
        let path = format!("/v4/{}/swap", chain);
        self.client.get_json_with_query(&path, params).await
    }

    pub async fn get_dex_list(&self, chain: Chain) -> Result<GetDexListResponse, OpenoceanError> {
        let path = format!("/v4/{}/dexList", chain);
        self.client.get_json(&path).await
    }

    pub async fn get_transaction(&self, chain: Chain, hash: String) -> Result<GetTransactionResponse, OpenoceanError> {
        let mut query = HashMap::new();
        query.insert("hash", hash);

        let path = format!("/v4/{}/getTransaction", chain);
        self.client.get_json_with_query(&path, &query).await
    }

    pub async fn decode_input_data(&self, chain: Chain, data: String, method: String) -> Result<DecodeInputDataResponse, OpenoceanError> {
        let mut query = HashMap::new();
        query.insert("data", data);
        query.insert("method", method);

        let path = format!("/v4/{}/decodeInputData", chain);
        self.client.get_json_with_query(&path, &query).await
    }

    pub async fn get_gas_price(&self, chain: Chain) -> Result<GasPriceResponse, OpenoceanError> {
        let path = format!("/v4/{}/gasPrice", chain);
        self.client.get_json(&path).await
    }
}

#[cfg(test)]
mod tests {
    use crate::OpenoceanConfig;

    use super::*;

    #[tokio::test]
    async fn test_quote() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.quote(Chain::Bsc, &QuoteParams {
            in_token_address: "0x55d398326f99059ff775485246999027b3197955".to_string(),
            out_token_address: "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d".to_string(),
            amount_decimals: "5000000000000000000".to_string(),
            gas_price_decimals: "1000000000".to_string(),
            slippage: None,
            disabled_dex_ids: None,
            enabled_dex_ids: None,
        }).await.unwrap();
        assert_eq!(res.code, 200);
        println!("quote: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_token_list() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.get_token_list(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("token list: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_price() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.get_price(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("gas price: {}", serde_json::to_string_pretty(&res).unwrap());
    }


    #[tokio::test]
    async fn test_reverse_quote() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.reverse_quote(Chain::Bsc, &&ReverseQuoteParams {
            in_token_address: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(),
            out_token_address: "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d".to_string(),
            gas_price: 1.to_string(),
            amount: 1.to_string(),
            slippage: None,
            disabled_dex_ids: None,
            enabled_dex_ids: None,
        }).await.unwrap();

        assert_eq!(res.code, 200);
        println!("reverse quote: {}", serde_json::to_string_pretty(&res).unwrap());        
    }

    #[tokio::test]
    async fn test_swap_quote() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.swap_quote(Chain::Bsc, &SwapQuoteParams {
            in_token_address: "0x55d398326f99059ff775485246999027b3197955".to_string(),
            out_token_address: "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d".to_string(),
            amount_decimals: "5000000000000000000".to_string(),
            gas_price_decimals: "1000000000".to_string(),
            slippage: Some("1".to_string()),
            account: "0x9116780aEf4B376499358fa7dEeC00cCF64fA801".to_string(),
            referrer: Some("0xD4eb4cbB1ECbf96a1F0C67D958Ff6fBbB7B037BB".to_string()),
            referrer_fee: None,
            disabled_dex_ids: None,
            enabled_dex_ids: None,
            sender: None,
            mint_output: None,
        }).await.unwrap();
        assert_eq!(res.code, 200);
        println!("swap quote: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_dex_list() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.get_dex_list(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("dex list: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_transaction() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.get_transaction(Chain::Bsc, "0x756b98a89714be5c640ea9922aba12e0c94bc30e5a17e111d1aa40373cc24782".to_string()).await.unwrap();
        assert_eq!(res.code, 200);
        println!("transaction: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_decode_input_data() {
        let client = OpenoceanClient::new(crate::OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.decode_input_data(Chain::Bsc, "000000xxxxxx".to_string(), "swap".to_string()).await.unwrap();
        println!("decode input data: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_gas_price() {
        let client = OpenoceanClient::new(crate::OpenoceanConfig::default()).unwrap();
        let swap = Swap::new(&client);
        let res = swap.get_gas_price(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("gas price: {}", serde_json::to_string_pretty(&res).unwrap());

        let res = swap.get_gas_price(Chain::Eth).await.unwrap();
        assert_eq!(res.code, 200);
        println!("gas price: {}", serde_json::to_string_pretty(&res).unwrap());
    }
}