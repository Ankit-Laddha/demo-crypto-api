use std::env;

use cucumber::World;
use dotenv::dotenv;

use crate::crypto_api::CryptoApi;

mod crypto_api;
mod steps;

#[tokio::main]
async fn main() {
    // Initialize the logger
    dotenv().ok();

    // Get the tags from the command line arguments
    let args: Vec<String> = env::args().collect();
    let tags: Vec<String> = args
        .iter()
        .filter_map(|arg| {
            if arg.starts_with("--tags=") {
                Some(arg[7..].to_string())
            } else {
                None
            }
        })
        .collect();

    if tags.is_empty() {
        // Run all scenarios if no tags are provided
        CryptoApi::cucumber()
            .with_default_cli()
            .run_and_exit("tests/features")
            .await;
    } else {
        // Filter scenarios by tags
        CryptoApi::cucumber()
            .filter_run("tests/features", move |_, _, sc| {
                sc.tags.iter().any(|t| tags.contains(t))
            })
            .await;
    }
}
