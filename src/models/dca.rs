use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::models::gasless::BaseResponse;



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct DcaCreateSwapParams {
    pub maker_amount: String,
    pub signature: String,
    pub order_maker: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub time: i64,
    pub times: i64,
    pub min_price: String,
    pub max_price: String,
    pub referrer: String,
    pub referrer_fee: String,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "enabledDexIds",  skip_serializing_if = "Option::is_none")]
    pub enabled_dex_ids: Option<Vec<i32>>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaCreateSwapResponse {
    pub code: i32,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaCancelOrderParams {
    pub order_hash: String,
    pub signature: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaCancelSwapResponse {    
    pub code: i32,
}


pub type GetDcaOrdersResponse = BaseResponse<Vec<DcaOrder>>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaOrder {
    pub maker_amount: String,
    pub taker_amount: String,
    pub order_hash: String,
    pub create_date_time: String,
    pub order_maker: String,
    pub expire_time: String,
    pub statuses: i32,
    pub time: i64,
    pub times: i64,
    pub have_filled: Option<String>,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub data: DcaOrderData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaOrderData {
    pub maker_asset: String,
    pub maker_asset_symbol: String,
    pub maker_asset_decimals: i32,
    pub maker_asset_icon: String,
    pub taker_asset: String,
    pub taker_asset_symbol: String,
    pub taker_asset_decimals: i32,
    pub taker_asset_icon: String,
}


pub type GetDcaOrderFillsResponse = BaseResponse<Vec<DcaOrderFill>>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaOrderFill {
    pub order_hash: String,
    pub tx_hash: String,
    pub filled_order_time: String,
    pub payment: String,
    pub payment_value: String,
    pub status: String,
    pub reason: String,
}