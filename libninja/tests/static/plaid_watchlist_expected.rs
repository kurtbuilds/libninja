#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let search_terms = WatchlistScreeningRequestSearchTerms {
        country: Some("your country".to_owned()),
        date_of_birth: Some(chrono::Utc::now().date_naive()),
        document_number: Some("your document number".to_owned()),
        legal_name: "your legal name".to_owned(),
        watchlist_program_id: "your watchlist program id".to_owned(),
    };
    let response = client
        .watchlist_screening_individual_create(search_terms)
        .client_user_id("your client user id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
