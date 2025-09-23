use std::{collections::HashMap, time::Duration};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Chain, DecodeInputDataResponse, GasPriceResponse, GetDexListResponse, GetTokenListResponse, GetTransactionResponse, OpenoceanError, QuoteParams, QuoteResponse, ReverseQuoteParams, ReverseQuoteResponse, SwapQuoteParams, SwapQuoteResponse};
use reqwest::{Client, Url};
use crate::{
    GasResponse,
};
use reqwest::header::CONTENT_TYPE;


// https://apis.openocean.finance/developer/widget/widget-v2


const BODY_SNIPPET_LIMIT: usize = 4096;

fn body_excerpt(bytes: &[u8]) -> String {
    let s = String::from_utf8_lossy(bytes);
    if s.len() > BODY_SNIPPET_LIMIT {
        format!("{}…", &s[..BODY_SNIPPET_LIMIT])
    } else {
        s.to_string()
    }
}




#[derive(Clone, Debug)]
pub struct OpenoceanConfig {
    pub base_url: Url,
    pub timeout: Duration,
    pub user_agent: Option<String>,
}

impl Default for OpenoceanConfig {
    fn default() -> Self {
        Self {
            base_url: Url::parse("https://open-api.openocean.finance").unwrap(),
            timeout: Duration::from_secs(30),
            user_agent: Some(format!("openocean-rs/{}", env!("CARGO_PKG_VERSION"))),
        }
    }
}

