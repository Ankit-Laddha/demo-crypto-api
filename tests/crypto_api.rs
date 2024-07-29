use cucumber::World;
use reqwest::{Client, StatusCode, header};

#[derive(Debug, Default, World)]
pub struct CryptoApi {
    pub endpoint: Option<String>,
    pub response_status: Option<StatusCode>,
    pub response_body: Option<String>,
    pub client: Option<Client>,
    pub headers: Option<header::HeaderMap>,
}