// TODO(lb): Refactor this horrible mess!
// TODO(#14): Support "narrow" relational schema
// TODO(#17): Configurable case conventions
// TODO(lb): Optional extra spaces

use std::io;
use std::io::Write;

use heck::ToUpperCamelCase;
use thiserror::Error;
use treeedbgen::Node;

// TODO(lb): Add indices for each field
fn node_with_fields(
    config: &PrivGenConfig,
    w: &mut impl Write,
    node: &Node,
) -> Result<String, io::Error> {
    let rel_name = match node.ty.as_str() {
        "true" => "true_literal",
        "false" => "false_literal",
        "nil" => "nil_literal",
        _ => &node.ty,
    };
    let type_name = node.ty.to_upper_camel_case();
    writeln!(w, ".type {}{} <: symbol", config.type_prefix, type_name)?;
    writeln!(
        w,
        ".decl {}{}(x: {}{})",
        config.relation_prefix, rel_name, config.type_prefix, type_name
    )?;
    if config.printsize {
        writeln!(w, ".printsize {}{}", config.relation_prefix, rel_name)?;
    }
    writeln!(
        w,
        "{}{}(as(x, {}{})) :- {}node(x, \"{}\", _, _, _, _, _, _, _, _, _, _, _).",
        config.relation_prefix,
        rel_name,
        config.type_prefix,
        type_name,
        config.relation_prefix,
        node.ty
    )?;

    for (field_name, field) in &node.fields {
        // Union type of all types this field could be
        let mut named_types: Vec<_> = field.types.iter().filter(|t| t.named).collect();
        let field_type_name: String;
        if named_types.is_empty() {
            // TODO(lb)
            continue;
        } else if named_types.len() == 1 {
            let t = named_types.pop().unwrap();
            field_type_name = format!("{}{}", config.type_prefix, t.ty.to_upper_camel_case());
        } else {
            field_type_name = format!(
                "{}Field{}{}",
                config.type_prefix,
                node.ty.to_upper_camel_case(),
                field_name.to_upper_camel_case(),
            );
            write!(w, ".type {field_type_name} = ")?;
            let mut types = Vec::new();
            for t in &field.types {
                if t.named {
                    types.push(format!(
                        "{}{}",
                        config.type_prefix,
                        t.ty.to_upper_camel_case()
                    ));
                }
            }
            writeln!(w, "{}", types.join(" | "))?;
        }

        // TODO(#18): Configurable field prefix/suffix
        let field_relation_name =
            format!("{}{}_{}_f", config.relation_prefix, &node.ty, field_name);
        writeln!(
            w,
            ".decl {}(x: {}{}, y: {})",
            field_relation_name, config.type_prefix, type_name, field_type_name
        )?;
        // writeln!(w, ".output {}(IO=stdout)", field_relation_name)?;
        writeln!(
            w,
            "{}(x, as(y, {})) :- {}{}(x), {}field(x, \"{}\", y).",
            field_relation_name,
            field_type_name,
            config.relation_prefix,
            rel_name,
            config.relation_prefix,
            field_name,
        )?;
    }

    if let Some(children) = &node.children {
        for child in &children.types {
            let child_type_name =
                format!("{}{}", config.type_prefix, child.ty.to_upper_camel_case());
            let child_relation_name =
                format!("{}{}_{}_c", config.relation_prefix, &node.ty, child.ty);
            writeln!(
                w,
                ".decl {}(x: {}{}, y: {})",
                child_relation_name, config.type_prefix, type_name, child_type_name
            )?;
            writeln!(
                w,
                "{}(x, as(y, {})) :- {}{}(x), {}child(x, _, y).",
                child_relation_name,
                child_type_name,
                config.relation_prefix,
                rel_name,
                config.relation_prefix,
            )?;
        }
    }

    Ok(format!("{}{}", config.type_prefix, type_name))
}

fn node_with_subtypes(
    config: &PrivGenConfig,
    w: &mut impl Write,
    node: &Node,
) -> Result<(), io::Error> {

    let named_subtypes = node.subtypes.iter().filter(|t| t.named).collect::<Vec<_>>();
    if named_subtypes.is_empty() {
        return Ok(());
    }

    write!(
        w,
        ".type {}{} = ",
        config.type_prefix,
        node.ty.to_upper_camel_case()
    )?;

    let mut types = Vec::new();
    for subtype in named_subtypes {
        types.push(format!(
            "{}{}",
            config.type_prefix,
            subtype.ty.to_upper_camel_case()
        ));
    }
    writeln!(w, "{}", types.join(" | "))?;
    Ok(())
}

fn gen_nodes(
    config: &PrivGenConfig,
    w: &mut impl Write,
    nodes: Vec<Node>,
) -> Result<Vec<String>, io::Error> {
    let mut types = Vec::new();
    for node in &nodes {
        if node.named {
            assert!(node.subtypes.is_empty() || node.fields.is_empty());
            if !node.subtypes.is_empty() {
                node_with_subtypes(config, w, node)?;
            } else {
                types.push(node_with_fields(config, w, node)?);
            }
            writeln!(w)?;
        }
    }
    Ok(types)
}

