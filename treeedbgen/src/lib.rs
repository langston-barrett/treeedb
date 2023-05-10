use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// node-types.json
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub ty: String,
    pub named: bool,
    #[serde(default)] // empty
    pub fields: HashMap<String, Field>,
    #[serde(default)] // empty
    pub subtypes: Vec<Subtype>,
    #[serde(default)] // empty
    pub children: Option<Field>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub multiple: bool,
    pub required: bool,
    pub types: Vec<Subtype>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subtype {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub ty: String,
    pub named: bool,
}

pub fn nodes(node_types_json_str: &str) -> Result<Vec<Node>, serde_json::Error> {
    let r: Vec<Node> = serde_json::from_str(node_types_json_str)?;
    Ok(r)
}
