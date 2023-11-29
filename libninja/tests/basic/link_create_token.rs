use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_name = "your client name";
    let language = "your language";
    let response = client
        .link_token_create(client_name, language)
        .access_token("your access token")
        .android_package_name("your android package name")
        .institution_id("your institution id")
        .link_customization_name("your link customization name")
        .redirect_uri("your redirect uri")
        .user_token("your user token")
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}