use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct OpenoceanBaseResponse<T> {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "data")]
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct OpenoceanToken {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "decimals")]
    pub decimals: u8,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "chain")]
    pub chain: String,
    #[serde(rename = "createtime")]
    pub create_time: String,
    #[serde(rename = "chainId")]
    pub chain_id: Option<i32>,
    #[serde(rename = "customSymbol")]
    pub custom_symbol: Option<String>,
    #[serde(rename = "customAddress")]
    pub custom_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OpenoceanGasPrice {
    #[serde(rename = "standard")]
    pub standard: f64,
    #[serde(rename = "fast")]
    pub fast: f64,
    #[serde(rename = "instant")]
    pub instant: f64,
}

#[derive(Debug, Deserialize)]
pub struct OpenoceanGasResponse {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "data")]
    pub data: OpenoceanGasPrice,
    #[serde(rename = "without_decimals")]
    pub without_decimals: OpenoceanGasPrice,
}