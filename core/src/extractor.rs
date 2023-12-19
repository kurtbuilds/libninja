use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use openapiv3::{OpenAPI, ReferenceOr, Schema};
use openapiv3 as oa;

use ::hir::{AuthLocation, AuthorizationParameter, AuthorizationStrategy, DocFormat, HirSpec, Language, Location, Operation, Record, Ty, Parameter, Doc};
pub use record::*;
pub use resolution::{schema_ref_to_ty, schema_ref_to_ty_already_resolved, schema_to_ty};
pub use resolution::*;
use mir::NewType;
use tracing_ez::{warn, debug, span};

mod resolution;
mod record;

/// You might need to call add_operation_models after this
pub fn extract_spec(spec: &OpenAPI) -> Result<HirSpec> {
    let mut result = HirSpec::default();
    extract_api_operations(spec, &mut result)?;
    extract_records(spec, &mut result)?;
    let servers = extract_servers(spec)?;
    let security = extract_security_strategies(spec);

    let api_docs_url = extract_api_docs_link(spec);

    result.servers = servers;
    result.security = security;
    result.api_docs_url = api_docs_url;
    sanitize_spec(&mut result);
    Ok(result)
}

pub fn is_optional(name: &str, param: &Schema, parent: &Schema) -> bool {
    param.schema_data.nullable || !parent.required(name)
}

pub fn extract_request_schema<'a>(
    operation: &'a oa::Operation,
    spec: &'a OpenAPI,
) -> Result<&'a Schema> {
    let body = operation
        .request_body
        .as_ref()
        .ok_or_else(|| anyhow!("No request body for operation {:?}", operation))?;
    let body = body.resolve(spec)?;
    let content = body
        .content
        .get("application/json")
        .ok_or_else(|| anyhow!("No json body"))?;
    Ok(content.schema.as_ref().expect(&format!("Expecting a ref for {}", operation.operation_id.as_ref().map(|s| s.as_str()).unwrap_or_default())).resolve(spec))
}

pub fn extract_param(param: &ReferenceOr<oa::Parameter>, spec: &OpenAPI) -> Result<Parameter> {
    span!("extract_param", param = ?param);

    let param = param.resolve(spec)?;
    let data = param.parameter_data_ref();
    let param_schema_ref = data
        .schema()
        .ok_or_else(|| anyhow!("No schema for parameter: {:?}", param))?;
    let ty = schema_ref_to_ty(param_schema_ref, spec);
    let schema = param_schema_ref.resolve(spec);
    Ok(Parameter {
        doc: None,
        name: data.name.to_string(),
        optional: !data.required,
        location: param.into(),
        ty,
        example: schema.schema_data.example.clone(),
    })
}

pub fn extract_inputs<'a>(
    operation: &'a oa::Operation,
    item: &'a oa::PathItem,
    spec: &'a OpenAPI,
) -> Result<Vec<Parameter>> {
    let mut inputs = operation
        .parameters
        .iter()
        .map(|param| extract_param(param, spec))
        .collect::<Result<Vec<_>, _>>()?;

    let args = item.parameters.iter().map(|param| extract_param(param, spec)).collect::<Result<Vec<_>, _>>()?;
    for param in args {
        if !inputs.iter().any(|p| p.name == param.name) {
            inputs.push(param);
        }
    }

    let schema = match extract_request_schema(operation, spec) {
        Err(_) => return Ok(inputs),
        Ok(schema) => schema,
    };

    if let oa::SchemaKind::Type(oa::Type::Array(oa::ArrayType { items, .. })) = &schema.schema_kind {
        let ty = if let Some(items) = items {
            schema_ref_to_ty(&items.unbox(), spec)
        } else {
            Ty::Any
        };
        let ty = Ty::Array(Box::new(ty));
        inputs.push(Parameter {
            name: "body".to_string(),
            ty,
            optional: false,
            doc: None,
            location: Location::Body,
            example: schema.schema_data.example.clone(),
        });
    } else if let Ok(props) = schema.properties_iter(spec) {
        let body_args = props.map(|(name, param)| {
            let ty = schema_ref_to_ty(param, spec);
            let param: &Schema = param.resolve(spec);
            let optional = is_optional(name, param, schema);
            let name = name.to_string();
            Parameter {
                name,
                ty,
                optional,
                doc: None,
                location: Location::Body,
                example: schema.schema_data.example.clone(),
            }
        });
        for param in body_args {
            if !inputs.iter().any(|p| p.name == param.name) {
                inputs.push(param);
            }
        }
    } else {
        inputs.push(Parameter {
            name: "body".to_string(),
            ty: Ty::Any,
            optional: false,
            doc: None,
            location: Location::Body,
            example: schema.schema_data.example.clone(),
        });
    }
    Ok(inputs)
}

