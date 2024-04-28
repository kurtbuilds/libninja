use std::collections::{BTreeMap, HashSet};

use convert_case::{Case, Casing};
/// Records are the "model"s of the MIR world. model is a crazy overloaded word though.
use openapiv3::{
    ObjectType, OpenAPI, ReferenceOr, RefOr, RefOrMap, Schema, SchemaData, SchemaKind,
    SchemaReference, StringType, Type,
};

use hir::{HirField, HirSpec, NewType, Record, StrEnum, Struct};
use mir::{Doc, Ty};

use crate::extractor;
use crate::extractor::{
    is_optional, is_primitive, schema_ref_to_ty, schema_ref_to_ty2, schema_to_ty,
};
use crate::sanitize::sanitize;
use crate::util::{is_plural, singular};

fn extract_fields(
    properties: &RefOrMap<Schema>,
    parent: &Schema,
    spec: &OpenAPI,
    hir: &mut HirSpec,
) -> BTreeMap<String, HirField> {
    properties
        .iter()
        .map(|(name, schema_ref)| {
            let schema = schema_ref.resolve(spec);
            let mut ty = None;
            if let RefOr::Item(schema) = schema_ref {
                if !is_primitive(schema, spec) {
                    let name = sanitize(name).to_case(Case::Pascal);
                    ty = extract_schema(&name, schema, spec, hir);
                }
            }
            let ty = ty.unwrap_or_else(|| schema_ref_to_ty2(schema_ref, spec, schema));
            let optional = is_optional(name, schema, parent);
            (
                name.clone(),
                HirField {
                    ty,
                    optional,
                    doc: extract_docs(schema),
                    example: schema.example.clone(),
                    flatten: false,
                },
            )
        })
        .collect()
}

fn create_field(field_schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> HirField {
    let field_schema = field_schema_ref.resolve(spec);
    let ty = schema_ref_to_ty2(field_schema_ref, spec, field_schema);
    let optional = field_schema.nullable;
    let example = field_schema.example.clone();
    let doc = extract_docs(field_schema);
    HirField {
        ty,
        optional,
        doc,
        example,
        flatten: false,
    }
}

pub fn effective_length(all_of: &[ReferenceOr<Schema>]) -> usize {
    let mut length = 0;
    for schema_ref in all_of {
        length += schema_ref.as_ref_str().map(|_s| 1).unwrap_or_default();
        length += schema_ref
            .as_item()
            .and_then(|s| s.get_properties())
            .map(|s| s.iter().len())
            .unwrap_or_default();
    }
    length
}

pub fn extract_schema(
    name: &str,
    schema: &Schema,
    spec: &OpenAPI,
    hir: &mut HirSpec,
) -> Option<Ty> {
    println!("Extracting schema: {}", name);
    let name = name.to_string();

    let k = &schema.kind;
    if let SchemaKind::Type(Type::Object(ObjectType { properties, .. })) = k {
        let fields = extract_fields(properties, schema, spec, hir);
        let s = Struct {
            name: name.clone(),
            fields,
            nullable: schema.nullable,
            docs: schema
                .description
                .as_ref()
                .map(|d| Doc(d.trim().to_string())),
        };
        hir.insert_schema(s);
        return None;
    }
    if let SchemaKind::Type(Type::String(StringType { enumeration, .. })) = k {
        if !enumeration.is_empty() {
            let s = StrEnum {
                name: name.clone(),
                variants: enumeration.iter().map(|s| s.clone()).collect(),
                docs: schema.description.as_ref().map(|d| Doc(d.clone())),
            };
            hir.insert_schema(s);
            return None;
        }
    }
    if let SchemaKind::AllOf { all_of } = k {
        extract_all_of(name, all_of.as_slice(), &schema.data, spec, hir);
        return None;
    }
    'foo: {
        let SchemaKind::Type(Type::Array(arr)) = k else {
            break 'foo;
        };
        let Some(items) = &arr.items.as_ref() else {
            break 'foo;
        };
        let Some(item) = items.as_item() else {
            break 'foo;
        };
        let schema_names = hir.schemas.iter().map(|(k, _)| k.clone()).collect();
        let Some(name) = create_unique_name(&schema_names, &name, &name) else {
            break 'foo;
        };
        extract_schema(&name, item, spec, hir);
        return Some(Ty::Array(Box::new(Ty::model(&name))));
    }
    extract_newtype(name, schema, spec, hir);
    None
}

