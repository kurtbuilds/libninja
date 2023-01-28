#![allow(unused)]
use ocg::CreateEnvironment;
use anyhow::Result;
use std::path::Path;

const POSTMAN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/integration/full_lifecycle/postman.yaml");


#[tokio::test]
#[cfg(feature = "integration")]
async fn test_lifecycle() -> Result<()> {
    let test_repo_name = "test-postman-rs";
    let repo = format!("libninjacom/{test_repo_name}");
    let res = (|| async {
        let token = env!("GH_TOKEN").to_string();

        let temp = tempfile::tempdir()?;
        let env = CreateEnvironment {
            repo: repo.clone(),
            gh_token: token.clone(),
            homepage: "https://docs.rs/postman/".to_string(),
            service: "Postman".to_string(),
            dir: temp.path().display().to_string(),
            tags: "openapi,rust,postman".to_string(),
        };
        ::ocg::create_repo(env.clone()).await?;

        std::fs::remove_dir_all(temp.path().join(test_repo_name))?;
        // Create it again because this is effectively idempotent.
        ::ocg::create_repo(env.clone()).await?;

        eprintln!("Generating library...");
        ::ocg::generate_library_using_spec_at_path(&Path::new(POSTMAN), ::ocg::OutputOptions {
            library_options: ::ocg::LibraryOptions {
                package_name: "postman".to_string(),
                service_name: "Postman".to_string(),
                language: ::ocg::Language::Rust,
                package_version: "0.1.0".to_string(),
                config: ::ocg::LibraryConfig::default(),
            },
            qualified_github_repo: env.repo,
            dest_path: temp.path().join(test_repo_name),
        })?;

        println!("Pushing repo...");
        ::ocg::push_repo(::ocg::PushEnvironment {
            repo: test_repo_name.to_string(),
            gh_token: token.clone(),
            dir: temp.path().display().to_string(),
            version: "0.1.0".to_string(),
        }).await?;
        Ok(())
    })().await;

    ocg::repo::delete_repo(&repo).await?;
    res
}