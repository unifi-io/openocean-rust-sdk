use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::models::gasless::BaseResponse;





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct SubmitTicketParams {
    pub hash: String,
    pub chain: String,
    pub version: String,
    pub question: String,
    pub account: String,
    pub quote: Quote,
    pub transaction: TransactionIn,
    pub error: ErrorIn,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct ErrorIn {
    pub code: i32,
    pub error: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct TransactionIn {
    pub from: String,
    pub to: String,
    pub value: String,
    pub data: String,
    pub gas_price: String,
    pub gas_limit: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Quote {
    pub quote_type: String,
    pub in_token_symbol: String,
    pub in_token_address: String,
    pub out_token_symbol: String,
    pub out_token_address: String,
    pub amount_all: i32,
    pub amount: String,
    pub gas_price: String,
    pub slippage: i32,
    pub referrer: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>,
}

pub type SubmitTicketResponse = BaseResponse<SubmitTicketData>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct SubmitTicketData {
    pub ticket: String,
}

pub type GetTicketResponse = BaseResponse<GetTicketData>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetTicketData {
    pub hash: String,
    pub remark: String,
    pub process: String,
    pub question: String,
    pub answer: String,
    pub params: TicketParams,
    pub account: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct TicketParams {
    pub quote: Quote,
}