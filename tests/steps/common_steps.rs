use std::env;
use cucumber::{given, then, when};
use reqwest::StatusCode;
use serde::Deserialize;
use crate::crypto_api::CryptoApi;


#[given(regex = r#"the Server API endpoint "(.*)""#)]
async fn given_the_server_api_endpoint(api: &mut CryptoApi, path: String) {
    let base_url = env::var("BASE_URL").expect("BASE_URL not set in .env file");
    let full_url = format!("{}{}", base_url, path);
    api.endpoint = Some(full_url);
}

async fn send_get_request(endpoint: &str) -> Result<(StatusCode, String), reqwest::Error> {
    let response = reqwest::get(endpoint).await?;
    let status = response.status();
    let body = response.text().await?;
    Ok((status, body))
}

#[when("I send a GET request to the endpoint")]
async fn when_i_send_a_get_request(api: &mut CryptoApi) {
    let endpoint = api.endpoint.as_ref().expect("Endpoint was not set");
    let (status, body) = send_get_request(endpoint).await.expect("Failed to send GET request");
    api.response_status = Some(status);
    api.response_body = Some(body);
}

#[then(regex = r#"the response status should be (\d+)"#)]
async fn then_the_response_status_should_be(api: &mut CryptoApi, expected_status: u16) {
    let actual_status = api.response_status.expect("Response status was not set");
    assert_eq!(actual_status, StatusCode::from_u16(expected_status).expect("Invalid status code"), "The response status does not match");
}