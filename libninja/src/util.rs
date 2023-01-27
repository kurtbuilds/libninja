use crate::options::OutputOptions;
use crate::{MirSpec, TEMPLATE_DIR};
use convert_case::{Case, Casing};
use openapiv3::{ArrayType, OpenAPI, Schema, SchemaKind};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};
use tera::{Context, Tera};
use url::Host;
pub use ln_model::build_struct;
use ocg_core::fs;

/// Create context for j2 files.
pub fn create_context(opts: &OutputOptions, mir_spec: &MirSpec) -> tera::Context {
    let mut context = Context::new();
    context.insert("package_name", &opts.library_options.package_name);
    context.insert("github_repo", &opts.qualified_github_repo);
    context.insert("package_version", &opts.library_options.package_version);
    context.insert("lang", &opts.library_options.generator.to_string());
    context.insert(
        "short_description",
        &format!(
            "{name} client, generated from the OpenAPI spec.",
            name = opts.library_options.service_name
        ),
    );
    context.insert("env_vars", &mir_spec.env_vars(&opts.library_options));
    if let Some(url) = &mir_spec.api_docs_url {
        context.insert("api_docs_url", url);
    }
    context
}

fn copy_files_recursive(
    dest_path: &Path,
    dir: &include_dir::Dir,
    project_template: &str,
    ignore: &[&str],
) {
    for dir in dir.dirs() {
        let path = dir.path().strip_prefix(project_template).unwrap();
        if ignore.contains(&path.to_str().unwrap()) {
            continue;
        }
        copy_files_recursive(dest_path, dir, project_template, ignore);
    }
    for file in dir.files() {
        if file.path().extension().unwrap_or_default() == "j2" {
            continue;
        }
        let path = file.path().strip_prefix(project_template).unwrap();
        if ignore.contains(&path.to_str().unwrap()) {
            continue;
        }
        let path = dest_path.join(path);
        // Skip if the file already exists.
        if path.exists() {
            continue;
        }
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write_file(&path, file.contents_utf8().unwrap()).unwrap();
    }
}

/// Copy static files to the destination path.
pub fn copy_files(dest_path: &Path, project_template: &str, ignore: &[&str]) -> anyhow::Result<()> {
    copy_files_recursive(
        dest_path,
        TEMPLATE_DIR.get_dir(project_template).unwrap(),
        project_template,
        ignore,
    );
    Ok(())
}

pub fn copy_templates(
    opts: &OutputOptions,
    tera: &tera::Tera,
    context: &tera::Context,
) -> anyhow::Result<()> {
    let project_template = opts.library_options.generator.to_string();
    TEMPLATE_DIR
        .get_dir(&project_template)
        .unwrap()
        .files()
        .filter(|f| f.path().extension().unwrap_or_default() == "j2")
        .for_each(|f| {
            let path = opts.dest_path.join(
                f.path()
                    .strip_prefix(&project_template)
                    .unwrap()
                    .with_extension(""),
            );
            if path.exists() {
                return;
            }
            let content = tera.render(f.path().to_str().unwrap(), context).unwrap();
            fs::write_file(&path, &content).unwrap();
        });
    Ok(())
}

pub fn prepare_templates() -> tera::Tera {
    let mut tera = tera::Tera::default();
    for dir in TEMPLATE_DIR.dirs() {
        for file in dir.files() {
            let path = file.path();
            tera.add_raw_template(path.to_str().unwrap(), file.contents_utf8().unwrap())
                .unwrap();
        }
    }
    tera
}

pub fn indent(s: &str, n: usize) -> String {
    let mut t = String::new();
    for line in s.trim().lines() {
        t.push_str(line);
        t.push('\n');
        for _ in 0..n {
            t.push(' ');
        }
    }
    t.trim_end().to_string()
}

pub fn code_sample(path: &Path) -> Option<String> {
    match path.read_dir() {
        Ok(read_dir) => {
            let mut examples = read_dir
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect::<Vec<_>>();
            examples.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            let first = examples.into_iter().next();
            match first {
                None => None,
                Some(first) => {
                    let code = String::from_utf8(fs::read(first).unwrap()).unwrap();
                    Some(code)
                    // Some(format!("```rust\n{}\n```\n", code))
                }
            }
        }
        Err(_) => None,
    }
}