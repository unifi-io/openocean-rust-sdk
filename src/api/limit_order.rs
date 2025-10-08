use crate::{models::limit_order::{CancelLimitOrderByAddressResponse, CancelLimitOrderParams, CancelLimitOrderResponse, CreateLimitOrderParams, CreateLimitOrderResponse, GetLimitOrdersByAddressParams}, Chain, OpenoceanClient, OpenoceanError};





#[derive(Clone)]
pub struct LimitOrder<'a>{
    client: &'a OpenoceanClient,
}

impl<'a> LimitOrder<'a> {
    pub fn new(client: &'a OpenoceanClient) -> Self {
        Self { client }
    }

    pub async fn create_limit_order(&self, chain: Chain, params: &CreateLimitOrderParams) -> Result<CreateLimitOrderResponse, OpenoceanError> {
        let path = format!("/v2/{}/limit-order", chain);
        self.client.post(&path, params).await
    }

    pub async fn cancel_limit_order(&self, chain: Chain, params: &CancelLimitOrderParams) -> Result<CancelLimitOrderResponse, OpenoceanError> {
        let path = format!("/v2/{}/limit-order/cancelLimitOrder", chain);
        self.client.post(&path, params).await
    }

    pub async fn get_limit_orders_by_address(&self, chain: Chain, address: String, params: &GetLimitOrdersByAddressParams) -> Result<CancelLimitOrderByAddressResponse, OpenoceanError> {
        let path = format!("/v2/{}/limit-order/address/{}", chain, address);
        self.client.get_json_with_query(&path, params).await
    }
}