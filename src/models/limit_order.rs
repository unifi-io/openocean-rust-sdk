use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{models::base::BaseResponse};




#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CreateLimitOrderParams {
    pub taker_asset: String,
    pub maker_asset: String,
    pub expire_time: String,
    pub order_maker: String,
    pub signature: String,
    pub taker_amount: String,
    pub maker_amount: String,
    pub referrer: Option<String>,
    pub referrer_fee: Option<String>,

    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "enabledDexIds",  skip_serializing_if = "Option::is_none")]
    pub enabled_dex_ids: Option<Vec<i32>>,

    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, i32>>")]
    #[serde(rename = "disabledDexIds", skip_serializing_if = "Option::is_none")]
    pub disabled_dex_ids: Option<Vec<i32>>,
}

pub type CreateLimitOrderResponse = BaseResponse<()>;



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CancelLimitOrderParams {
    pub order_hash: String,
    pub signature: String,
}

pub type CancelLimitOrderResponse = BaseResponse<()>;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetLimitOrdersByAddressParams {
    pub statuses: String,
    pub limit: i32,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    pub maker_amount: String,
    pub taker_amount: String,
    pub signature: String,
    pub order_hash: String,
    pub create_date_time: String,
    pub order_maker: String,
    pub remaining_maker_amount: String,
    pub expire_time: String,
    pub statuses: i32,
    pub data: LimitOrderData,
    pub maker_rate: String,
    pub taker_rate: String,
    pub referrer: Option<String>,
    pub referrer_fee: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderData {
    pub maker_asset: String,
    pub maker_asset_symbol: String,
    pub maker_asset_decimals: i32,
    pub maker_asset_icon: String,
    pub taker_asset: String,
    pub taker_asset_symbol: String,
    pub taker_asset_decimals: i32,
    pub taker_asset_icon: String,
    pub get_maker_amount: String,
    pub get_taker_amount: String,
    pub maker_asset_data: String,
    pub taker_asset_data: String,
    pub salt: String,
    pub permit: String,
    pub predicate: String,
    pub interaction: String,
    pub making_amount: String,
    pub taking_amount: String,
    pub maker: String,
    pub receiver: String,
    pub allowed_sender: String,
}


pub type CancelLimitOrderByAddressResponse = BaseResponse<Vec<LimitOrder>>;