impl OpenoceanConfig {
    pub fn builder() -> OpenoceanConfigBuilder {
        OpenoceanConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct OpenoceanConfigBuilder {
    base_url: Option<Url>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
}

impl OpenoceanConfigBuilder {
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(Url::parse(url).expect("valid base url"));
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    pub fn build(self) -> OpenoceanConfig {
        OpenoceanConfig {
            base_url: self.base_url.unwrap_or_else(|| Url::parse("https://open-api.openocean.finance").unwrap()),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            user_agent: self.user_agent.or_else(|| Some(format!("openocean-rs/{}", env!("CARGO_PKG_VERSION")))),
        }
    }
}

pub struct OpenoceanClient {
    config: OpenoceanConfig,
    client: Client,
}

impl OpenoceanClient {
    pub fn new(config: OpenoceanConfig) -> Result<Self, OpenoceanError> {
        let mut builder = Client::builder().timeout(config.timeout);

        if let Some(ua) = &config.user_agent {
            builder = builder.user_agent(ua.clone());
        }

        let client = builder
            .build()
            .map_err(|e| OpenoceanError::Network(format!("failed to build http client: {e}")))?;

        Ok(Self { config, client })
    }

    #[inline]
    fn build_url(&self, path: &str) -> Result<Url, OpenoceanError> {
        self.config
            .base_url
            .join(path)
            .map_err(|e| OpenoceanError::Internal(format!("join url error: {e}")))
    }

    async fn parse_json<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, OpenoceanError> {
        let status = resp.status();
        let content_type = resp
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // 先把原始字节读出来；不论成功失败都有“证据”
        let bytes = resp.bytes().await.map_err(OpenoceanError::from)?;

        if !status.is_success() {
            return Err(OpenoceanError::Http {
                status: status.as_u16(),
                body: body_excerpt(&bytes),
                content_type,
            });
        }

        // 使用 serde_path_to_error 捕获精确路径
        let mut de = serde_json::Deserializer::from_slice(&bytes);
        match serde_path_to_error::deserialize::<_, T>(&mut de) {
            Ok(v) => Ok(v),
            Err(err) => {
                let path = err.path().to_string();           // 失败字段的 JSON 路径
                let message = err.inner().to_string();       // 具体错误信息
                Err(OpenoceanError::Parse {
                    message,
                    path,
                    body: body_excerpt(&bytes),
                })
            }
        }
    }

    async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, OpenoceanError> {
        let url = self.build_url(path)?;
        let resp = self.client.get(url).send().await?;
        Self::parse_json(resp).await
    }

    async fn get_json_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T, OpenoceanError>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let url = self.build_url(path)?;
        let resp = self.client.get(url).query(query).send().await?;
        Self::parse_json(resp).await
    }

    // ---------- Public APIs ----------

    pub async fn get_token_list(&self, chain: Chain) -> Result<GetTokenListResponse, OpenoceanError> {
        let path = format!("/v4/{}/tokenList", chain);
        self.get_json::<GetTokenListResponse>(&path).await
    }

    pub async fn get_price(&self, chain: Chain) -> Result<GasResponse, OpenoceanError> {
        let path = format!("/v4/{}/gasPrice", chain);
        self.get_json::<GasResponse>(&path).await
    }

    pub async fn quote(&self, chain: Chain, params: &QuoteParams) -> Result<QuoteResponse, OpenoceanError> {
        let path = format!("/v4/{}/quote", chain);
        let res: QuoteResponse = self.get_json_with_query(&path, params).await?;
        Ok(res)
    }

    pub async fn reverse_quote(&self, chain: Chain, parmas: &ReverseQuoteParams) -> Result<ReverseQuoteResponse, OpenoceanError> {
        let path = format!("/v4/{}/reverseQuote", chain);
        let res: ReverseQuoteResponse = self.get_json_with_query(&path, parmas).await?;
        Ok(res)
    }

    pub async fn swap_quote(&self, chain: Chain, params: &SwapQuoteParams) -> Result<SwapQuoteResponse, OpenoceanError> {
        let path = format!("/v4/{}/swap", chain);
        let res: SwapQuoteResponse = self.get_json_with_query(&path, params).await?;
        Ok(res)
    }

    pub async fn get_dex_list(&self, chain: Chain) -> Result<GetDexListResponse, OpenoceanError> {
        let path = format!("/v4/{}/dexList", chain);
        let res: GetDexListResponse = self.get_json(&path).await?;
        Ok(res)
    }

    pub async fn get_transaction(&self, chain: Chain, hash: String) -> Result<GetTransactionResponse, OpenoceanError> {
        let mut query = HashMap::new();
        query.insert("hash", hash);

        let path = format!("/v4/{}/getTransaction", chain);
        let res: GetTransactionResponse = self.get_json_with_query(&path, &query).await?;
        Ok(res)
    }

    pub async fn decode_input_data(&self, chain: Chain, data: String, method: String) -> Result<DecodeInputDataResponse, OpenoceanError> {
        let mut query = HashMap::new();
        query.insert("data", data);
        query.insert("method", method);

        let path = format!("/v4/{}/decodeInputData", chain);
        let res: DecodeInputDataResponse = self.get_json_with_query(&path, &query).await?;
        Ok(res)
    }

    pub async fn get_gas_price(&self, chain: Chain) -> Result<GasPriceResponse, OpenoceanError> {
        let path = format!("/v4/{}/gasPrice", chain);
        let res: GasPriceResponse = self.get_json(&path).await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::Chain;

    use super::*;

    #[tokio::test]
    async fn test_get_token_list() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let res = client.get_token_list(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("token list: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_price() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let res = client.get_price(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("gas price: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_quote() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let res = client.quote(Chain::Bsc, &QuoteParams {
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
    async fn test_reverse_quote() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let res = client.reverse_quote(Chain::Bsc, &&ReverseQuoteParams {
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
        let res = client.swap_quote(Chain::Bsc, &SwapQuoteParams {
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
        let res = client.get_dex_list(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("dex list: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_transaction() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let res = client.get_transaction(Chain::Bsc, "0x756b98a89714be5c640ea9922aba12e0c94bc30e5a17e111d1aa40373cc24782".to_string()).await.unwrap();
        assert_eq!(res.code, 200);
        println!("transaction: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_decode_input_data() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let res = client.decode_input_data(Chain::Bsc, "000000xxxxxx".to_string(), "swap".to_string()).await.unwrap();
        println!("decode input data: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_gas_price() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let res = client.get_gas_price(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("gas price: {}", serde_json::to_string_pretty(&res).unwrap());

        let res = client.get_gas_price(Chain::Eth).await.unwrap();
        assert_eq!(res.code, 200);
        println!("gas price: {}", serde_json::to_string_pretty(&res).unwrap());
    }
}