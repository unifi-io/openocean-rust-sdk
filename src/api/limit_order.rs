use crate::{models::limit_order::{CreateLimitOrderParams, CreateLimitOrderResponse}, Chain, OpenoceanClient, OpenoceanError};





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
}