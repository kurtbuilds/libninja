/// Records are the "model"s of the MIR world. model is a crazy overloaded word though.

use openapiv3::{ObjectType, OpenAPI, ReferenceOr, Schema, SchemaData, SchemaKind, SchemaReference, StatusCode, StringType, Type};
use ln_mir::{Doc, Name};
use std::collections::{BTreeMap, HashMap};
use tracing_ez::warn;
use crate::{extractor, hir};
use crate::extractor::{schema_to_ty, schema_ref_to_ty_already_resolved};
use crate::hir::{MirField, Record, StrEnum, Struct};
use indexmap::IndexMap;
use anyhow::Result;
use crate::child_schemas::ChildSchemas;

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
        length += schema_ref.as_ref_str().map(|_s| 1).unwrap_or_default();
        length += schema_ref.as_item()
            .and_then(|s| s.properties() )
            .map(|s| s.iter().len() )
            .unwrap_or_default();
    }
    length
}

pub fn create_record(name: &str, schema: &Schema, spec: &OpenAPI) -> Record {
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
                Record::TypeAlias(Name::new(name), MirField {
                    ty: schema_ref_to_ty_already_resolved(&all_of[0], spec, schema),
                    optional: schema.schema_data.nullable,
                    ..MirField::default()
                })
            } else {
                create_record_from_all_of(name, all_of, &schema.schema_data, spec)
            }
        }
        // Default case, a newtype with a single field
        _ => Record::NewType(hir::NewType {
            name: Name::new(name),
            fields: vec![MirField {
                ty: schema_to_ty(schema, spec),
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
                            let mut field = create_field(schema, spec);
                            if !item.required(name) {
                                field.optional = true;
                            }
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
    let mut result: BTreeMap<String, Record> = BTreeMap::new();
    let mut schema_lookup = HashMap::new();
    spec.add_child_schemas(&mut schema_lookup);
    for (name, schema) in schema_lookup {
        let rec = create_record(&name, schema, spec);
        let name = rec.name().0.clone();
        result.insert(name, rec);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaData, SchemaKind};
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