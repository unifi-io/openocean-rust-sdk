use crate::{models::ticket::{GetTicketResponse, SubmitTicketParams, SubmitTicketResponse}, OpenoceanClient, OpenoceanError};




#[derive(Clone)]
pub struct Ticket<'a>{
    client: &'a OpenoceanClient,
}

impl<'a> Ticket<'a> {
    pub fn new(client: &'a OpenoceanClient) -> Self {
        Self { client }
    }

    pub async fn submit(&self, referer: String, params: &SubmitTicketParams) -> Result<SubmitTicketResponse, OpenoceanError> {
        let path = format!("/{}/ticket", referer);
        self.client.post(&path, params).await
    }

    pub async fn get_ticket(
        &self,
        referer: String,
    ) -> Result<GetTicketResponse, OpenoceanError> {
        let path = format!("/{}/ticket", referer);
        self.client.get_json(&path).await
    }
}