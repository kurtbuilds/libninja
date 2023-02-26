use openapiv3::{ArrayType, OpenAPI, ReferenceOr, Schema, SchemaKind, SchemaReference};
use tracing_ez::warn;

use crate::hir::Ty;

use openapiv3 as oa;

pub fn schema_ref_to_ty(schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> Ty {
    let schema = schema_ref.resolve(spec);
    schema_ref_to_ty_already_resolved(schema_ref, spec, schema)
}

pub fn schema_ref_to_ty_already_resolved(schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI, schema: &Schema) -> Ty {
    if is_primitive(schema, spec) {
        concrete_schema_to_ty(schema, spec)
    } else {
        match schema_ref {
            ReferenceOr::Reference { reference } => {
                let r = oa::SchemaReference::from_str(reference);
                match r {
                    SchemaReference::Schema { schema: s } => Ty::model(&s),
                    SchemaReference::Property { schema: _, property: _ } => unimplemented!(),
                }
            }
            ReferenceOr::Item(schema) => concrete_schema_to_ty(schema, spec)
        }
    }
}

/// You probably want schema_ref_to_ty, not this method. Reason being, you want
/// to use the ref'd model if one exists (e.g. User instead of resolving to Ty::Any)
pub fn concrete_schema_to_ty(schema: &Schema, spec: &OpenAPI) -> Ty {
    match &schema.schema_kind {
        SchemaKind::Type(oa::Type::String(s)) => {
            match s.format.as_str() {
                "currency" => Ty::Currency {
                    serialization: crate::hir::CurrencySerialization::String,
                },
                "date" => Ty::Date {
                    serialization: crate::hir::DateSerialization::Iso8601,
                },
                "date-time" => Ty::DateTime,
                _ => Ty::String,
            }
        }
        SchemaKind::Type(oa::Type::Number(_)) => Ty::Float,
        SchemaKind::Type(oa::Type::Integer(_)) => {
            let null_as_zero = schema.schema_data.extensions.get("x-null-as-zero")
                .and_then(|v| v.as_bool()).unwrap_or(false);
            match schema.schema_data.extensions.get("x-format").and_then(|s| s.as_str()) {
                Some("date") => Ty::Date {
                    serialization: crate::hir::DateSerialization::Integer,
                },
                _ => Ty::Integer { null_as_zero }
            }
        }
        SchemaKind::Type(oa::Type::Boolean {}) => Ty::Boolean,
        SchemaKind::Type(oa::Type::Object(_)) => Ty::Any,
        SchemaKind::Type(oa::Type::Array(ArrayType {
                                             items: Some(item), ..
                                         })) => {
            let inner = item.unbox();
            let inner = schema_ref_to_ty(&inner, spec);
            Ty::Array(Box::new(inner))
        }
        SchemaKind::Type(oa::Type::Array(ArrayType { items: None, .. })) => {
            warn!("Array with no items. Defaulting to Array<Any>");
            Ty::Array(Box::new(Ty::Any))
        }
        SchemaKind::Any(..) => Ty::Any,
        SchemaKind::AllOf { all_of } => {
            if all_of.len() == 1 {
                schema_ref_to_ty(&all_of[0], spec)
            } else {
                Ty::Any
            }
        }
        SchemaKind::OneOf { .. } => Ty::Any,
        SchemaKind::AnyOf { .. } => Ty::Any,
        SchemaKind::Not { .. } => Ty::Any,
    }
}


pub fn is_primitive(schema: &Schema, spec: &OpenAPI) -> bool {
    use openapiv3::SchemaKind::*;
    use openapiv3::Type::*;
    match &schema.schema_kind {
        Type(String(_)) => true,
        Type(Number(_)) => true,
        Type(Integer(_)) => true,
        Type(Boolean {}) => true,
        Type(Array(ArrayType {
                       items: Some(inner), ..
                   })) => {
            let inner = inner.unbox();
            let inner = inner.resolve(spec);
            is_primitive(inner, spec)
        }
        SchemaKind::AllOf { all_of } => {
            all_of.len() == 1 && is_primitive(all_of[0].resolve(spec), spec)
        }
        _ => false,
    }
}
