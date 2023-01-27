use tracing_ez::warn;
use openapiv3 as oa;
use openapiv3::{ArrayType, OpenAPI, ReferenceOr, Schema, SchemaKind, SchemaReference};
use crate::mir;
use crate::mir::Ty;
use crate::util::is_primitive;


pub fn schema_ref_to_ty(schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> mir::Ty {
    let schema = schema_ref.resolve(spec);
    schema_ref_to_ty_already_resolved(schema_ref, spec, schema)
}

pub fn schema_ref_to_ty_already_resolved(schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI, schema: &Schema) -> mir::Ty {
    if is_primitive(schema, spec) {
        concrete_schema_to_ty(schema, spec)
    } else {
        match schema_ref {
            ReferenceOr::Reference { reference } => {
                let r = oa::SchemaReference::from_str(reference);
                match r {
                    SchemaReference::Schema { schema: s } => Ty::model(&s),
                    SchemaReference::Property { schema, property } => unimplemented!(),
                }
            }
            ReferenceOr::Item(schema) => concrete_schema_to_ty(schema, spec)
        }
    }
}

/// You probably want schema_ref_to_ty, not this method. Reason being, you want
/// to use the ref'd model if one exists (e.g. User instead of resolving to Ty::Any)
pub fn concrete_schema_to_ty(schema: &Schema, spec: &OpenAPI) -> mir::Ty {
    match &schema.schema_kind {
        SchemaKind::Type(oa::Type::String(s)) => Ty::String,
        SchemaKind::Type(oa::Type::Number(_)) => Ty::Float,
        SchemaKind::Type(oa::Type::Integer(_)) => Ty::Integer,
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
