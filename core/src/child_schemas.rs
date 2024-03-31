use std::collections::HashMap;

use openapiv3::{OpenAPI, Operation, RequestBody, Response, Schema, SchemaKind, Type};

use crate::sanitize::sanitize;

pub trait ChildSchemas {
    fn add_child_schemas<'a>(&'a self, acc: &mut HashMap<String, &'a Schema>);
}

impl ChildSchemas for Schema {
    fn add_child_schemas<'a>(&'a self, acc: &mut HashMap<String, &'a Schema>) {
        match &self.kind {
            SchemaKind::Type(Type::Array(a)) => {
                let Some(items) = &a.items else {
                    return;
                };
                let Some(item) = items.as_item() else {
                    return;
                };
                if let Some(title) = &item.title {
                    let title = sanitize(title).to_string();
                    acc.entry(title).or_insert(item);
                }
                item.add_child_schemas(acc);
            }
            SchemaKind::Type(Type::Object(o)) => {
                if let Some(title) = &self.title {
                    let title = sanitize(title).to_string();
                    acc.entry(title).or_insert(self);
                }
                for (_name, prop) in &o.properties {
                    let Some(prop) = prop.as_item() else {
                        continue;
                    };
                    if let Some(title) = &prop.title {
                        let title = sanitize(title).to_string();
                        acc.entry(title).or_insert(prop);
                    }
                    prop.add_child_schemas(acc);
                }
            }
            SchemaKind::Type(_) => {}
            SchemaKind::OneOf { one_of: schemas }
            | SchemaKind::AllOf { all_of: schemas }
            | SchemaKind::AnyOf { any_of: schemas } => {
                for schema in schemas {
                    let Some(schema) = schema.as_item() else {
                        continue;
                    };
                    if let Some(title) = &schema.title {
                        let title = sanitize(title).to_string();
                        acc.entry(title).or_insert(schema);
                    }
                    schema.add_child_schemas(acc);
                }
            }
            SchemaKind::Not { .. } => {}
            SchemaKind::Any(_) => {}
        }
    }
}

impl ChildSchemas for Operation {
    fn add_child_schemas<'a>(&'a self, acc: &mut HashMap<String, &'a Schema>) {
        'body: {
            let Some(body) = &self.request_body else {
                break 'body;
            };
            let Some(body) = body.as_item() else {
                break 'body;
            };
            body.add_child_schemas(acc);
        }
        for par in &self.parameters {
            let Some(par) = par.as_item() else {
                continue;
            };
            let Some(schema) = par.data.schema() else {
                continue;
            };
            let Some(schema) = schema.as_item() else {
                continue;
            };
            schema.add_child_schemas(acc);
        }
        for (_code, response) in &self.responses.responses {
            let Some(response) = response.as_item() else {
                continue;
            };
            response.add_child_schemas(acc);
        }
    }
}

impl ChildSchemas for RequestBody {
    fn add_child_schemas<'a>(&'a self, acc: &mut HashMap<String, &'a Schema>) {
        for (_key, content) in &self.content {
            let Some(schema) = &content.schema else {
                continue;
            };
            let Some(schema) = schema.as_item() else {
                continue;
            };
            if let Some(title) = &schema.title {
                acc.entry(title.clone()).or_insert(schema);
            }
            schema.add_child_schemas(acc);
        }
    }
}

impl ChildSchemas for Response {
    fn add_child_schemas<'a>(&'a self, acc: &mut HashMap<String, &'a Schema>) {
        for (k, content) in &self.content {
            let Some(schema) = &content.schema else {
                continue;
            };
            let Some(schema) = schema.as_item() else {
                continue;
            };
            if let Some(title) = &schema.title {
                acc.entry(title.clone()).or_insert(schema);
            }
            schema.add_child_schemas(acc);
        }
    }
}

impl ChildSchemas for OpenAPI {
    fn add_child_schemas<'a>(&'a self, acc: &mut HashMap<String, &'a Schema>) {
        for (_path, _method, op, _item) in self.operations() {
            op.add_child_schemas(acc);
        }
        for (name, schema) in &self.schemas {
            let Some(schema) = schema.as_item() else {
                continue;
            };
            acc.entry(name.clone()).or_insert(schema);
            schema.add_child_schemas(acc);
        }
    }
}
