use crate::{Language, Location, Parameter, Struct};
use convert_case::{Case, Casing};
use mir::{Doc, Ty};

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

    pub fn parameters_by_header_query_body(&self) -> (Vec<&Parameter>, Vec<&Parameter>, Vec<&Parameter>) {
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

    pub fn use_required_struct(&self, _sourcegen: Language) -> bool {
        // matches!(sourcegen, Language::Rust | Language::Golang | Language::Typescript if self.crowded_args())
        self.crowded_args()
    }

    pub fn required_struct(&self, sourcegen: Language) -> Struct {
        let fields = match sourcegen {
            // Language::Typescript => self
            //     .parameters
            //     .iter()
            //     .map(|p| (p.name.clone(), p.into()))
            //     .collect(),
            Language::Rust
            // | Language::Golang
            => self
                .parameters
                .iter()
                .filter(|p| !p.optional)
                .map(|p| (p.name.clone(), p.into()))
                .collect(),
            // _ => unimplemented!(),
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
