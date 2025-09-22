use std::time::Duration;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Chain, GetTokenListResponse, OpenoceanError, QuoteParams, QuoteResponse, ReverseQuoteParams, ReverseQuoteResponse};
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
        println!("token list: {:?}", res);
    }

    #[tokio::test]
    async fn test_get_price() {
        let client = OpenoceanClient::new(OpenoceanConfig::default()).unwrap();
        let res = client.get_price(Chain::Bsc).await.unwrap();
        assert_eq!(res.code, 200);
        println!("gas price: {:?}", res);
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
        println!("quote: {:?}", res);
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
        println!("reverse quote: {:?}", res);        
    }
}