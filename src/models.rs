use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Number;
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};

use crate::types::U128;




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



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub error_msg: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
    pub icon: String,
    pub chain: String,
    pub create_time: String,
    pub chain_id: Option<i32>,
    pub custom_symbol: Option<String>,
    pub custom_address: Option<String>,
}

pub type GetTokenListResponse = BaseResponse<Vec<Token>>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GasPrice {
    pub standard: f64,
    pub fast: f64,
    pub instant: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GasResponse {
    pub code: i32,
    pub data: GasPrice,
    #[serde(rename = "without_decimals")]
    pub without_decimals: GasPrice,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct QuoteParams {
    pub in_token_address: String,
    pub out_token_address: String,
    pub amount_decimals: String,
    pub gas_price_decimals: String,
    pub slippage: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "enabledDexIds",  skip_serializing_if = "Option::is_none")]
    pub enabled_dex_ids: Option<Vec<i32>>,
}


pub type QuoteResponse = BaseResponse<QuoteData>;


#[derive(Debug, Deserialize, Serialize)]
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



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
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


#[derive(Debug, Deserialize, Serialize)]
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






#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
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



#[derive(Debug, Deserialize, Serialize)]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dex {
    pub index: i32,
    pub code: String,
    pub name: String,
}

pub type GetDexListResponse = BaseResponse<Vec<Dex>>;




#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub id: i64,
    pub tx_id: Option<String>,
    pub block_number: u64,
    pub tx_index: u64,
    pub address: String,
    pub tx_hash: String,
    pub tx_hash_url: String,
    pub sender: String,
    pub receiver: String,
    pub in_token_address: String,
    pub in_token_symbol: String,
    pub out_token_address: String,
    pub out_token_symbol: String,
    pub referrer: String,
    pub in_amount: String,
    pub out_amount: String,
    pub fee: Option<String>,
    pub referrer_fee: Option<String>,
    pub usd_valuation: f64,
    pub create_at: String,
    pub update_at: String,
    pub tx_fee: String,
    pub tx_fee_valuation: String,
    pub in_token_decimals: u8,
    pub out_token_decimals: u8,
    pub in_amount_value: String,
    pub out_amount_value: String,
    pub tx_profit: String,
    pub tx_profit_valuation: String,
    pub platform: Option<String>,
    pub status: i32,
}


pub type GetTransactionResponse = BaseResponse<Transaction>;


#[derive(Debug, Deserialize, Serialize)]
pub struct DecodeInputDataResponse {
    pub caller: String,
    pub desc: SwapDesc,
    pub calls: Vec<CallStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapDesc {
    pub src_token: String,
    pub dst_token: String,
    pub src_receiver: String,
    pub dst_receiver: String,

    pub amount: String,
    pub min_return_amount: String,
    pub guaranteed_amount: String,
    pub flags: String,
    pub referrer: String,
    pub permit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallStep {
    pub target: String,
    pub gas_limit: String,
    pub value: String,
    pub data: String,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct GasPriceResponse {
    pub code: u32,
    pub data: GasPriceData,
    // pub without_decimals: GasPriceData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GasPriceData {
    NonEvm(GasPriceDataNonEvm),
    Evm(GasPriceDataEvm),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasPriceDataEvm {
    pub base: f64,
    pub standard: GasPriceTierInt,
    pub fast: GasPriceTierInt,
    pub instant: GasPriceTierInt,
    pub low: GasPriceTierInt,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasPriceTierInt {
    pub legacy_gas_price: U128,
    pub max_priority_fee_per_gas: U128,
    pub max_fee_per_gas: U128,
    pub wait_time_estimate: U128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GasPriceDataNonEvm {
    pub standard: f64,
    pub fast: f64,
    pub instant: f64,
}