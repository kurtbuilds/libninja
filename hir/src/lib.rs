use std::collections::BTreeMap;
/// The API model.
/// Higher level compared to code level models in ln-model.
use std::fmt::Debug;
use std::iter::{empty, Iterator, once};
use std::string::{String, ToString};

use anyhow::Result;
use convert_case::{Case, Casing};
use openapiv3 as oa;

pub use lang::*;
use mir::{Doc, ParamKey};
use mir::Ty;

mod lang;

/// Parameter is an input to an OpenAPI operation.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: Ty,
    pub location: Location,
    pub optional: bool,
    pub doc: Option<Doc>,
    pub example: Option<serde_json::Value>,
}

impl Parameter {
    pub fn to_key(&self) -> ParamKey {
        if self.ty.is_iterable() && self.location == Location::Query {
            ParamKey::RepeatedKey(self.name.clone())
        } else {
            ParamKey::Key(self.name.clone())
        }
    }

    pub fn path(name: &str, ty: Ty) -> Self {
        Self {
            name: name.to_string(),
            ty,
            location: Location::Path,
            optional: false,
            doc: None,
            example: None,
        }
    }
}

/// Describes how an Parameter should be placed in an API request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Path,
    Body,
    Query,
    Header,
    Cookie,
}

impl From<&oa::Parameter> for Location {
    fn from(p: &oa::Parameter) -> Self {
        match p.kind {
            oa::ParameterKind::Query { .. } => Location::Query,
            oa::ParameterKind::Header { .. } => Location::Header,
            oa::ParameterKind::Path { .. } => Location::Path,
            oa::ParameterKind::Cookie { .. } => Location::Cookie,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthParam {
    pub name: String,
    pub location: AuthLocation,
}

#[derive(Debug, Clone)]
pub enum AuthLocation {
    Header { key: String },
    Basic,
    Bearer,
    Token,
    Query { key: String },
    Cookie { key: String },
}

#[derive(Debug, Clone)]
pub enum AuthStrategy {
    Token(TokenAuth),
    OAuth2(Oauth2Auth),
    NoAuth,
}

#[derive(Debug, Clone)]
pub struct TokenAuth {
    pub name: String,
    pub fields: Vec<AuthParam>,
}

#[derive(Debug, Clone)]
pub struct Oauth2Auth {
    pub auth_url: String,
    pub exchange_url: String,
    pub refresh_url: String,
    // scope name, scope description
    pub scopes: Vec<(String, String)>,
}

#[derive(Debug, Default, Clone)]
pub struct HirField {
    pub ty: Ty,
    pub optional: bool,
    pub doc: Option<Doc>,
    pub example: Option<serde_json::Value>,
    pub flatten: bool,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub nullable: bool,
    pub fields: BTreeMap<String, HirField>,
    pub docs: Option<Doc>,
}

impl Into<Record> for Struct {
    fn into(self) -> Record {
        Record::Struct(self)
    }
}

#[derive(Debug, Clone)]
pub struct NewType {
    pub name: String,
    pub fields: Vec<HirField>,
    pub docs: Option<Doc>,
}

#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: String,
    pub ty: Ty,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct StrEnum {
    pub name: String,
    pub variants: Vec<String>,
    pub docs: Option<Doc>,
}

impl Into<Record> for StrEnum {
    fn into(self) -> Record {
        Record::Enum(self)
    }
}

/// an object type in the HIR
#[derive(Debug, Clone)]
pub enum Record {
    Struct(Struct),
    NewType(NewType),
    TypeAlias(String, HirField),
    Enum(StrEnum),
}

impl From<NewType> for Record {
    fn from(nt: NewType) -> Self {
        Record::NewType(nt)
    }
}

impl Record {
    pub fn name(&self) -> &str {
        match self {
            Record::Struct(s) => &s.name,
            Record::Enum(e) => &e.name,
            Record::NewType(n) => &n.name,
            Record::TypeAlias(name, _) => name,
        }
    }

    pub fn len_fields(&self) -> usize {
        match self {
            Record::Struct(s) => s.fields.len(),
            Record::Enum(_) => 0,
            Record::NewType(n) => n.fields.len(),
            Record::TypeAlias(_, _) => 0,
        }
    }

    pub fn fields(&self) -> Box<dyn Iterator<Item = &HirField> + '_> {
        match self {
            Record::Struct(s) => Box::new(s.fields.values()),
            Record::Enum(_) => Box::new(empty()),
            Record::NewType(n) => Box::new(n.fields.iter()),
            Record::TypeAlias(_, f) => Box::new(once(f)),
        }
    }

    pub fn fields_mut(&mut self) -> Box<dyn Iterator<Item = &mut HirField> + '_> {
        match self {
            Record::Struct(s) => Box::new(s.fields.iter_mut().map(|(_, f)| f)),
            Record::Enum(_) => Box::new(empty()),
            Record::NewType(n) => Box::new(n.fields.iter_mut()),
            Record::TypeAlias(_, f) => Box::new(once(f)),
        }
    }

    /// This is just for debug/testing to simplify output. It's not used in the actual codegen.
    pub fn clear_docs(&mut self) {
        for f in self.fields_mut() {
            f.doc = None;
        }
    }

    pub fn optional(&self) -> bool {
        match self {
            Record::Struct(_s) => false,
            Record::Enum(_) => false,
            Record::NewType(_) => false,
            Record::TypeAlias(_, f) => f.optional,
        }
    }

    pub fn as_struct(&self) -> Option<&Struct> {
        match self {
            Record::Struct(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HirSpec {
    pub operations: Vec<Operation>,
    pub schemas: BTreeMap<String, Record>,

    pub servers: BTreeMap<String, String>,
    pub security: Vec<AuthStrategy>,

    pub api_docs_url: Option<String>,
}

impl HirSpec {
    pub fn insert_schema(&mut self, record: impl Into<Record>) {
        let record = record.into();
        let name = record.name().to_string();
        if !name.chars().next().unwrap().is_uppercase() {
            panic!("Schema name must be uppercase: {}", name);
        }
        self.schemas.insert(name, record);
    }
}

pub enum ServerStrategy {
    /// No servers were provided, so we pass a base URL
    BaseUrl,
    /// There's only one
    Single(String),
    /// There's multiple choices
    Env,
}

impl ServerStrategy {
    pub fn env_var_for_strategy(&self, service_name: &str) -> Option<String> {
        match self {
            ServerStrategy::BaseUrl => Some(format!(
                "{}_BASE_URL",
                service_name.to_case(Case::ScreamingSnake)
            )),
            ServerStrategy::Single(_) => None,
            ServerStrategy::Env => Some(format!(
                "{}_ENV",
                service_name.to_case(Case::ScreamingSnake)
            )),
        }
    }
}

pub fn qualified_env_var(service: &str, var_name: &str) -> String {
    format!("{} {}", service, var_name).to_case(Case::ScreamingSnake)
}

impl HirSpec {
    pub fn get_record(&self, name: &str) -> Result<&Record> {
        self.schemas
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("No record named {}", name))
    }

    pub fn get_operation(&self, name: &str) -> Result<&Operation> {
        self.operations
            .iter()
            .find(|o| o.name == name)
            .ok_or_else(|| anyhow::anyhow!("No operation named {}", name))
    }

    pub fn server_strategy(&self) -> ServerStrategy {
        let len = self.servers.len();
        if len == 0 {
            ServerStrategy::BaseUrl
        } else if len == 1 {
            ServerStrategy::Single(self.servers.values().next().unwrap().clone())
        } else {
            ServerStrategy::Env
        }
    }

    pub fn multiple_security(&self) -> bool {
        self.security.len() > 1
    }

    pub fn env_vars(&self, service_name: &str) -> Vec<String> {
        let mut env_vars = vec![];
        if let Some(env) = self.server_strategy().env_var_for_strategy(service_name) {
            env_vars.push(env);
        }
        for strategy in &self.security {
            match strategy {
                AuthStrategy::Token(t) => {
                    for f in &t.fields {
                        let qev = qualified_env_var(service_name, &f.name);
                        env_vars.push(qev);
                    }
                }
                AuthStrategy::OAuth2(_) => {
                    env_vars.push(qualified_env_var(service_name, "CLIENT_ID"));
                    env_vars.push(qualified_env_var(service_name, "CLIENT_SECRET"));
                }
                AuthStrategy::NoAuth => {}
            }
        }
        env_vars
    }

    pub fn has_security(&self) -> bool {
        !self.security.is_empty()
    }

    pub fn has_basic_auth(&self) -> bool {
        self.security
            .iter()
            .any(|s| matches!(s, AuthStrategy::Token(_)))
    }

    pub fn oauth2_auth(&self) -> Option<&Oauth2Auth> {
        self.security
            .iter()
            .filter_map(|s| match s {
                AuthStrategy::OAuth2(o) => Some(o),
                _ => None,
            })
            .next()
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub name: String,
    pub doc: Option<Doc>,
    pub parameters: Vec<Parameter>,
    pub ret: Ty,
    pub path: String,
    pub method: String,
}

impl Operation {
    // Mostly for Go
    pub fn flat_package_name(&self) -> String {
        self.name.to_case(Case::Flat)
    }

    pub fn file_name(&self) -> String {
        self.name.to_case(Case::Snake)
    }

    pub fn request_struct_name(&self) -> String {
        format!("{}Request", self.name)
    }

    pub fn required_struct_name(&self) -> String {
        format!("{}Required", self.name)
    }

    pub fn crowded_args(&self) -> bool {
        self.parameters.iter().filter(|p| !p.optional).count() > 3
    }

    pub fn has_response(&self) -> bool {
        !matches!(self.ret, Ty::Unit)
    }

    pub fn optional_args(&self) -> Vec<&Parameter> {
        self.parameters.iter().filter(|p| p.optional).collect()
    }

    pub fn required_args(&self) -> Vec<&Parameter> {
        self.parameters.iter().filter(|p| !p.optional).collect()
    }

    pub fn parameters_by_header_query_body(
        &self,
    ) -> (Vec<&Parameter>, Vec<&Parameter>, Vec<&Parameter>) {
        let mut header = Vec::new();
        let mut query = Vec::new();
        let mut body = Vec::new();
        self.parameters.iter().for_each(|p| match p.location {
            Location::Header => header.push(p),
            Location::Query => query.push(p),
            Location::Body => body.push(p),
            _ => {}
        });
        (header, query, body)
    }

    pub fn use_required_struct(&self, sourcegen: Language) -> bool {
        matches!(sourcegen, Language::Rust | Language::Golang | Language::Typescript if self.crowded_args())
    }

    /// Returns the params that are used as function arguments.
    pub fn function_args(&self, generator: Language) -> Vec<Parameter> {
        match generator {
            Language::Golang if self.crowded_args() => {
                vec![Parameter {
                    name: "args".to_string(),
                    ty: Ty::model("Required"),
                    location: Location::Body,
                    optional: false,
                    doc: None,
                    example: None,
                }]
            }
            _ if self.use_required_struct(generator) => {
                vec![Parameter {
                    name: "args".to_string(),
                    ty: Ty::Model(self.required_struct_name()),
                    location: Location::Body,
                    optional: false,
                    doc: None,
                    example: None,
                }]
            }
            _ => self
                .parameters
                .iter()
                .filter(|p| !p.optional)
                .cloned()
                .collect(),
        }
    }

    pub fn required_struct(&self, sourcegen: Language) -> Struct {
        let fields = match sourcegen {
            Language::Typescript => self
                .parameters
                .iter()
                .map(|p| (p.name.clone(), p.into()))
                .collect(),
            Language::Rust | Language::Golang => self
                .parameters
                .iter()
                .filter(|p| !p.optional)
                .map(|p| (p.name.clone(), p.into()))
                .collect(),
            _ => unimplemented!(),
        };
        Struct {
            nullable: false,
            name: self.required_struct_name(),
            fields,
            docs: None,
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            doc: None,
            parameters: Vec::new(),
            ret: Ty::Unit,
            path: "".to_string(),
            method: "".to_string(),
        }
    }
}

impl From<&Parameter> for HirField {
    fn from(p: &Parameter) -> Self {
        Self {
            ty: p.ty.clone(),
            optional: p.optional,
            doc: p.doc.clone(),
            example: p.example.clone(),
            flatten: false,
        }
    }
}