fn extract_newtype(name: String, schema: &Schema, spec: &OpenAPI, hir: &mut HirSpec) {
    let t = NewType {
        name: name.clone(),
        fields: vec![HirField {
            ty: schema_to_ty(schema, spec),
            optional: schema.nullable,
            doc: None,
            example: None,
            flatten: false,
        }],
        docs: schema.description.as_ref().map(|d| Doc(d.clone())),
    };
    hir.insert_schema(t);
}

fn extract_all_of(
    name: String,
    all_of: &[ReferenceOr<Schema>],
    data: &SchemaData,
    spec: &OpenAPI,
    hir: &mut HirSpec,
) {
    if effective_length(&all_of) == 1 {
        let ty = schema_ref_to_ty(&all_of[0], spec);
        let field = HirField {
            ty,
            optional: data.nullable,
            ..HirField::default()
        };
        hir.insert_schema(Record::TypeAlias(name.clone(), field));
        return;
    }
    let mut fields = BTreeMap::new();
    for schema in all_of {
        match &schema {
            ReferenceOr::Reference { reference } => {
                let schema_ref = SchemaReference::from_str(reference);
                let name = extractor::get_name(schema_ref);
                let mut field = create_field(schema, spec);
                field.flatten = true;
                fields.insert(name, field);
            }
            ReferenceOr::Item(item) => {
                let Some(props) = item.get_properties() else {
                    continue;
                };
                for (name, schema) in props {
                    let mut field = create_field(schema, spec);
                    if !field.ty.is_iterable() && !item.required().iter().any(|s| s == name) {
                        field.optional = true;
                    }
                    fields.insert(name.to_string(), field);
                }
            }
        }
    }
    let s = Struct {
        nullable: data.nullable,
        name: name.to_string(),
        fields,
        docs: data.description.as_ref().map(|d| Doc(d.clone())),
    };
    hir.insert_schema(s);
}

/// When encountering anonymous nested structs (e.g. array items), use this function to come up with a name.
/// name: the object it resides on
/// field: the field name
fn create_unique_name(
    current_schemas: &HashSet<String>,
    name: &str,
    field: &str,
) -> Option<String> {
    if is_plural(field) {
        let singular_field = singular(field).to_case(Case::Pascal);
        if !current_schemas.contains(&singular_field) {
            return Some(singular_field);
        }
        let singular_field = format!("{}{}", name.to_case(Case::Pascal), singular_field);
        if !current_schemas.contains(&singular_field) {
            return Some(singular_field);
        }
    }
    let singular_field = format!("{}Item", field.to_case(Case::Pascal));
    if !current_schemas.contains(&singular_field) {
        return Some(singular_field);
    }
    let singular_field = format!("{}{}", name.to_case(Case::Pascal), singular_field);
    if !current_schemas.contains(&singular_field) {
        return Some(singular_field);
    }
    None
}

#[cfg(test)]
mod tests {
    use openapiv3::{OpenAPI, Schema, SchemaData, SchemaKind};
    use serde_yaml::from_str;

    use hir::HirSpec;

    use super::*;

    #[test]
    fn test_all_of_required_set_correctly() {
        let mut hir = HirSpec::default();
        let mut schema: Schema = from_str(include_str!("./pet_tag.yaml")).unwrap();
        let SchemaKind::AllOf { all_of } = &schema.kind else {
            panic!()
        };
        let spec = OpenAPI::default();
        let name = "PetTag".to_string();
        extract_all_of(name, &all_of, &SchemaData::default(), &spec, &mut hir);
        let rec = hir.schemas.get("PetTag").unwrap();
        let mut fields = rec.fields();
        let eye_color = fields.next().unwrap();
        let weight = fields.next().unwrap();
        assert_eq!(eye_color.optional, false);
        assert_eq!(weight.optional, true);
    }
}

pub fn extract_docs(schema: &Schema) -> Option<Doc> {
    schema
        .description
        .as_ref()
        .map(|d| Doc(d.trim().to_string()))
}
