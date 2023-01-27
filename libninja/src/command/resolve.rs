use clap::Args;
use std::convert::Infallible;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{anyhow, Result};
use serde_json::{json, Value};

fn walk(value: &mut Value, callback: impl Fn(&mut Value, String)) -> Result<()> {
    fn _walk(value: &mut Value, callback: &impl Fn(&mut Value, String)) -> Result<()> {
        match value {
            Value::Array(arr) => {
                for item in arr {
                    _walk(item, callback)?;
                }
            }
            Value::Object(ref o) if o.len() == 1 && o.contains_key("$ref") => {
                let path = o.get("$ref").unwrap().as_str().unwrap().to_string();
                callback(value, path)
            }
            Value::Object(o) => {
                for (_, mut value) in o {
                    _walk(value, callback)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    _walk(value, &callback)
}

#[derive(Debug)]
struct PathWithAnchor {
    path: String,
    anchor: Option<String>,
}

impl std::fmt::Display for PathWithAnchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)?;
        if let Some(anchor) = &self.anchor {
            write!(f, "#{}", anchor)?;
        }
        Ok(())
    }
}

impl FromStr for PathWithAnchor {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut strs = s.splitn(2, '#')
            .map(|s| s.to_string());
        Ok(Self {
            path: strs.next().unwrap(),
            anchor: strs.next(),
        })
    }
}

fn resolve_json_path(mut val: Value, path: &str) -> Result<Value> {
    let mut split = path.split('/').skip(1);
    for key in split {
        match val {
            Value::Object(ref mut o) => {
                val = o.remove(key).ok_or_else(|| anyhow!("Failed to resolve JSON Pointer, key not found: {}, {}", path, key))?;
            }
            Value::Array(ref mut a) => {
                let index = key.parse::<usize>()?;
                if index >= a.len() {
                    return Err(anyhow!("Failed to resolve JSON Pointer, index out of range: {}, {}", path, key));
                }
                val = a.remove(index);
            }
            _ => {
                return Err(anyhow!("not an object or array"));
            }
        }
    }
    Ok(val)
}


//components/schemas
// schemas
//components/responses
// components/parameters
fn reroute_refs(mut val: Value) -> Result<Value> {
    walk(&mut val, |value, path| {
        let path = PathWithAnchor::from_str(&path).unwrap();
        let obj_type = Path::new(&path.path).file_stem().unwrap().to_str().unwrap(); // schemas, properties, etc.
        let obj_name = &path.anchor.unwrap()[1..];
        *value = json!({
            "$ref": Value::String(format!("#/components/{}/{}", obj_type, obj_name))
        });
    })?;
    Ok(val)
}

#[derive(Debug, Args)]
pub struct Resolve {
    pub path: String,
}

impl Resolve {
    pub fn run(self) -> Result<()> {
        let path = PathBuf::from(&self.path).canonicalize().unwrap();
        let oa_dir = path.parent().unwrap();
        let mut doc = serde_json::from_reader::<_, Value>(std::fs::File::open(&path).unwrap()).unwrap();

        walk(&mut doc, |val, path| {
            let path = PathWithAnchor::from_str(&path).unwrap();
            let child_doc = serde_json::from_reader::<_, Value>(std::fs::File::open(oa_dir.join(&path.path)).unwrap()).unwrap();
            if path.anchor.is_some() {
                let anchor = path.anchor.unwrap();
                let child_doc = resolve_json_path(child_doc, &anchor).unwrap();
                let child_doc = reroute_refs(child_doc).unwrap();
                *val = child_doc;
            } else {
                let child_doc = reroute_refs(child_doc).unwrap();
                *val = child_doc;
            }
        }).unwrap();
        println!("{}", serde_yaml::to_string(&doc).unwrap());
        Ok(())
    }
}