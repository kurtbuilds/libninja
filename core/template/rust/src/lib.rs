#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
pub use httpclient::{Error, Result, InMemoryResponseExt};
use std::sync::{Arc, OnceLock};
use crate::model::*;
