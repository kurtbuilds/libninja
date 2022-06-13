#![allow(non_snake_case)]
#![allow(unused)]

use std::path::Path;
use anyhow::Result;
use openapi_client_generator::{generate_library, generate_library_at_path, GenerateLibrary};
use clap::{Command, Arg};


fn main() -> Result<()> {
    let matches = Command::new("openapi-client-generator")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("gen")
            .arg(Arg::new("name")
                .long("name")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::new("yaml_spec")
                .required(true)
            )
            .arg(Arg::new("output_dir")
                .long("output-dir")
                .short('o')
                .default_value("src")
                .takes_value(true)
            )
        )
        .get_matches();
    match matches.subcommand().unwrap() {
        ("gen", matches) => {
            let name = matches.get_one::<&str>("name").unwrap().to_string();
            let yaml_spec = matches.get_one::<&str>("yaml_spec").unwrap();
            let output_dir = matches.get_one::<&str>("output_dir").unwrap();

            generate_library_at_path(Path::new(yaml_spec),GenerateLibrary {
                name,
                dest_path: output_dir.into(),
                lib_rs_path: None,
                model_rs_path: None,
            })?;
        }
        _ => panic!("Unknown command"),
    }
    Ok(())
}