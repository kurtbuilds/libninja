use openapiv3::{ArrayType, OpenAPI, ReferenceOr, Schema, SchemaKind, SchemaReference};
use openapiv3 as oa;
use tracing::warn;

use mir::Ty;

pub fn schema_ref_to_ty(schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> Ty {
    let schema = schema_ref.resolve(spec);
    schema_ref_to_ty_already_resolved(schema_ref, spec, schema)
}

pub fn schema_ref_to_ty_already_resolved(
    schema_ref: &ReferenceOr<Schema>,
    spec: &OpenAPI,
    schema: &Schema,
) -> Ty {
    if is_primitive(schema, spec) {
        schema_to_ty(schema, spec)
    } else {
        match schema_ref {
            ReferenceOr::Reference { reference } => {
                let r = oa::SchemaReference::from_str(reference);
                match r {
                    SchemaReference::Schema { schema: s } => Ty::model(&s),
                    SchemaReference::Property {
                        schema: _,
                        property: _,
                    } => unimplemented!(),
                }
            }
            ReferenceOr::Item(schema) => schema_to_ty(schema, spec),
        }
    }
}

/// You probably want schema_ref_to_ty, not this method. Reason being, you want
/// to use the ref'd model if one exists (e.g. User instead of resolving to Ty::Any)
pub fn schema_to_ty(schema: &Schema, spec: &OpenAPI) -> Ty {
    match &schema.kind {
        SchemaKind::Type(oa::Type::String(s)) => match s.format.as_str() {
            "decimal" => Ty::Currency {
                serialization: mir::DecimalSerialization::String,
            },
            "integer" => Ty::Integer {
                serialization: mir::IntegerSerialization::String,
            },
            "date" => Ty::Date {
                serialization: mir::DateSerialization::Iso8601,
            },
            "date-time" => Ty::DateTime,
            _ => Ty::String,
        },
        SchemaKind::Type(oa::Type::Number(_)) => Ty::Float,
        SchemaKind::Type(oa::Type::Integer(_)) => {
            let null_as_zero = schema
                .data
                .extensions
                .get("x-null-as-zero")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if null_as_zero {
                return Ty::Integer {
                    serialization: mir::IntegerSerialization::NullAsZero,
                };
            }
            match schema
                .data
                .extensions
                .get("x-format")
                .and_then(|s| s.as_str())
            {
                Some("date") => Ty::Date {
                    serialization: mir::DateSerialization::Integer,
                },
                _ => Ty::Integer {
                    serialization: mir::IntegerSerialization::Simple,
                },
            }
        }
        SchemaKind::Type(oa::Type::Boolean {}) => Ty::Boolean,
        SchemaKind::Type(oa::Type::Object(_)) => {
            if let Some(title) = &schema.title {
                Ty::model(&title)
            } else {
                Ty::Any(Some(schema.clone()))
            }
        }
        SchemaKind::Type(oa::Type::Array(ArrayType {
            items: Some(item), ..
        })) => {
            let inner = schema_ref_to_ty(&item, spec);
            Ty::Array(Box::new(inner))
        }
        SchemaKind::Type(oa::Type::Array(ArrayType { items: None, .. })) => {
            warn!("Array with no items. Defaulting to Array<Any>");
            Ty::Array(Box::new(Ty::default()))
        }
        SchemaKind::Any(..) => Ty::default(),
        SchemaKind::AllOf { all_of } => {
            if all_of.len() == 1 {
                schema_ref_to_ty(&all_of[0], spec)
            } else {
                Ty::default()
            }
        }
        SchemaKind::OneOf { .. } => Ty::default(),
        SchemaKind::AnyOf { .. } => Ty::default(),
        SchemaKind::Not { .. } => Ty::default(),
    }
}

pub fn is_primitive(schema: &Schema, spec: &OpenAPI) -> bool {
    use openapiv3::SchemaKind::*;
    use openapiv3::Type::*;
    match &schema.kind {
        Type(String(_)) => true,
        Type(Number(_)) => true,
        Type(Integer(_)) => true,
        Type(Boolean {}) => true,
        Type(Array(ArrayType {
            items: Some(inner), ..
        })) => {
            let inner = inner.resolve(spec);
            is_primitive(inner, spec)
        }
        SchemaKind::AllOf { all_of } => {
            all_of.len() == 1 && is_primitive(all_of[0].resolve(spec), spec)
        }
        _ => false,
    }
}
