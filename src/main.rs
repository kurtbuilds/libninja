#![allow(non_snake_case)]
#![allow(unused)]
use anyhow::Result;
use openapi_client_generator::{generate_library, GenerateLibrary};
use clap::{Command, Arg};


fn main() -> Result<()>{
    generate_library(GenerateLibrary {
        name: "Plaid".to_string(),
        yaml_path: "./data/openapi-spec/plaid/2020-09-14.yaml".into(),
        dest_path: "./build/src/".into(),
        lib_rs_path: None,
        model_rs_path: None
    })
}