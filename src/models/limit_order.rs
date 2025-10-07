use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::models::base::BaseResponse;




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

pub type CreateLimitOrderResponse = BaseResponse<(())>;