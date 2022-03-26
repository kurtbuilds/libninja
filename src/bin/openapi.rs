#![allow(unused)]
use std::fs::File;
use anyhow::Result;
use openapiv3::{MediaType, OpenAPI, PathItem, ReferenceOr, Schema, SchemaKind, Type};
use proc_macro2::TokenStream;
use quote::quote;
use openapi_client_generator::codegen::util::ToToken;


fn main() -> Result<()> {
    // env_logger::init();
    let file = File::open("data/openapi-spec/plaid/2020-09-14.yaml")?;
    // let file = File::open("data/openapi-spec/lob/1.5.0.yaml")?;
    // let file = File::open("data/openapi-spec/smartsheet/2020-02-16.yaml")?;
    // let file = File::open("data/openapi-spec/stripe/2020-08-27.yaml")?;
    // let file = File::open("data/openapi-spec/twilio/1.27/twilio_accounts_v1.yaml")?;
    let spec: OpenAPI = serde_yaml::from_reader(file)?;
    spec.components.as_ref().unwrap().schemas.iter().for_each(|(name, schema)| {
        if name != "ItemGetResponse" {
            return;
        }
        println!("{}", name);
        let schema: &Schema = schema.as_item().as_ref().unwrap();
        schema.properties().unwrap().iter().for_each(|(k, v)| {
            let prop_schema = v.as_ref().resolve(&spec).unwrap();
            println!("property: {}, {:?}", k, prop_schema);
        });
        // println!("{:?}", schema.schema_kind);
        // println!("{:?}", schema);
    });
    // print!("{spec:#?}");
    Ok(())
}