use crate::crypto_api::CryptoApi;
use cucumber::then;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct OpenOrdersResponse {
    error: Vec<String>,
    result: Option<OpenOrders>,
}

#[derive(Deserialize, Debug)]
struct OpenOrders {
    open: HashMap<String, OrderInfo>,
}

#[derive(Deserialize, Debug)]
struct OrderInfo {
    refid: Option<String>,
    userref: Option<i32>,
    status: String,
    opentm: f64,
    starttm: i32,
    expiretm: i32,
    descr: OrderDescription,
    vol: String,
    vol_exec: String,
    cost: String,
    fee: String,
    price: String,
    misc: String,
    oflags: String,
    trades: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct OrderDescription {
    pair: String,
    r#type: String,
    ordertype: String,
    price: String,
    price2: String,
    leverage: String,
    order: String,
    close: String,
}

#[then("the response should contain valid open orders information")]
async fn then_response_contains_valid_open_orders(api: &mut CryptoApi) {
    let response_body = api
        .response_body
        .as_ref()
        .expect("Response body was not set");

    // Parse the response JSON into your response struct
    let response: OpenOrdersResponse =
        serde_json::from_str(response_body).expect("Failed to parse OPEN ORDERS JSON");

    // Assertions to ensure the response is without errors
    assert!(
        response.error.is_empty(),
        "Response contains errors: [{:?}]",
        response.error
    );
}

#[then("the response should contain error about invalid api_key")]
#[then("the response should contain error about invalid api_secret")]
async fn then_response_contains_(api: &mut CryptoApi) {
    let response_body = api
        .response_body
        .as_ref()
        .expect("Response body was not set");

    // Parse the response JSON into your response struct
    let response: OpenOrdersResponse =
        serde_json::from_str(response_body).expect("Failed to parse OPEN ORDERS JSON");

    // Assertions to ensure the response is without errors
    assert!(
        !response.error.is_empty(),
        "Response does not contains errors"
    );
    assert!(
        response.error.contains(&"EAPI:Invalid key".to_string()),
        "Response does not contain the expected error 'EAPI:Invalid key'"
    );
}