pub fn extract_response_success<'a>(
    operation: &'a oa::Operation,
    spec: &'a OpenAPI,
) -> Option<&'a ReferenceOr<Schema>> {
    use openapiv3::StatusCode;

    let response = operation
        .responses
        .responses
        .get(&StatusCode::Code(200))
        .or_else(|| operation.responses.responses.get(&StatusCode::Code(201)))
        .or_else(|| operation.responses.responses.get(&StatusCode::Code(202)))
        .or_else(|| operation.responses.responses.get(&StatusCode::Code(204)))
        .or_else(|| operation.responses.responses.get(&StatusCode::Code(302)));
    response?;
    let response = response
        .unwrap_or_else(|| panic!("No success response for operation {:?}", operation))
        .resolve(spec)
        .unwrap();
    response
        .content
        .get("application/json")
        .and_then(|media| media.schema.as_ref())
}

pub fn extract_operation_doc(operation: &oa::Operation, format: DocFormat) -> Option<Doc> {
    let mut doc_pieces = vec![];
    if let Some(summary) = operation.summary.as_ref() {
        if !summary.is_empty() {
            doc_pieces.push(summary.clone());
        }
    }
    if let Some(description) = operation.description.as_ref() {
        if !description.is_empty() {
            if !doc_pieces.is_empty() && description == &doc_pieces[0] {} else {
                doc_pieces.push(description.clone());
            }
        }
    }
    if let Some(external_docs) = operation.external_docs.as_ref() {
        doc_pieces.push(match format {
            DocFormat::Markdown => format!("See endpoint docs at <{}>.", external_docs.url),
            DocFormat::Rst => format!(
                "See endpoint docs at `{url} <{url}>`_.",
                url = external_docs.url
            ),
        })
    }
    if doc_pieces.is_empty() {
        None
    } else {
        Some(Doc(doc_pieces.join("\n\n")))
    }
}

pub fn extract_schema_docs(schema: &Schema) -> Option<Doc> {
    dbg!(schema);
    schema
        .schema_data
        .description
        .as_ref()
        .map(|d| Doc(d.trim().to_string()))
}

pub fn make_name_from_method_and_url(method: &str, url: &str) -> String {
    let names = url
        .split('/')
        .filter(|s| !s.starts_with('{'))
        .collect::<Vec<_>>();
    let last_group = url
        .split('/')
        .filter(|s| s.starts_with('{'))
        .last()
        .map(|s| {
            let mut param = &s[1..s.len() - 1];
            if let Some(name) = names.last() {
                if param.starts_with(name) {
                    param = &param[name.len() + 1..];
                }
            }
            format!("_by_{}", param)
        })
        .unwrap_or_default();
    let name = names.join("_");
    format!("{method}{name}{last_group}")
}

pub fn extract_api_operations(spec: &OpenAPI, result: &mut HirSpec) -> Result<()> {
    for (path, method, operation, item) in spec.operations() {
        let name = match &operation.operation_id {
            None => make_name_from_method_and_url(method, path),
            Some(name) => name.clone(),
        };
        let doc = extract_operation_doc(operation, DocFormat::Markdown);
        let mut parameters = extract_inputs(operation, item, spec)?;
        parameters.sort_by(|a, b| a.name.cmp(&b.name));
        let response_success = extract_response_success(operation, spec);
        let mut needs_response_model = None;
        let ret = match response_success {
            None => Ty::Unit,
            Some(ReferenceOr::Item(s)) => {
                if matches!(s.schema_kind, oa::SchemaKind::Type(oa::Type::Object(_))) {
                    needs_response_model = Some(s);
                    Ty::model(&format!("{}Response", name))
                } else {
                    schema_to_ty(s, spec)
                }
            },
            Some(x @ ReferenceOr::Reference { .. }) => {
                schema_ref_to_ty(x, spec)
            }
        };

        if let Some(s) = needs_response_model {
            let response_name = format!("{}Response", name);
            result.schemas.insert(response_name.clone(), create_record(&response_name, s, spec));
        }
        result.operations.push(Operation {
            name,
            doc,
            parameters,
            ret,
            path: path.to_string(),
            method: method.to_string(),
        });
    }
    Ok(())
}


