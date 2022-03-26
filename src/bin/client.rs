use openapi_client_generator::handwritten::PlaidClient;

#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env().record();
    let access_token ="access-sandbox-aba1243f-9647-4ae2-ada6-a7f87f0684b0";
    let item_get = client.item_get(access_token)
        .await
        .unwrap();
    println!("{:#?}", item_get);
}