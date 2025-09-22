use std::time::Duration;
use serde::de::DeserializeOwned;

use crate::OpenoceanError;
use reqwest::{Client, Url};
use std::fmt;
use crate::{
    OpenoceanBaseResponse, OpenoceanToken, OpenoceanGasResponse,
};

// https://apis.openocean.finance/developer/widget/widget-v2

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


#[derive(Clone, Copy, Debug)]
pub enum Chain {
    Eth,
    Bsc,
    Arbitrum,
    Polygon,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Chain::Eth => "eth",
            Chain::Bsc => "bsc",
            Chain::Arbitrum => "arbitrum",
            Chain::Polygon => "polygon",
        };
        f.write_str(s)
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

    async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, OpenoceanError> {
        let mut url = self.config.base_url.clone();
        let mut p = url.path().trim_end_matches('/').to_string();
        p.push_str(path);
        url.set_path(&p);
        {
            let mut qp = url.query_pairs_mut();
            for (k, v) in query {
                qp.append_pair(k, v);
            }
        }
    
        let resp = self.client.get(url).send().await?;
    
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(OpenoceanError::Http { status, body });
        }
    
        Ok(resp.json::<T>().await?)
    }

    pub async fn get_token_list(&self, chain: Chain) -> Result<OpenoceanBaseResponse<Vec<OpenoceanToken>>, OpenoceanError> {
        let path = format!("/v4/{}/tokenList", chain);
        self.get::<OpenoceanBaseResponse<Vec<OpenoceanToken>>>(&path, &[]).await
    }

    pub async fn get_price(&self, chain: Chain) -> Result<OpenoceanGasResponse, OpenoceanError> {
        let path = format!("/v4/{}/gasPrice", chain);
        self.get::<OpenoceanGasResponse>(&path, &[]).await
    }
}

#[cfg(test)]
mod tests {
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
}