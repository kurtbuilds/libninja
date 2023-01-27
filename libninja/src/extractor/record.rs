/// Records are the "model"s of the MIR world. model is a crazy overloaded word though.

use openapiv3::{ObjectType, OpenAPI, ReferenceOr, Schema, SchemaData, SchemaKind, SchemaReference, StringType, Type};
use ln_model::{Doc, Name};
use std::collections::{BTreeMap, HashMap};
use std::ops::IndexMut;
use tracing_ez::warn;
use crate::{extractor, mir};
use crate::extractor::schema_ref_to_ty_already_resolved;
use crate::mir::{MirField, Record, StrEnum, Struct};
use indexmap::IndexMap;
use anyhow::Result;

fn properties_to_fields(properties: &IndexMap<String, ReferenceOr<Schema>>, schema: &Schema, spec: &OpenAPI) -> BTreeMap<Name, MirField> {
    properties
        .iter()
        .map(|(name, field_schema_ref)| {
            let field_schema = field_schema_ref.resolve(spec);
            let ty = schema_ref_to_ty_already_resolved(
                field_schema_ref,
                spec,
                field_schema,
            );
            let optional = extractor::is_optional(name, field_schema, schema);
            (Name::new(name), MirField {
                ty,
                optional,
                doc: extractor::extract_schema_docs(field_schema),
                example: None,
                flatten: false,
            })
        })
        .collect()
}

pub fn effective_length(all_of: &[ReferenceOr<Schema>]) -> usize {
    let mut length = 0;
    for schema_ref in all_of {
        length += schema_ref.as_ref_str().map(|s| 1).unwrap_or_default();
        length += schema_ref.as_item()
            .and_then(|s| s.properties() )
            .map(|s| s.iter().len() )
            .unwrap_or_default();
    }
    length
}

pub fn create_record(name: &str, schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> Record {
    let schema = schema_ref.resolve(spec);
    match &schema.schema_kind {
        // The base case, a regular object
        SchemaKind::Type(Type::Object(ObjectType { properties, .. })) => {
            let fields = properties_to_fields(properties, schema, spec);
            Record::Struct(Struct { name: Name::new(name), fields, nullable: schema.schema_data.nullable })
        }
        // An enum
        SchemaKind::Type(Type::String(StringType { enumeration, .. }))
        if !enumeration.is_empty() =>
            {
                Record::Enum(StrEnum {
                    name: Name::new(name),
                    variants: enumeration
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                })
            }
        // A newtype with multiple fields
        SchemaKind::AllOf { all_of } => {
            if effective_length(all_of) == 1 {
                Record::TypeAlias(Name::new(name), mir::MirField {
                    ty: schema_ref_to_ty_already_resolved(&all_of[0], spec, schema),
                    optional: schema.schema_data.nullable,
                    ..MirField::default()
                })
            } else {
                create_record_from_all_of(name, all_of, &schema.schema_data, spec)
            }
        }
        // Default case, a newtype with a single field
        _ => Record::NewType(mir::NewType {
            name: Name::new(name),
            fields: vec![MirField {
                ty: schema_ref_to_ty_already_resolved(schema_ref, spec, schema),
                optional: schema.schema_data.nullable,
                doc: None,
                example: None,
                flatten: false,
            }],
        }),
    }
}


fn create_field(field_schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> MirField {
    let field_schema = field_schema_ref.resolve(spec);
    let ty = schema_ref_to_ty_already_resolved(
        field_schema_ref,
        spec,
        field_schema,
    );
    let optional = field_schema.schema_data.nullable;
    let example = field_schema.schema_data.example.clone();
    let doc = field_schema.schema_data.description.clone().map(Doc);
    MirField { ty, optional, doc, example, flatten: false }
}

fn create_record_from_all_of(name: &str, all_of: &[ReferenceOr<Schema>], schema_data: &SchemaData, spec: &OpenAPI) -> Record {
    let mut fields = BTreeMap::new();
    for schema in all_of {
        match &schema {
            ReferenceOr::Reference { reference } => {
                let schema_ref = SchemaReference::from_str(reference);
                let name = extractor::get_name(schema_ref);
                let mut field = create_field(schema, spec);
                field.flatten = true;
                fields.insert(Name(name), field);
            }
            ReferenceOr::Item(item) => {
                match item.properties() {
                    Some(props) => {
                        for (name, schema) in props {
                            let field = create_field(schema, spec);
                            fields.insert(Name::new(name), field);
                        }
                    }
                    None => {
                        warn!("Could not extract {} properties {:?}", name, item);
                    }
                }
            }
        }
    }
    Record::Struct(Struct {
        nullable: schema_data.nullable,
        name: Name::new(name),
        fields,
    })
}

// records are data types: structs, newtypes
pub fn extract_records(spec: &OpenAPI) -> Result<BTreeMap<String, Record>> {
    if spec.components.is_none() {
        return Ok(BTreeMap::new());
    }
    let mut result: BTreeMap<String, Record> = spec.schemas()
        .into_iter()
        .map(|(name, schema)| {
            create_record(name, schema, spec)
        })
        .map(|r| {
            let name = r.name().0.clone();
            Ok((name, r))
        })
        .collect::<Result<_>>()?;
    Ok(result)
}
