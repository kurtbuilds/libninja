use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_name = "your client name";
    let language = "your language";
    let response = client
        .link_token_create(client_name, language)
        .webhook("your webhook")
        .access_token("your access token")
        .link_customization_name("your link customization name")
        .redirect_uri("your redirect uri")
        .android_package_name("your android package name")
        .institution_id("your institution id")
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
