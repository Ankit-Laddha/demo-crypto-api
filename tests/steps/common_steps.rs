use crate::crypto_api::CryptoApi;
use base64::encode as base64_encode;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::{decode as base64_decode, Engine};
use cucumber::{given, then, when};
use hmac::digest::Digest;
use hmac::{Hmac, KeyInit, Mac};
use reqwest::{header, StatusCode};
use serde::Deserialize;
use sha2::{Sha256, Sha512};
use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;

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
    let (status, body) = send_get_request(endpoint)
        .await
        .expect("Failed to send GET request");
    api.response_status = Some(status);
    api.response_body = Some(body);
}

#[then(regex = r#"the response status should be (\d+)"#)]
async fn then_the_response_status_should_be(api: &mut CryptoApi, expected_status: u16) {
    let actual_status = api.response_status.expect("Response status was not set");
    assert_eq!(
        actual_status,
        StatusCode::from_u16(expected_status).expect("Invalid status code"),
        "The response status does not match"
    );
}

#[when("a POST request is sent to the endpoint with valid credentials")]
async fn post_request_with_valid_credentials(api: &mut CryptoApi) {
    let url = api.endpoint.as_ref().expect("Endpoint was not set");

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string();

    // Create the required body parameters
    let mut body = HashMap::new();
    body.insert("nonce", nonce.clone());
    body.insert("trades", "false".to_string());

    // Create HashMap with references to the `String` values
    let body_refs: HashMap<&str, &String> = body.iter().map(|(k, v)| (*k, v)).collect();

    let signature =
        generate_kraken_signature("/0/private/OpenOrders", &nonce, &body_refs);

    let post_body = format!("nonce={}&trades=false", nonce);
    let (status, body) = send_post_request(&url, &signature, &post_body)
        .await
        .expect("Failed to send POST request");
    println!("LOG: {}", body);
    api.response_status = Some(status);
    api.response_body = Some(body);
}

async fn send_post_request(
    url: &str,
    signature: &str,
    body: &str,
) -> Result<(StatusCode, String), reqwest::Error> {
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("API-Key", api_key)
        .header("API-Sign", signature)
        .header(
            header::CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=utf-8",
        )
        .body(body.to_string())
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;
    assert!(!body.is_empty(), "Response body should not be empty");
    println!("Response: {}", body);
    Ok((status, body))
}

fn generate_kraken_signature(
    uri_path: &str,
    nonce: &str,
    data: &HashMap<&str, &String>,
) -> String {
    // Create post data in the form "key=value&key=value"
    let mut postdata = String::new();
    for (key, value) in data.iter() {
        if !postdata.is_empty() {
            postdata.push('&');
        }
        write!(&mut postdata, "{}={}", key, value).expect("Error formatting data");
    }

    // Combine nonce and post data
    let encoded = format!("{}{}", nonce, postdata);
    let encoded_hash = Sha256::digest(encoded.as_bytes());

    // Create HMAC message
    let mut message = Vec::new();
    message.extend_from_slice(uri_path.as_bytes());
    message.extend_from_slice(&encoded_hash);

    let api_secret = env::var("API_SECRET").expect("API_SECRET not set in .env file");
    // Decode secret from base64
    let decoded_secret = STANDARD.decode(api_secret).expect("Invalid API secret");

    // Create HMAC-SHA512
    let mut mac =
        HmacSha512::new_from_slice(&decoded_secret).expect("HMAC can accept key of any size");
    mac.update(&message);

    // Base64 encode the signature
    let signature = STANDARD.encode(mac.finalize().into_bytes());

    signature
}
