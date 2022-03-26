use std::fmt::{Display, Formatter};
use httpclient::{RequestBuilder};
use httpclient::middleware::RecorderMiddleware;
use serde_json::json;
use serde::{Serialize, Deserialize};

pub struct PlaidClient {
    client: httpclient::Client,
    authentication: Option<PlaidAuthentication>,
}

pub enum PlaidAuthentication {
    ClientSecret { client_id: String, secret: String },
}

pub enum PlaidEnvironment {
    Sandbox,
    Development,
    Production,
}

impl From<&str> for PlaidEnvironment {
    fn from(env: &str) -> Self {
        match env {
            "sandbox" => PlaidEnvironment::Sandbox,
            "development" => PlaidEnvironment::Development,
            "production" => PlaidEnvironment::Production,
            _ => panic!("Invalid Plaid environment: {}", env),
        }
    }
}

impl Display for PlaidEnvironment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaidEnvironment::Sandbox => write!(f, "sandbox"),
            PlaidEnvironment::Development => write!(f, "development"),
            PlaidEnvironment::Production => write!(f, "production"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub item_id: String,
    pub institution_id: String,
    pub webhook: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatus {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemGetResponse {
    pub item: Item,
    pub status: ItemStatus,
    pub request_id: String,
}

trait Authenticatable {
    fn authenticate(self, authenticator: &Option<PlaidAuthentication>) -> Self;
}

impl<'a> Authenticatable for RequestBuilder<'a> {
    fn authenticate(self, authenticator: &Option<PlaidAuthentication>) -> Self {
        if let Some(authenticator) = authenticator {
            match authenticator {
                PlaidAuthentication::ClientSecret { client_id, secret } => {
                    self.push_json(json!({
                        "client_id": client_id,
                        "secret": secret,
                    }))
                }
            }
        } else {
            self
        }
    }
}


impl PlaidClient {
    pub fn with_authentication(environment: PlaidEnvironment, authentication: PlaidAuthentication) -> PlaidClient {
        PlaidClient {
            client: httpclient::Client::new(Some(format!("https://{}.plaid.com", environment))),
            authentication: Some(authentication),
        }
    }

    pub fn record(mut self) -> Self {
        self.client = self.client.with_middleware(RecorderMiddleware::new());
        self
    }

    pub fn from_env() -> Self {
        Self::with_authentication(
            PlaidEnvironment::from(std::env::var("PLAID_ENVIRONMENT").unwrap().as_str()),
            PlaidAuthentication::ClientSecret {
                client_id: std::env::var("PLAID_CLIENT_ID").unwrap(),
                secret: std::env::var("PLAID_SECRET").unwrap(),
            },
        )
    }

    pub async fn item_get(&self, access_token: &str) -> anyhow::Result<ItemGetResponse> {
        let res = self.client.post("/item/get")
            .json(json!({
                "access_token": access_token,
            }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res
                .json()
                .await
                .map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res
                    .text()
                    .await
                    .map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}