#[derive(Error, Debug)]
pub enum GenError {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("JSON parsing error")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct GenConfig {
    pub printsize: bool,
    pub prefix: Option<String>,
}

struct PrivGenConfig {
    printsize: bool,
    relation_prefix: String,
    type_prefix: String,
}

impl PrivGenConfig {
    fn new(config: &GenConfig) -> Self {
        PrivGenConfig {
            printsize: config.printsize,
            relation_prefix: if let Some(pfx) = &config.prefix {
                format!("{pfx}_")
            } else {
                "".to_owned()
            },
            type_prefix: config
                .prefix
                .clone()
                .unwrap_or_default()
                .to_upper_camel_case(),
        }
    }
}

fn declare_node(config: &PrivGenConfig, w: &mut impl Write) -> Result<(), GenError> {
    writeln!(w, ".type {}NodeKind <: symbol", config.type_prefix)?;
    writeln!(w, ".type {}IsNamed <: symbol", config.type_prefix)?;
    writeln!(w, ".type {}IsError <: symbol", config.type_prefix)?;
    writeln!(w, ".type {}IsExtra <: symbol", config.type_prefix)?;
    writeln!(w, ".type {}IsMissing <: symbol", config.type_prefix)?;
    writeln!(w, ".type {}StartByte <: number", config.type_prefix)?;
    writeln!(w, ".type {}EndByte <: number", config.type_prefix)?;
    writeln!(w, ".type {}StartRow <: number", config.type_prefix)?;
    writeln!(w, ".type {}StartCol <: number", config.type_prefix)?;
    writeln!(w, ".type {}EndRow <: number", config.type_prefix)?;
    writeln!(w, ".type {}EndCol <: number", config.type_prefix)?;
    writeln!(w, ".type {}NodeText <: symbol", config.type_prefix)?;
    writeln!(w, ".type {}NodeIndex <: number", config.type_prefix)?;
    writeln!(
        w,
        ".decl {}node({})",
        config.relation_prefix,
        vec![
            format!("id: {}Node", config.type_prefix),
            format!("kind: {}NodeKind", config.type_prefix),
            format!("is_named: {}IsNamed", config.type_prefix),
            format!("is_extra: {}IsExtra", config.type_prefix),
            format!("is_error: {}IsError", config.type_prefix),
            format!("is_missing: {}IsMissing", config.type_prefix),
            format!("start_byte: {}StartByte", config.type_prefix),
            format!("end_byte: {}EndByte", config.type_prefix),
            format!("start_row: {}StartRow", config.type_prefix),
            format!("start_col: {}StartCol", config.type_prefix),
            format!("end_row: {}EndRow", config.type_prefix),
            format!("end_col: {}EndCol", config.type_prefix),
            format!("text: {}NodeText", config.type_prefix),
        ]
        .join(", ")
    )?;
    // TODO(#20): Other inline relations like this one
    writeln!(
        w,
        ".decl {}node_text(x: {}Node, y: {}NodeText) inline",
        config.relation_prefix, config.type_prefix, config.type_prefix,
    )?;
    writeln!(
        w,
        "{}node_text(x, y) :- {}node(x, _, _, _, _, _, _, _, _, _, _, _, y).",
        config.relation_prefix, config.relation_prefix,
    )?;
    writeln!(
        w,
        ".input {}node(IO=file, filename=\"node.csv\", rfc4180=true)",
        config.relation_prefix,
    )?;
    if config.printsize {
        writeln!(w, ".printsize {}node", config.relation_prefix)?;
    }
    Ok(())
}

fn declare_field(config: &PrivGenConfig, w: &mut impl Write) -> Result<(), GenError> {
    writeln!(w, ".type {}GrammarFieldName <: symbol", config.type_prefix)?;
    writeln!(
        w,
        ".decl {}field({})",
        config.relation_prefix,
        [
            format!("parent: {}Node", config.type_prefix),
            format!("name: {}GrammarFieldName", config.type_prefix),
            format!("child: {}Node", config.type_prefix),
        ]
        .join(", ")
    )?;
    writeln!(
        w,
        ".input {}field(IO=file, filename=\"field.csv\", rfc4180=true)",
        config.relation_prefix,
    )?;
    if config.printsize {
        writeln!(w, ".printsize {}field", config.relation_prefix)?;
        // writeln!(w, ".output {}field(IO=stdout)", config.relation_prefix)?;
    }
    Ok(())
}

fn declare_child(config: &PrivGenConfig, w: &mut impl Write) -> Result<(), GenError> {
    writeln!(
        w,
        ".decl {}child({})",
        config.relation_prefix,
        [
            format!("parent: {}Node", config.type_prefix),
            format!("index: {}NodeIndex", config.type_prefix),
            format!("child: {}Node", config.type_prefix),
        ]
        .join(", ")
    )?;
    writeln!(
        w,
        ".input {}child(IO=file, filename=\"child.csv\", rfc4180=true)",
        config.relation_prefix,
    )?;
    if config.printsize {
        writeln!(w, ".printsize {}child", config.relation_prefix)?;
    }
    Ok(())
}

pub fn r#gen(
    config: &GenConfig,
    w: &mut impl Write,
    node_types_json_str: &str,
) -> Result<(), GenError> {
    writeln!(
        w,
        "// NOTE: This file was generated by treeedb v{}. Do not edit!",
        env!("CARGO_PKG_VERSION")
    )?;
    let config = PrivGenConfig::new(config);
    let types = gen_nodes(&config, w, treeedbgen::nodes(node_types_json_str)?)?;

    writeln!(
        w,
        ".type {}Node = {}",
        config.type_prefix,
        types.join(" | ")
    )?;

    declare_node(&config, w)?;
    declare_field(&config, w)?;
    declare_child(&config, w)?;

    Ok(())
}
