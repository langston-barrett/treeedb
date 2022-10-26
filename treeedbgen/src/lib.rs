// TODO(lb): Feature to use smart-string
use serde::{Deserialize, Serialize};

/// node-types.json
#[derive(Serialize, Deserialize)]
pub struct Node {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub ty: String,
    pub named: bool,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct Field {}

pub fn nodes(node_types_json_str: &str) -> Result<Vec<Node>, serde_json::Error> {
    let r: Vec<Node> = serde_json::from_str(node_types_json_str)?;
    Ok(r)
}
