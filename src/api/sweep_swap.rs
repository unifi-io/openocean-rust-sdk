use crate::{models::{sweep_swap::{MultiSwapQuoteParams, MultiSwapQuoteResponse}}, Chain, OpenoceanClient, OpenoceanError};




#[derive(Clone)]
pub struct SweepSwap<'a>{
    client: &'a OpenoceanClient,
}

impl<'a> SweepSwap<'a> {
    pub fn new(client: &'a OpenoceanClient) -> Self {
        Self { client }
    }

    pub async fn multi_swap_quote(&self, chain: Chain, params: &MultiSwapQuoteParams) -> Result<MultiSwapQuoteResponse, OpenoceanError> {
        let path = format!("/{}/multi_swap_route", chain);
        self.client.post(&path, params).await
    }
}