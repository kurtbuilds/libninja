use serde_yaml::Value;
use openapiv3::OpenAPI;
use crate::custom;

pub fn modify_spec(value: Value, service_name: &str) -> anyhow::Result<OpenAPI> {
    let spec = match service_name.to_lowercase().as_str() {
        "plaid" => {
            custom::modify_plaid(value)
        }
        "sendgrid" => {
            custom::modify_sendgrid(value)
        }
        "recurly" => {
            custom::modify_recurly(value)
        }
        "openai" => {
            custom::modify_openai(value)
        }
        _ => {
            serde_yaml::from_value(value)?
        }
    };
    Ok(spec)
}
