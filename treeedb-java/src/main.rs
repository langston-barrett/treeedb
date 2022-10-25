use std::fs;

use anyhow::{Context, Result};
use clap::Parser;

mod cli;

fn read_file(file: &str) -> Result<String> {
    fs::read_to_string(file).with_context(|| format!("Failed to read file {}", file))
}

fn parse(language: tree_sitter::Language, code: &str) -> Result<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(language)
        .context("Failed to set tree-sitter parser language")?;
    parser.parse(code, None).context("Failed to parse code")
}

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let language = tree_sitter_java::language();
    let mut fc = treeedb::wide::WideCsvConsumer::new("node.csv".into(), "field.csv".into())?;
    for path in args.java_files {
        let tree = parse(language, &read_file(&path)?)?;
        treeedb::facts(&mut fc, tree).unwrap();
    }
    Ok(())
}
