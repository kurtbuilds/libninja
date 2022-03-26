use plaid_openapi::PlaidClient;
use plaid_openapi::PlaidAuthentication;
use httpclient::middleware::{RecorderMiddleware, LoggerMiddleware, RecorderMode};

#[tokio::main]
async fn main() {
    let client = PlaidClient::new("https://sandbox.plaid.com/")
        .with_authentication(PlaidAuthentication::from_env())
        .with_middleware(LoggerMiddleware::new())
        .with_middleware(RecorderMiddleware::with_mode(RecorderMode::IgnoreRecordings));
    let access_token ="access-sandbox-aba1243f-9647-4ae2-ada6-a7f87f0684b0";
    let item_get = client.item_get(access_token.to_string())
        .await;
    println!("{:#?}", item_get);
}