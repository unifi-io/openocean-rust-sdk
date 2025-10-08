use crate::{models::zap::{BuildRouteParams, BuildRouteResponse, RouteParams, RouteResponse}, Chain, OpenoceanClient, OpenoceanError};




#[derive(Clone)]
pub struct Zap<'a>{
    client: &'a OpenoceanClient,
}

impl<'a> Zap<'a> {
    pub fn new(client: &'a OpenoceanClient) -> Self {
        Self { client }
    }

    pub async fn route(&self, chain: Chain, params: &RouteParams) -> Result<RouteResponse, OpenoceanError> {
        let path = format!("/zap/{}/in/route", chain);
        self.client.post(&path, params).await
    }

    pub async fn build_route(&self, chain: Chain, params: &BuildRouteParams) -> Result<BuildRouteResponse, OpenoceanError> {
        let path = format!("/zap/{}/in/route/build", chain);
        self.client.post(&path, params).await
    }
}