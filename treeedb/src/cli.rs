use std::fs;

use anyhow::{Context, Result};
use clap::Parser;

/// Generate Datalog facts from source code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// TODO(lb): Option for output directory
// TODO(lb): Option to raise an error on a parse failure
pub struct Args {
    /// Source code to consume; if empty, parse from stdin
    #[arg(value_name = "SRC_FILE")]
    pub source_files: Vec<String>,
}

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

pub fn main(language: tree_sitter::Language) -> Result<()> {
    let args = Args::parse();
    let mut fc = super::wide::WideCsvConsumer::new("node.csv".into(), "field.csv".into())?;
    // TODO(lb): read from stdin
    for path in args.source_files {
        let tree = parse(language, &read_file(&path)?)?;
        super::facts(&mut fc, tree).unwrap();
    }
    Ok(())
}
