use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{models::swap::QuoteToken, Chain};



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: Option<String>,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct GaslessQuoteParams {
    pub chain: String,
    pub in_token_address: String,
    pub out_token_address: String,
    pub amount_decimals: String,
    pub gas_price_decimals: String,
    pub slippage: Option<String>,
    pub referrer: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct QuoteFee {
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    pub usd: String,
    pub in_fee_amount: f64,
    pub volume: f64,
}


pub type GaslessQuoteResponse = BaseResponse<GaslessQuoteData>;

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct GaslessQuoteData {
    pub in_token: QuoteToken,
    pub out_token: QuoteToken,
    pub native: QuoteToken,
    pub fees: Vec<QuoteFee>,
    pub flag: i32,
    pub in_amount: String,
    pub out_amount: String,
    pub estimated_gas: i64,
    pub path: QuotePath,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub fee: Option<f64>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GaslessSwapParams {
    pub from: String,
    pub to: String,
    pub data: String,
    pub amount_decimals: String,
    pub fee_amount1: String,
    pub fee_amount2: String,
    pub flag: i32,
    pub gas_price_decimals: i32,
    pub deadline: i64,
    pub in_token: String,
    pub out_token: String,
    pub nonce: i64,
    pub permit: String,
    #[serde(rename = "usdvaluation")]
    pub usdvaluation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GaslessSwapResponse {
    pub code: i32,
    pub msg: Option<String>,
    pub order_hash: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderStatusParams {
    pub chain: Chain,
    pub order_hash: String,
}


pub type GetOrderStatusResponse = BaseResponse<GetOrderStatusData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderStatusData {
    pub hash: String,
    pub err: String,
}