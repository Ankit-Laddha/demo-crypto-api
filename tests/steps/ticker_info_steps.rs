use crate::crypto_api::CryptoApi;
use cucumber::then;
use serde::Deserialize;

#[derive(Deserialize)]
struct TickerResponse {
    error: Vec<String>,
    result: std::collections::HashMap<String, TickerInfo>,
}

#[derive(Deserialize)]
struct TickerInfo {
    a: Vec<String>, // Ask
    b: Vec<String>, // Bid
    c: Vec<String>, // Last trade closed
    v: Vec<String>, // Volume
    p: Vec<String>, // Volume weighted average price
    t: Vec<u64>,    // Number of trades
    l: Vec<String>, // Low
    h: Vec<String>, // High
    o: String,      // Today's opening price
}

#[then(regex = r#"the response should contain "(.*)" trading pair information"#)]
async fn check_response_for_valid_trading_pair_information(
    api: &mut CryptoApi,
    trading_pair: String,
) {
    let response_body = api
        .response_body
        .as_ref()
        .expect("Response body was not set");
    let response: TickerResponse =
        serde_json::from_str(response_body).expect("Failed to parse JSON");

    assert!(response.error.is_empty(), "Response contains errors");
    assert!(
        response.result.contains_key(&trading_pair),
        "Trading pair information missing"
    );
}