fn extract_servers(spec: &OpenAPI) -> Result<BTreeMap<String, String>> {
    let mut servers = BTreeMap::new();
    if spec.servers.len() == 1 {
        let server = spec.servers.first().unwrap();
        servers.insert("default".to_string(), server.url.clone());
        return Ok(servers);
    }
    'outer: for server in &spec.servers {
        for keyword in [
            "beta",
            "production",
            "development",
            "sandbox",
        ] {
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

fn sanitize_spec(spec: &mut HirSpec) {
    // skip alias structs
    let optional_short_circuit: HashMap<String, String> = spec.schemas.iter()
        .filter(|(_, r)| r.optional())
        .filter_map(|(_, r)| {
            let Record::TypeAlias(alias, field) = r else { return None; };
            let Ty::Model(resolved) = &field.ty else { return None; };
            Some((alias.clone(), resolved.clone()))
        })
        .collect();
    for record in spec.schemas.values_mut() {
        for field in record.fields_mut() {
            let Ty::Model(name) = &field.ty else { continue; };
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

fn extract_security_fields(_name: &str, requirement: &oa::SecurityRequirement, spec: &OpenAPI) -> Result<Vec<AuthorizationParameter>> {
    use openapiv3::{SecurityScheme, APIKeyLocation};
    let security_schemas = &spec.components.as_ref().unwrap().security_schemes;
    let mut fields = vec![];
    for (name, _scopes) in requirement {
        let schema = security_schemas.get(name).unwrap().as_item().unwrap();
        let location = match schema {
            SecurityScheme::APIKey {
                location,
                name,
                description: _,
            } => match location {
                APIKeyLocation::Header => {
                    if ["bearer_auth", "bearer"].contains(&&*name.to_case(Case::Snake)) {
                        AuthLocation::Bearer
                    } else {
                        AuthLocation::Header {
                            key: name.to_string(),
                        }
                    }
                }
                APIKeyLocation::Query => {
                    AuthLocation::Query {
                        key: name.to_string(),
                    }
                }
                APIKeyLocation::Cookie => {
                    AuthLocation::Cookie {
                        key: name.to_string(),
                    }
                }
            },
            SecurityScheme::HTTP {
                scheme,
                bearer_format: _,
                description: _,
            } => match scheme.as_str() {
                "basic" => AuthLocation::Basic,
                "bearer" => AuthLocation::Bearer,
                "token" => AuthLocation::Token,
                _ => {
                    println!("{:?}", schema);
                    unimplemented!()
                }
            },
            _ => {
                warn!("Skipping authorization for {:?}", schema);
                return Err(anyhow!("Unsupported authorization schema"));
            }
        };

        fields.push(AuthorizationParameter {
            name: name.to_string(),
            env_var: name.to_case(Case::ScreamingSnake),
            location,
        });
    }
    Ok(fields)
}

pub fn extract_security_strategies(spec: &OpenAPI) -> Vec<AuthorizationStrategy> {
    let mut strats = vec![];
    let security = match spec.security.as_ref() {
        None => return strats,
        Some(s) => s,
    };
    for requirement in security {
        let (name, _scopes) = requirement.iter().next().unwrap();
        let fields = match extract_security_fields(name, requirement, spec) {
            Ok(f) => f,
            Err(_e) => {
                continue;
            }
        };
        strats.push(AuthorizationStrategy {
            name: name.clone(),
            fields,
        })
    }
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
        oa::SchemaReference::Property { property, .. } => property
    }
}


/// Add the models for operations that have structs for their required params.
/// E.g. linkTokenCreate has >3 required params, so it has a struct.
pub fn add_operation_models(lang: Language, mut spec: HirSpec) -> Result<HirSpec> {
    let mut new_schemas = vec![];
    for op in &spec.operations {
        if op.use_required_struct(lang) {
            new_schemas.push((op.required_struct_name(), Record::Struct(op.required_struct(lang))));
        }
    }
    spec.schemas.extend(new_schemas);
    Ok(spec)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_operation_name() {
        let method = "get";
        let url = "/diffs/{id}";
        let op_name = make_name_from_method_and_url(method, url);
        assert_eq!(op_name, "get_diffs_by_id");
    }

    #[test]
    fn test_make_operation_name2() {
        let method = "get";
        let url = "/user/{user_id}/account/{account_id}";
        let op_name = make_name_from_method_and_url(method, url);
        assert_eq!(op_name, "get_user_account_by_id");
    }
}
