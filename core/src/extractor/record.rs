use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use indexmap::IndexMap;
/// Records are the "model"s of the MIR world. model is a crazy overloaded word though.

use openapiv3::{ObjectType, OpenAPI, ReferenceOr, Schema, SchemaData, SchemaKind, SchemaReference, StringType, Type};
use tracing::warn;

use hir::{Doc, HirField, Record, StrEnum, Struct, NewType};

use crate::extractor;
use crate::child_schemas::ChildSchemas;
use crate::extractor::{schema_ref_to_ty_already_resolved, schema_to_ty};

fn properties_to_fields(properties: &IndexMap<String, ReferenceOr<Schema>>, schema: &Schema, spec: &OpenAPI) -> BTreeMap<String, HirField> {
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
            (name.clone(), HirField {
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
        length += schema_ref.as_ref_str().map(|_s| 1).unwrap_or_default();
        length += schema_ref.as_item()
            .and_then(|s| s.properties() )
            .map(|s| s.iter().len() )
            .unwrap_or_default();
    }
    length
}

pub fn create_record(name: &str, schema: &Schema, spec: &OpenAPI) -> Record {
    let name = name.to_string();
    match &schema.schema_kind {
        // The base case, a regular object
        SchemaKind::Type(Type::Object(ObjectType { properties, .. })) => {
            let fields = properties_to_fields(properties, schema, spec);
            Record::Struct(Struct { name, fields, nullable: schema.schema_data.nullable })
        }
        // An enum
        SchemaKind::Type(Type::String(StringType { enumeration, .. }))
        if !enumeration.is_empty() =>
            {
                Record::Enum(StrEnum {
                    name,
                    variants: enumeration
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                })
            }
        // A newtype with multiple fields
        SchemaKind::AllOf { all_of } => {
            if effective_length(all_of) == 1 {
                Record::TypeAlias(name, HirField {
                    ty: schema_ref_to_ty_already_resolved(&all_of[0], spec, schema),
                    optional: schema.schema_data.nullable,
                    ..HirField::default()
                })
            } else {
                create_record_from_all_of(&name, all_of, &schema.schema_data, spec)
            }
        }
        // Default case, a newtype with a single field
        _ => Record::NewType(NewType {
            name,
            fields: vec![HirField {
                ty: schema_to_ty(schema, spec),
                optional: schema.schema_data.nullable,
                doc: None,
                example: None,
                flatten: false,
            }],
        }),
    }
}


fn create_field(field_schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> HirField {
    let field_schema = field_schema_ref.resolve(spec);
    let ty = schema_ref_to_ty_already_resolved(
        field_schema_ref,
        spec,
        field_schema,
    );
    let optional = field_schema.schema_data.nullable;
    let example = field_schema.schema_data.example.clone();
    let doc = field_schema.schema_data.description.clone().map(Doc);
    HirField { ty, optional, doc, example, flatten: false }
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
                fields.insert(name, field);
            }
            ReferenceOr::Item(item) => {
                match item.properties() {
                    Some(props) => {
                        for (name, schema) in props {
                            let mut field = create_field(schema, spec);
                            if !item.required(name) {
                                field.optional = true;
                            }
                            fields.insert(name.to_string(), field);
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
        name: name.to_string(),
        fields,
    })
}

// records are data types: structs, newtypes
pub fn extract_records(spec: &OpenAPI) -> Result<BTreeMap<String, Record>> {
    let mut result: BTreeMap<String, Record> = BTreeMap::new();
    let mut schema_lookup = HashMap::new();

    spec.add_child_schemas(&mut schema_lookup);
    for (mut name, schema) in schema_lookup {
        let rec = create_record(&name, schema, spec);
        let name = rec.name().to_string();
        result.insert(name, rec);
    }

    for (name, schema_ref)  in spec.schemas() {
        let Some(reference) = schema_ref.as_ref_str() else { continue; };
        result.insert(name.clone(), Record::TypeAlias(name.clone(), create_field(&schema_ref, spec)));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use openapiv3::{OpenAPI, Schema, SchemaData, SchemaKind};

    use crate::extractor::record::create_record_from_all_of;

    #[test]
    fn test_all_of_required_set_correctly() {
        let mut additional_props: Schema = serde_yaml::from_str(include_str!("./pet_tag.yaml")).unwrap();
        let SchemaKind::AllOf { all_of } = &additional_props.schema_kind else { panic!() };
        let spec = OpenAPI::default();
        let rec = create_record_from_all_of("PetTag", &all_of, &SchemaData::default(), &spec);
        let mut fields = rec.fields();
        let eye_color = fields.next().unwrap();
        let weight = fields.next().unwrap();
        assert_eq!(eye_color.optional, false);
        assert_eq!(weight.optional, true);
    }
}