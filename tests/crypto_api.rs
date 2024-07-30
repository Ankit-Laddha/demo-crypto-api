use cucumber::World;
use reqwest::StatusCode;

#[derive(Debug, Default, World)]
pub struct CryptoApi {
    pub endpoint: Option<String>,
    pub response_status: Option<StatusCode>,
    pub response_body: Option<String>,
}
