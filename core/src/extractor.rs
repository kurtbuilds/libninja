use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use openapiv3 as oa;
use openapiv3::{APIKeyLocation, OpenAPI, ReferenceOr, RefOr, Schema, SecurityScheme};
use tracing_ez::{debug, span, warn};

use hir::{
    AuthLocation, AuthParam, AuthStrategy, HirSpec, Language, Location, Oauth2Auth, Operation,
    Parameter, Record, TokenAuth,
};
use mir::{Doc, DocFormat, NewType};
use mir::Ty;
pub use record::*;
pub use ty::*;
pub use ty::{schema_ref_to_ty, schema_ref_to_ty2, schema_to_ty};

use crate::extractor::operation::extract_operation;
use crate::sanitize::sanitize;
use crate::util::{is_plural, singular};

mod operation;
mod record;
mod ty;

/// You might need to call add_operation_models after this
pub fn extract_without_treeshake(spec: &OpenAPI) -> Result<HirSpec> {
    let mut hir = HirSpec::default();

    // its important for built in schemas to come before operations, because
    // we do some "create new schema" operations, and if those new ones overwrite
    // the built in ones, that leads to confusion.
    for (name, schema) in &spec.components.schemas {
        let schema = schema.as_item().expect("Expected schema, not reference");
        extract_schema(&name, schema, spec, &mut hir);
    }

    for (path, method, operation, item) in spec.operations() {
        extract_operation(spec, path, method, operation, item, &mut hir);
    }

    let servers = extract_servers(spec)?;
    let security = extract_security_strategies(spec);

    let api_docs_url = extract_api_docs_link(spec);

    hir.servers = servers;
    hir.security = security;
    hir.api_docs_url = api_docs_url;
    Ok(hir)
}

pub fn extract_spec(spec: &OpenAPI) -> Result<HirSpec> {
    let mut hir = extract_without_treeshake(spec)?;
    treeshake(&mut hir);
    validate(&hir);
    debug!(
        "Extracted {} schemas: {:?}",
        hir.schemas.len(),
        hir.schemas.keys()
    );
    Ok(hir)
}

pub fn validate(spec: &HirSpec) {
    for (name, schema) in &spec.schemas {
        if let Record::Struct(s) = schema {
            for (field, schema) in s.fields.iter() {
                if let Ty::Any(Some(inner)) = &schema.ty {
                    warn!(
                        "Field {} in schema {} is an Any with inner: {:?}",
                        field, name, inner
                    );
                } else if let Ty::Model(s) = &schema.ty {
                    if !spec.schemas.contains_key(s) {
                        warn!(
                            "Field {} in schema {} is a model that doesn't exist: {}",
                            field, name, s
                        );
                    }
                }
            }
        }
    }
}

// pub fn deanonymize_array_items(spec: &mut HirSpec, openapi: &OpenAPI) {
//     let current_schemas = spec
//         .schemas
//         .iter()
//         .map(|(name, _)| name.clone())
//         .collect::<HashSet<_>>();
//     let mut new_schemas = vec![];
//     for (name, schema) in spec.schemas.iter_mut() {
//         let Record::Struct(s) = schema else {
//             continue;
//         };
//         for (field, schema) in s.fields.iter_mut() {
//             let Ty::Array(item) = &mut schema.ty else {
//                 continue;
//             };
//             let Ty::Any(Some(inner)) = item.as_mut() else {
//                 continue;
//             };
//             let Some(name) = create_unique_name(&current_schemas, name, field) else {
//                 continue;
//             };
//             let record = create_record(&name, inner, openapi);
//             *item = Box::new(Ty::model(&name));
//             new_schemas.push((name, record));
//         }
//     }
//     spec.schemas.extend(new_schemas);
// }

pub fn is_optional(name: &str, param: &Schema, parent: &Schema) -> bool {
    if param.nullable {
        return true;
    }
    let Some(req) = parent.get_required() else {
        return false;
    };
    !req.iter().any(|s| s == name)
}

fn extract_servers(spec: &OpenAPI) -> Result<BTreeMap<String, String>> {
    let mut servers = BTreeMap::new();
    if spec.servers.len() == 1 {
        let server = spec.servers.first().unwrap();
        servers.insert("default".to_string(), server.url.clone());
        return Ok(servers);
    }
    'outer: for server in &spec.servers {
        for keyword in ["beta", "production", "development", "sandbox"] {
            if matches!(&server.description, Some(desc) if desc.to_lowercase().contains(keyword)) {
                servers.insert(keyword.to_string(), server.url.clone());
                continue 'outer;
            }
        }
        warn!("Server description not recognized. User must pass in server directly. Description: {:?}", server.description);
        return Ok(BTreeMap::new());
    }
    Ok(servers)
}

fn extract_api_docs_link(spec: &OpenAPI) -> Option<String> {
    spec.external_docs.as_ref().map(|e| e.url.clone())
}

/// Remove from the HirSpec anything that appears to be unused
fn remove_unused(spec: &mut HirSpec) {
    let mut used: HashSet<String> = HashSet::new();
    for (_name, schema) in spec.schemas.iter() {
        for field in schema.fields() {
            if let Some(name) = &field.ty.inner_model() {
                used.insert(name.to_string());
            };
        }
    }
    for operation in spec.operations.iter() {
        if let Some(name) = &operation.ret.inner_model() {
            used.insert(name.to_string());
        };
        for param in operation.parameters.iter() {
            if let Some(name) = &param.ty.inner_model() {
                used.insert(name.to_string());
            };
        }
    }
    let count_before = spec.schemas.len();
    spec.schemas.retain(|name, _| {
        let needed = used.contains(name) || name.ends_with("Webhook");
        if !needed {
            debug!("Removing unused schema: {}", name);
        }
        needed
    });
    let count_after = spec.schemas.len();
    if count_before == count_after {
        debug!("No schemas removed in removed_unused");
    }
}

