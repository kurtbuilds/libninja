use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use openapiv3::{ArrayType, OpenAPI, Operation, Parameter, PathItem, RefOr, ReferenceOr, Schema, SchemaKind, Type};
use tracing_ez::span;

use hir::{HirSpec, Location};
use mir::{Doc, DocFormat, Ty};

use crate::extractor;
use crate::extractor::record::extract_schema;
use crate::extractor::{is_primitive, schema_ref_to_ty, schema_ref_to_ty2, schema_to_ty};

pub fn extract_operation(spec: &OpenAPI, path: &str, method: &str, op: &Operation, item: &PathItem, hir: &mut HirSpec) {
    let name = make_name(op.operation_id.as_ref(), method, path);
    let doc = extract_doc(op, DocFormat::Markdown);
    let mut parameters = extract_parameters(op, item, spec).unwrap();
    parameters.sort_by(|a, b| a.name.cmp(&b.name));
    let ret = match get_res(op, spec) {
        None => Ty::Unit,
        Some(x @ ReferenceOr::Reference { .. }) => schema_ref_to_ty(x, spec),
        Some(ReferenceOr::Item(res)) => {
            let name = format!("{}Response", name.to_case(Case::Pascal));
            extract_schema(&name, res, spec, hir);
            if is_primitive(res, spec) {
                schema_to_ty(res, spec)
            } else if matches!(res.kind, SchemaKind::Type(Type::Array(_))) {
                schema_to_ty(res, spec)
            } else {
                Ty::Model(name)
            }
        }
    };
    hir.operations.push(hir::Operation {
        name: name.to_case(Case::Pascal),
        doc,
        parameters,
        ret,
        path: path.to_string(),
        method: method.to_string(),
    });
}

