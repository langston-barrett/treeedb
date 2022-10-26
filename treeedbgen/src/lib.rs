use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// node-types.json
#[derive(Serialize, Deserialize)]
pub struct Node {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub ty: String,
    pub named: bool,
    #[serde(default)] // empty
    pub fields: HashMap<String, Field>,
    #[serde(default)] // empty
    pub subtypes: Vec<Subtype>,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    multiple: bool,
    required: bool,
    types: Vec<Subtype>,
}

#[derive(Serialize, Deserialize)]
pub struct Subtype {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    ty: String,
    named: bool,
}

pub fn nodes(node_types_json_str: &str) -> Result<Vec<Node>, serde_json::Error> {
    let r: Vec<Node> = serde_json::from_str(node_types_json_str)?;
    Ok(r)
}
