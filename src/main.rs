#![allow(non_snake_case)]
use std::fmt::format;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use anyhow::Result;
use convert_case::{Case, Casing};
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, SecurityRequirement};
use proc_macro2::TokenStream;
use quote::quote;
use openapi_client_generator::codegen::client::{impl_Authenticatable, impl_ServiceAuthentication, impl_ServiceClient, struct_ServiceAuthentication, struct_ServiceClient};
use openapi_client_generator::codegen::model::all_struct_Schema;
use openapi_client_generator::format_code;


fn main() -> Result<()>{
    let file = File::open("data/openapi-spec/plaid/2020-09-14.yaml")?;
    let spec: OpenAPI = serde_yaml::from_reader(file)?;
    // spec.security.unwrap().iter().for_each(|s: SecurityRequirement| {
    //     s.iter().for_each(|(k, v)| {
    //         println!("{} {}", k, v);
    //     })
    //
    //     println!("{:?}", s);
    // });
    let struct_ServiceClient = struct_ServiceClient("Plaid");
    let struct_ServiceAuthentication = struct_ServiceAuthentication("Plaid", &spec);
    let impl_ServiceClient = impl_ServiceClient("Plaid", &spec);
    let impl_ServiceAuthentication = impl_ServiceAuthentication("Plaid", &spec);
    let impl_Authenticatable = impl_Authenticatable("Plaid", &spec);

    let all_struct_Schema = all_struct_Schema(&spec);

    let tok = quote! {
        #struct_ServiceClient
        #impl_ServiceClient
        #struct_ServiceAuthentication
        #impl_ServiceAuthentication
        #impl_Authenticatable

        #all_struct_Schema
    };
    let code = format_code(tok).unwrap();
    let mut f =  OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("gen/plaid/src/lib.rs")?;
    let template = fs::read_to_string("src/codegen/template/lib.rs")?;
    f.write(template.as_bytes())?;
    f.write("\n".as_bytes())?;
    f.write(code.as_bytes())?;
    Ok(())
}