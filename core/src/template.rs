use tera::Context;

use hir::HirSpec;

use crate::{OutputConfig, PackageConfig, write_file};

pub static TEMPLATE_DIR: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/template");

pub fn copy_builtin_templates(
    opts: &PackageConfig,
    tera: &tera::Tera,
    context: &Context,
) -> anyhow::Result<()> {
    let project_template = opts.language.to_string();
    TEMPLATE_DIR
        .get_dir(&project_template)
        .unwrap()
        .files()
        .filter(|f| f.path().extension().unwrap_or_default() == "j2")
        .for_each(|f| {
            let path = opts.dest.join(
                f.path()
                    .strip_prefix(&project_template)
                    .unwrap()
                    .with_extension(""),
            );
            if path.exists() {
                return;
            }
            let content = tera.render(f.path().to_str().unwrap(), context).unwrap();
            write_file(&path, &content).unwrap();
        });
    Ok(())
}

pub fn add_templates(tera: &mut tera::Tera, dir: &include_dir::Dir<'static>) {
    for dir in dir.dirs() {
        for file in dir.files() {
            let path = file.path();
            tera.add_raw_template(path.to_str().unwrap(), file.contents_utf8().unwrap())
                .unwrap();
        }
    }
}

pub fn prepare_templates() -> tera::Tera {
    let mut tera = tera::Tera::default();
    add_templates(&mut tera, &TEMPLATE_DIR);
    #[cfg(feature = "commercial")]
    add_templates(&mut tera, &ln_commercial::COMMERCIAL_TEMPLATE_DIR);
    tera
}

/// Create context for j2 files.
pub fn create_context(opts: &PackageConfig, spec: &HirSpec) -> Context {
    let mut context = Context::new();
    context.insert("package_name", &opts.package_name);
    context.insert("lang", &opts.language.to_string());
    context.insert(
        "short_description",
        &format!(
            "{name} client, generated from the OpenAPI spec.",
            name = opts.service_name
        ),
    );
    context.insert("env_vars", &spec.env_vars(&opts.service_name));
    if let Some(url) = &spec.api_docs_url {
        context.insert("api_docs_url", url);
    }
    context
}

pub fn get_template_file(path: &str) -> &'static str {
    TEMPLATE_DIR.get_file(path).expect(&format!("{} not found in TEMPLATE_DIR", path)).contents_utf8().unwrap()
}
