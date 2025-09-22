use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Number;
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};




/// 允许同时接受 JSON string 或 number，并统一转成 String
#[derive(Deserialize)]
#[serde(untagged)]
enum NumOrStr {
    N(Number),
    S(String),
}

fn de_num_or_str_to_string<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let v: NumOrStr = NumOrStr::deserialize(de)?;
    Ok(match v {
        NumOrStr::N(n) => n.to_string(), // e.g. "1.0106346115864016e21"
        NumOrStr::S(s) => s,
    })
}



#[derive(Debug, Deserialize)]
pub struct BaseResponse<T> {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "data")]
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct Token {
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

pub type GetTokenListResponse = BaseResponse<Vec<Token>>;

#[derive(Debug, Deserialize)]
pub struct GasPrice {
    #[serde(rename = "standard")]
    pub standard: f64,
    #[serde(rename = "fast")]
    pub fast: f64,
    #[serde(rename = "instant")]
    pub instant: f64,
}

#[derive(Debug, Deserialize)]
pub struct GasResponse {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "data")]
    pub data: GasPrice,
    #[serde(rename = "without_decimals")]
    pub without_decimals: GasPrice,
}



#[serde_as]

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteParams {
    #[serde(rename = "inTokenAddress")]
    pub in_token_address: String,
    #[serde(rename = "outTokenAddress")]
    pub out_token_address: String,
    #[serde(rename = "amountDecimals")]
    pub amount_decimals: String,
    #[serde(rename = "gasPriceDecimals")]
    pub gas_price_decimals: String,
    #[serde(rename = "slippage")]
    pub slippage: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "enabledDexIds",  skip_serializing_if = "Option::is_none")]
    pub enabled_dex_ids: Option<Vec<i32>>,
}


pub type QuoteResponse = BaseResponse<QuoteData>;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteData {
    pub in_token: QuoteToken,
    pub out_token: QuoteToken,

    pub in_amount: String,
    pub out_amount: String,
    pub estimated_gas: String,

    #[serde(default)]
    pub dexes: Vec<QuoteDex>,

    pub path: QuotePath,

    pub save: f64,
    #[serde(rename = "price_impact")]
    pub price_impact: String, // e.g.: "0.01%"
    pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteToken {
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    pub usd: String,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteDex {
    pub dex_index: i32,
    pub dex_code: String,
    pub swap_amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotePath {
    pub from: String,
    pub to: String,
    pub parts: u32,
    #[serde(default)]
    pub routes: Vec<QuoteRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRoute {
    pub parts: u32,
    pub percentage: f64, // e.g.: 100; may not be an integer, so use f64
    #[serde(default)]
    pub sub_routes: Vec<QuoteSubRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSubRoute {
    pub from: String,
    pub to: String,
    pub parts: u32,
    #[serde(default)]
    pub dexes: Vec<QuoteSubRouteDex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSubRouteDex {
    pub dex: String,
    pub id: String,
    pub parts: u32,
    pub percentage: f64,
}




#[serde_as]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct ReverseQuoteParams {
    pub in_token_address: String,
    pub out_token_address: String,
    pub amount: String,
    pub gas_price: String,
    pub slippage: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "enabledDexIds",  skip_serializing_if = "Option::is_none")]
    pub enabled_dex_ids: Option<Vec<i32>>,
}



pub type ReverseQuoteResponse = BaseResponse<ReverseQuoteData>;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseQuoteData {
    pub in_token: QuoteToken,
    pub out_token: QuoteToken,

    pub in_amount: String,
    pub out_amount: String,
    pub estimated_gas: String,

    #[serde(default)]
    pub dexes: Vec<QuoteDex>,

    pub path: QuotePath,

    pub save: f64,
    #[serde(rename = "price_impact")]
    pub price_impact: String, // e.g.: "0.01%"
    #[serde(rename = "reverseAmount", deserialize_with = "de_num_or_str_to_string")]
    pub reverse_amount: String,
}







#[serde_as]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct SwapQuoteParams {
    pub in_token_address: String,
    pub out_token_address: String,
    pub amount_decimals: String,
    pub gas_price_decimals: String,
    pub slippage: Option<String>,
    pub account: String,
    pub referrer: Option<String>,
    pub referrer_fee: Option<f64>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "enabledDexIds",  skip_serializing_if = "Option::is_none")]
    pub enabled_dex_ids: Option<Vec<i32>>,
    pub sender: Option<String>,
    pub mint_output: Option<u64>,
}



#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapQuoteData {
    pub in_token: QuoteToken,
    pub out_token: QuoteToken,

    pub in_amount: String,
    pub out_amount: String,
    #[serde(deserialize_with = "de_num_or_str_to_string")]
    pub estimated_gas: String,
    pub min_out_amount: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas_price: String,
    pub data: String,
    pub chain_id: i32,
    pub rfq_dealine: Option<i32>,
    pub gmx_fee: i32,
    #[serde(rename = "price_impact")]
    pub price_impact: String,
}

pub type SwapQuoteResponse = BaseResponse<SwapQuoteData>;