/// make a name for hir::Operation
fn make_name(operation_id: Option<&String>, method: &str, path: &str) -> String {
    if let Some(name) = operation_id {
        return name.replace(".", "_");
    }
    let names = path.split('/').filter(|s| !s.starts_with('{')).collect::<Vec<_>>();
    let last_group = path
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

fn extract_doc(operation: &Operation, format: DocFormat) -> Option<Doc> {
    let mut doc_pieces = vec![];
    if let Some(summary) = operation.summary.as_ref() {
        if !summary.is_empty() {
            doc_pieces.push(summary.clone());
        }
    }
    if let Some(description) = operation.description.as_ref() {
        if !description.is_empty() {
            if !doc_pieces.is_empty() && description == &doc_pieces[0] {
            } else {
                doc_pieces.push(description.clone());
            }
        }
    }
    if let Some(external_docs) = operation.external_docs.as_ref() {
        doc_pieces.push(match format {
            DocFormat::Markdown => format!("See endpoint docs at <{}>.", external_docs.url),
            DocFormat::Rst => format!("See endpoint docs at `{url} <{url}>`_.", url = external_docs.url),
        })
    }
    if doc_pieces.is_empty() {
        None
    } else {
        Some(Doc(doc_pieces.join("\n\n")))
    }
}

pub fn extract_parameters(op: &Operation, item: &PathItem, spec: &OpenAPI) -> Result<Vec<hir::Parameter>> {
    let mut inputs = op
        .parameters
        .iter()
        .map(|param| extract_param(param, spec))
        .collect::<Result<Vec<_>, _>>()?;

    let args = item
        .parameters
        .iter()
        .map(|param| extract_param(param, spec))
        .collect::<Result<Vec<_>, _>>()?;
    for param in args {
        if !inputs.iter().any(|p| p.name == param.name) {
            inputs.push(param);
        }
    }

    let Some(body) = get_body(op, spec) else {
        return Ok(inputs);
    };

    if let SchemaKind::Type(Type::Array(ArrayType { items, .. })) = &body.kind {
        let ty = if let Some(items) = items {
            schema_ref_to_ty(&items, spec)
        } else {
            Ty::default()
        };
        let ty = Ty::Array(Box::new(ty));
        inputs.push(hir::Parameter {
            name: "body".to_string(),
            ty,
            optional: false,
            doc: None,
            location: Location::Body,
            example: body.example.clone(),
        });
        return Ok(inputs);
    }
    let mut props = body.properties_iter(spec).peekable();

    if props.peek().is_some() {
        let body_args = props.map(|(name, param)| {
            let ty = schema_ref_to_ty(param, spec);
            let param: &Schema = param.resolve(spec);
            let optional = extractor::is_optional(name, param, body);
            let name = name.to_string();
            hir::Parameter {
                name,
                ty,
                optional,
                doc: None,
                location: Location::Body,
                example: body.example.clone(),
            }
        });
        for param in body_args {
            if !inputs.iter().any(|p| p.name == param.name) {
                inputs.push(param);
            }
        }
    } else {
        inputs.push(hir::Parameter {
            name: "body".to_string(),
            ty: Ty::default(),
            optional: false,
            doc: None,
            location: Location::Body,
            example: body.example.clone(),
        });
    }
    Ok(inputs)
}

pub fn get_body<'a>(op: &'a Operation, spec: &'a OpenAPI) -> Option<&'a Schema> {
    let body = op.request_body.as_ref()?;
    let body = body.resolve(spec).unwrap();
    let content = body.content.get("application/json")?;
    let body = content.schema.as_ref()?;
    Some(body.resolve(spec))
}

pub fn get_res<'a>(operation: &'a Operation, spec: &'a OpenAPI) -> Option<&'a RefOr<Schema>> {
    use openapiv3::StatusCode;

    let res = &operation.responses.responses;
    let Some(res) = res
        .get(&StatusCode::Code(200))
        .or_else(|| res.get(&StatusCode::Code(201)))
        .or_else(|| res.get(&StatusCode::Code(202)))
        .or_else(|| res.get(&StatusCode::Code(204)))
        .or_else(|| res.get(&StatusCode::Code(302)))
    else {
        panic!("No success response for operation {:?}", operation);
    };
    let res = res.resolve(spec).unwrap();
    res.content
        .get("application/json")
        .and_then(|media| media.schema.as_ref())
}

pub fn extract_param(param: &ReferenceOr<Parameter>, spec: &OpenAPI) -> Result<hir::Parameter> {
    span!("extract_param", param = ?param);
    let param = param.resolve(spec)?;
    let data = &param.data;
    let param_schema_ref = data
        .schema()
        .ok_or_else(|| anyhow!("No schema for parameter: {:?}", param))?;
    let schema = param_schema_ref.resolve(spec);
    let ty = schema_ref_to_ty2(param_schema_ref, spec, schema);
    Ok(hir::Parameter {
        doc: None,
        name: data.name.to_string(),
        optional: !data.required,
        location: param.into(),
        ty,
        example: schema.example.clone(),
    })
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn test_make_operation_name() {
        let method = "get";
        let url = "/diffs/{id}";
        let op_name = make_name(None, method, url);
        assert_eq!(op_name, "get_diffs_by_id");
    }

    #[test]
    fn test_make_operation_name2() {
        let method = "get";
        let url = "/user/{user_id}/account/{account_id}";
        let op_name = make_name(None, method, url);
        assert_eq!(op_name, "get_user_account_by_id");
    }

    #[test]
    pub fn test_required_args() {
        let spec = include_str!("../../../test_specs/basic.yaml");
        let mut spec: OpenAPI = from_str(spec).unwrap();
        spec.paths.retain(|k, _| k == "/link/token/create");
        let (operation, path) = spec.get_operation("linkTokenCreate").unwrap();
        let inputs = extract_parameters(&operation, path, &spec).unwrap();
        assert_eq!(inputs[8].name, "user_token");
        assert_eq!(inputs[8].optional, true);
    }
}
