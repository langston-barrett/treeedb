pub fn gen(node_types_json_str: &str) -> Result<String, serde_json::Error> {
    let _nodes = treeedbgen::nodes(node_types_json_str)?;
    Ok("TODO".to_string())
}
