#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let processor_token = "your processor token";
    let response = client
        .processor_identity_match(processor_token)
        .user(IdentityMatchUser {
            address: Some(AddressDataNotRequired {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
            }),
            email_address: Some("your email address".to_owned()),
            legal_name: Some("your legal name".to_owned()),
            phone_number: Some("your phone number".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
