use cucumber::then;
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

#[then("the response should contain valid server time")]
async fn check_response_for_valid_server_time(api: &mut CryptoApi) {
    let response_body = api.response_body.as_ref().expect("Response body was not set");
    let response: ServerTimeResponse = serde_json::from_str(response_body).expect("Failed to parse JSON");

    assert!(response.error.is_empty(), "Response contains errors");
    assert!(response.result.unixtime > 0, "Unix time is not valid");
    assert!(!response.result.rfc1123.is_empty(), "RFC1123 time is missing");
}