/// effectively performs tree shaking on the spec, strip out models that are unused
fn treeshake(spec: &mut HirSpec) {
    // skip alias structs
    let optional_short_circuit: HashMap<String, String> = spec
        .schemas
        .iter()
        .filter(|(_, r)| r.optional())
        .filter_map(|(_, r)| {
            let Record::TypeAlias(alias, field) = r else {
                return None;
            };
            let Ty::Model(resolved) = &field.ty else {
                return None;
            };
            Some((alias.clone(), resolved.clone()))
        })
        .collect();
    for record in spec.schemas.values_mut() {
        for field in record.fields_mut() {
            let Ty::Model(name) = &field.ty else {
                continue;
            };
            let Some(rename_to) = optional_short_circuit.get(name) else {
                continue;
            };
            field.ty = Ty::model(rename_to);
            field.optional = true;
        }
    }

    // Remove unused models
    remove_unused(spec);
    // Do it twice for 2nd layer of unused models. Super cheap way to remove models
    // that are only unused recursively. E.g. A -> B. A is removed on first pass, B
    // but B isn't. On second pass, B is removed.
    remove_unused(spec);
}

pub fn spec_defines_auth(spec: &HirSpec) -> bool {
    !spec.security.is_empty()
}

fn extract_key_location(loc: &APIKeyLocation, name: &str) -> AuthLocation {
    match loc {
        APIKeyLocation::Header => {
            if ["bearer_auth", "bearer"].contains(&&*name.to_case(Case::Snake)) {
                AuthLocation::Bearer
            } else {
                AuthLocation::Header {
                    key: name.to_string(),
                }
            }
        }
        APIKeyLocation::Query => AuthLocation::Query {
            key: name.to_string(),
        },
        APIKeyLocation::Cookie => AuthLocation::Cookie {
            key: name.to_string(),
        },
    }
}

pub fn extract_security_strategies(spec: &OpenAPI) -> Vec<AuthStrategy> {
    let mut strats = vec![];
    let schemes = &spec.security_schemes;
    for requirement in &spec.security {
        if requirement.is_empty() {
            strats.push(AuthStrategy::NoAuth);
            continue;
        }
        let (scheme_name, _scopes) = requirement.iter().next().unwrap();
        let scheme = schemes
            .get(scheme_name)
            .expect(&format!("Security scheme {} not found.", scheme_name));
        debug!("Found security scheme for {}: {:?}", scheme_name, scheme);
        let scheme = scheme
            .as_item()
            .expect("TODO support refs in securitySchemes");
        match scheme {
            SecurityScheme::APIKey { location, name, .. } => {
                let location = extract_key_location(&location, &name);
                strats.push(AuthStrategy::Token(TokenAuth {
                    name: scheme_name.to_string(),
                    fields: vec![AuthParam {
                        name: name.to_string(),
                        location,
                    }],
                }));
            }
            SecurityScheme::OAuth2 { flows, .. } => {
                if let Some(flow) = &flows.authorization_code {
                    strats.push(AuthStrategy::OAuth2(Oauth2Auth {
                        auth_url: flow.authorization_url.clone(),
                        exchange_url: flow.token_url.clone(),
                        refresh_url: flow
                            .refresh_url
                            .clone()
                            .unwrap_or_else(|| flow.token_url.clone()),
                        scopes: flow
                            .scopes
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect(),
                    }))
                }
            }
            SecurityScheme::HTTP {
                scheme,
                bearer_format,
                description,
            } => {
                strats.push(AuthStrategy::Token(TokenAuth {
                    name: scheme_name.to_string(),
                    fields: vec![AuthParam {
                        name: scheme_name.to_string(),
                        // env_var: scheme_name.to_case(Case::ScreamingSnake),
                        location: AuthLocation::Bearer,
                    }],
                }));
            }
            SecurityScheme::OpenIDConnect { .. } => {}
        }
    }
    debug!("extracted {} security: {:?}", strats.len(), strats);
    strats
}

pub fn extract_newtype(name: &str, schema: &oa::Schema, spec: &OpenAPI) -> NewType<Ty> {
    let ty = schema_to_ty(schema, spec);

    NewType {
        name: name.to_string(),
        doc: None,
        ty,
        public: true,
    }
}

fn get_name(schema_ref: oa::SchemaReference) -> String {
    match schema_ref {
        oa::SchemaReference::Schema { schema } => schema,
        oa::SchemaReference::Property { property, .. } => property,
    }
}

/// Add the models for operations that have structs for their required params.
/// E.g. linkTokenCreate has >3 required params, so it has a struct.
pub fn add_operation_models(lang: Language, mut spec: HirSpec) -> Result<HirSpec> {
    let mut new_schemas = vec![];
    for op in &spec.operations {
        if op.use_required_struct(lang) {
            new_schemas.push((
                op.required_struct_name(),
                Record::Struct(op.required_struct(lang)),
            ));
        }
    }
    spec.schemas.extend(new_schemas);
    Ok(spec)
}
