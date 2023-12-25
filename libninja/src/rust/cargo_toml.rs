use std::collections::btree_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::process::Output;
use cargo_toml::{Inheritable, Manifest, Package, Dependency, DependencyDetail, DepsSet};
use ln_core::{fs, get_template_file, OutputConfig, PackageConfig};
use crate::rust::Extras;

pub fn update_cargo_toml(extras: &Extras, opts: &OutputConfig, context: &HashMap<String, String>) -> anyhow::Result<String> {
    let cargo = opts.dest_path.join("Cargo.toml");

    let mut m = Manifest::from_path(&cargo).ok().unwrap_or_else(|| {
        let mut m = default_manifest();
        let p = m.package.as_mut().unwrap();
        if let Some(name) = context.get("github_repo") {
            p.set_homepage(Some(name.clone()));
            p.set_repository(Some(name.clone()));
        }
        p.name = opts.package_name.clone();
        p.set_documentation(Some(format!("https://docs.rs/{}", &opts.package_name)));
        let lib = m.lib.as_mut().expect("Cargo.toml must have a lib section");
        lib.doctest = false;
        m
    });
    let package = m.package.as_mut().expect("Cargo.toml must have a package section");

    if let Some(v) = &opts.version {
        package.version = Inheritable::Set(v.clone());
    } else if let Inheritable::Set(t) = &mut package.version {
        if t == "" {
            *t = "0.1.0".to_string();
        } else {
            let mut ver = semver::Version::parse(t).unwrap();
            if ver.major == 0 {
                ver.minor += 1;
                ver.patch = 0;
            } else {
                ver.major += 1;
                ver.minor = 0;
                ver.patch = 0;
            }
            *t = ver.to_string();
        }
    }
    let package_version = package.version().to_string();

    ensure_dependency(&mut m.dependencies, "httpclient", "0.20.2", &[]);
    ensure_dependency(&mut m.dependencies, "serde", "1.0.137", &["derive"]);
    ensure_dependency(&mut m.dependencies, "serde_json", "1.0.81", &[]);
    ensure_dependency(&mut m.dependencies, "futures", "0.3.25", &[]);
    ensure_dependency(&mut m.dependencies, "chrono", "0.4.26", &["serde"]);
    ensure_dependency(&mut m.dev_dependencies, "tokio", "1.18.2", &["full"]);
    if extras.currency {
        ensure_dependency(&mut m.dependencies, "rust_decimal", "1.33.0", &["serde-with-str"]);
        ensure_dependency(&mut m.dependencies, "rust_decimal_macros", "1.33.0", &[]);
    }
    if extras.date_serialization {
        m.dependencies.entry("chrono".to_string())
            .or_insert(Dependency::Detailed(DependencyDetail {
                version: Some("0.4.23".to_string()),
                features: vec!["serde".to_string()],
                default_features: true,
                ..DependencyDetail::default()
            }));
    }
    if opts.config.ormlite {
        ensure_dependency(&mut m.dependencies, "ormlite", "0.16.0", &["decimal"]);
        let d = m.dependencies.get_mut("ormlite").unwrap();
        d.detail_mut().optional = true;
    }
    if opts.config.fake {
        ensure_dependency(&mut m.dependencies, "fake", "2.9", &["derive", "chrono", "rust_decimal", "http", "uuid"]);
        let d = m.dependencies.get_mut("fake").unwrap();
        d.detail_mut().optional = true;
    }
    if extras.basic_auth {
        ensure_dependency(&mut m.dependencies, "base64", "0.21.0", &[]);
    }
    if extras.oauth2 {
        ensure_dependency(&mut m.dependencies, "httpclient_oauth2", "0.1.3", &[]);
    }
    m.example = vec![];
    fs::write_file(&cargo, &toml::to_string(&m).unwrap())?;
    Ok(package_version)
}

fn detailed(version: &str, features: &[&str]) -> Dependency {
    Dependency::Detailed(DependencyDetail {
        version: Some(version.to_string()),
        features: features.iter().map(|f| f.to_string()).collect(),
        default_features: true,
        ..DependencyDetail::default()
    })
}

fn simple(version: &str) -> Dependency {
    Dependency::Simple(version.to_string())
}

fn ensure_dependency(deps: &mut DepsSet, name: &str, version: &str, features: &[&str]) {
    deps.entry(name.to_string())
        .and_modify(|dep| {
            let current_version = dep.req().to_string();
            let mut current_features = dep.req_features().to_vec();
            let version = if version > current_version.as_str() {
                version
            } else {
                &current_version
            };
            if !features.is_empty() {
                let mut features = features.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
                features.retain(|f| !current_features.contains(f));
                current_features.extend(features);
            }
            if current_features.is_empty() {
                *dep = Dependency::Simple(version.to_string());
            } else {
                let detail = dep.detail_mut();
                detail.version = Some(version.to_string());
                detail.features = current_features;
            }
        })
        .or_insert_with(|| if features.is_empty() {
            simple(version)
        } else {
            detailed(version, features)
        });
}

fn default_manifest() -> Manifest {
    let package: Package = serde_json::from_str(r#"{
        "name": "",
        "edition": "2021",
        "readme": "README.md",
        "license": "MIT",
        "version": ""
    }"#).unwrap();
    Manifest {
        package: Some(package),
        workspace: None,
        dependencies: Default::default(),
        dev_dependencies: Default::default(),
        build_dependencies: Default::default(),
        target: Default::default(),
        features: Default::default(),
        replace: Default::default(),
        patch: Default::default(),
        lib: Some(Default::default()),
        profile: Default::default(),
        badges: Default::default(),
        bin: vec![],
        bench: vec![],
        test: vec![],
        example: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// We're really just testing that it doesn't panic, since Default on package doesn't work and
    /// it's non-exhaustive
    #[test]
    fn test_default_manifest() {
        let manifest = default_manifest();
    }
}