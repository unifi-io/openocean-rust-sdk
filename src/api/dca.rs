use crate::{models::dca::{DcaCancelOrderParams, DcaCancelSwapResponse, DcaCreateSwapParams, DcaCreateSwapResponse, GetDcaOrderFillsResponse, GetDcaOrdersResponse}, Chain, OpenoceanClient, OpenoceanError};






#[derive(Clone)]
pub struct Dca<'a>{
    client: &'a OpenoceanClient,
}


impl<'a> Dca<'a> {
    pub fn new(client: &'a OpenoceanClient) -> Self {
        Self { client }
    }

    pub async fn create_dca_order(&self, chain: Chain, params: &DcaCreateSwapParams) -> Result<DcaCreateSwapResponse, OpenoceanError> {
        let path = format!("/v2/{}/dca/swap", chain);
        self.client.post(&path, params).await
    }

    pub async fn cancel_dca_order(&self, chain: Chain, params: &DcaCancelOrderParams) -> Result<DcaCancelSwapResponse, OpenoceanError> {
        let path = format!("/v2/{}/dca/cancel", chain);
        self.client.post(&path, params).await
    }

    pub async fn get_dca_orders(&self, chain: Chain, address: String) -> Result<GetDcaOrdersResponse, OpenoceanError> {
        let path = format!("/v2/{}/dca/address/{}", chain, address);
        self.client.get_json(&path).await
    }

    pub async fn get_dca_order_fills(&self, chain: Chain, order_hash: String) -> Result<GetDcaOrderFillsResponse, OpenoceanError> {
        let path = format!("/v2/{}/dca/fill/{}", chain, order_hash);
        self.client.get_json(&path).await
    }
}