use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::models::gasless::BaseResponse;





#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct RouteParams {
    pub dex: String,
    pub pool: String,
    pub position_tick_upper: f64,
    pub position_tick_lower: f64,
    pub tokens: Vec<TokenParam>,
    pub slippage: String,
    pub referrer: Option<String>,
    pub referrer_fee: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct TokenParam {
    pub token: String,
    pub amount: String,
}


pub type RouteResponse = BaseResponse<RouteData>;

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct RouteData {
    pub chain_id: String,
    pub pool_detail: Pool,
    pub zap_details: ZapDetails,
    pub route: String,
    pub route_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    pub pool_id: String,
    pub dex: String,
    pub token0: Token,
    pub token1: Token,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub symbol: String,
    pub name: String,
    pub address: String,
    pub decimals: u8,
    pub price: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct ZapDetails {
    pub initial_amount_usd: f64,
    pub actions: Vec<ZapAction>,
    pub added_liquidity_usd: f64,
    pub zap_impact: f64,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct ZapAction {
    pub r#type: String,
    pub data: ZapActionData,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ZapActionData {
    ProtocalFee(ProtocalFee),
    AggregatorSwap(AggregatorSwap),
    AddLiquidity(AddLiquidity),
} 


#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct ProtocalFee {
    pub address: String,
    pub amount: Token,
    pub amount_usd: String,
    pub zap_fee_rate: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct AggregatorSwap {
    pub token_in: ActionTokenParam,
    pub token_out: ActionTokenParam,
    pub swap_impact: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct ActionTokenParam {
    pub address: String,
    pub amount: String,
    pub amoutn_usd: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct AddLiquidity {
    pub token0: ActionTokenParam,
    pub token1: ActionTokenParam,
    pub liquidity: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct BuildRouteParams {
    pub route: String,
    pub deadline: String,
    pub account: String,
    pub permits: Vec<Permit>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct Permit {
    pub token: String,
    pub permit: String,
}


pub type BuildRouteResponse = BaseResponse<BuildRouteData>;

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct BuildRouteData {
    pub zap_details: ZapDetails,
    pub to: String,
    pub value: String,
    pub data: String,
}