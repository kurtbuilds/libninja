use serde::Serialize;
use anyhow::anyhow;
use crate::command::Success;

pub async fn delete_repo(repo: &str) -> anyhow::Result<()> {
    println!("{}: Deleting repo...", repo);
    let mut child = tokio::process::Command::new("gh")
        .arg("repo")
        .arg("delete")
        .arg(repo)
        .arg("--confirm")
        .spawn()?;
    child.wait().await?.ok()
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CreateEnvironment {
    pub repo: String,
    pub gh_token: String,
    pub homepage: String,
    pub service: String,
    pub dir: String,
    pub tags: String,
}


pub async fn create_repo(env: CreateEnvironment) -> anyhow::Result<()> {
    if !env.repo.contains('/') {
      return Err(anyhow!("When you create env, the repo must be qualified, i.e. `owner/repo`"));
    }
    let script = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/script/create.sh"));
    let dir = env.dir.clone();
    println!("{}: Creating repo...", &env.repo);
    let env = serde_json::to_value(env)?;
    let vars = env.as_object().unwrap().into_iter().map(|(k, v)| {
        (k.to_string(), v.as_str().unwrap().to_string())
    });
    let mut child = tokio::process::Command::new("bash")
        .current_dir(dir)
        .envs(vars)
        .arg("-c")
        .arg(script)
        .spawn()?;
    child.wait().await?.ok()
}


pub async fn push_repo(env: PushEnvironment) -> anyhow::Result<()> {
    println!("{}: Start pushing repo...", &env.repo);
    let script = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/script/push.sh"));
    let dir = env.dir.clone();
    println!("{}: Pushing repo...", &env.repo);
    let env = serde_json::to_value(env)?;
    let vars = env.as_object().unwrap().into_iter().map(|(k, v)| {
        (k.to_string(), v.as_str().unwrap().to_string())
    });
    println!("About to run push.sh...");
    let mut child = tokio::process::Command::new("bash")
        .current_dir(dir)
        .envs(vars)
        .arg("-c")
        .arg(script)
        .spawn()?;
    println!("push.sh launched at least...");
    child.wait().await?.ok()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PushEnvironment {
    pub repo: String,
    pub gh_token: String,
    pub dir: String,
    pub version: String,
}
