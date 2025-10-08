use serde::{Deserialize, Serialize};
use serde_with::serde_as;





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct MultiSwapQuoteParams {
    pub in_token: InTokenParams,
    pub out_token: OutTokenParams,
    pub gas_price: f64,
    pub referrer: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>,
    pub account: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct InTokenParams {
    pub in_token_symbol: String,
    pub in_token_address: String,
    pub amount: String,
    pub slippage: u16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct OutTokenParams {
    pub out_token_symbol: String,
    pub out_token_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct MultiSwapQuoteResponse {
    pub in_token: Vec<Token>,
    pub out_token: Token,
    pub from: String,
    pub to: String,
    pub swap: Vec<Swap>,
    pub gas_price: String,
    pub chain_id: String,
    pub value: String,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Token {
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Swap {
    pub in_amount: String,
    pub out_amount: String,
    pub min_out_amount: String,
}