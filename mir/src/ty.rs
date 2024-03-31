use openapiv3 as oa;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateSerialization {
    Iso8601,
    Integer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecimalSerialization {
    String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegerSerialization {
    Simple,
    String,
    NullAsZero,
}

#[derive(Debug, Clone)]
pub enum Ty {
    String,
    Integer { serialization: IntegerSerialization },
    Float,
    Boolean,
    Array(Box<Ty>),
    // OpenAPI name for the model. Hasn't been converted to a language type (e.g. cased, sanitized)
    Model(String),
    Unit,
    Date { serialization: DateSerialization },
    DateTime,
    Currency { serialization: DecimalSerialization },
    Any(Option<oa::Schema>),
}

impl Default for Ty {
    fn default() -> Self {
        Ty::Any(None)
    }
}

impl Ty {
    pub fn integer() -> Self {
        Ty::Integer {
            serialization: IntegerSerialization::Simple,
        }
    }

    pub fn inner_model(&self) -> Option<&String> {
        match self {
            Ty::Model(name) => Some(name),
            Ty::Array(ty) => ty.inner_model(),
            _ => None,
        }
    }

    pub fn is_iterable(&self) -> bool {
        self.inner_iterable().is_some()
    }

    pub fn inner_iterable(&self) -> Option<&Ty> {
        match self {
            Ty::Array(ty) => Some(ty.as_ref()),
            _ => None,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Ty::String => true,
            Ty::Integer { .. } => true,
            Ty::Float => true,
            Ty::Boolean => true,
            Ty::Array(_) => false,
            Ty::Model(_) => false,
            Ty::Any(_) => false,
            Ty::Unit => true,
            Ty::Date { .. } => true,
            Ty::Currency { .. } => true,
            Ty::DateTime => true,
        }
    }

    pub fn model(s: &str) -> Self {
        if s.contains('(') {
            panic!("Model names should not contain parens: {}", s);
        }
        Ty::Model(s.to_string())
    }
}
