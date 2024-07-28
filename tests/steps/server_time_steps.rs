use cucumber::{given, then, when};
use reqwest::StatusCode;
use serde::Deserialize;
use crate::crypto_api::CryptoApi;

#[derive(Deserialize, Debug)]
struct ServerTimeResponse {
    error: Vec<String>,
    result: ServerTime,
}

#[derive(Deserialize, Debug)]
struct ServerTime {
    unixtime: u64,
    rfc1123: String,
}

#[given(regex = r#"the Server API endpoint "(.*)""#)]
async fn given_the_server_api_endpoint(api: &mut CryptoApi, endpoint: String) {
    api.endpoint = Some(endpoint);
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

#[then("the response should contain valid server time")]
async fn check_response_for_valid_server_time(api: &mut CryptoApi) {
    let response_body = api.response_body.as_ref().expect("Response body was not set");
    let response: ServerTimeResponse = serde_json::from_str(response_body).expect("Failed to parse JSON");

    assert!(response.error.is_empty(), "Response contains errors");
    assert!(response.result.unixtime > 0, "Unix time is not valid");
    assert!(!response.result.rfc1123.is_empty(), "RFC1123 time is missing");
}