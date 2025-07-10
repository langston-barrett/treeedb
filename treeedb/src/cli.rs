use std::fs;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{Context, Result};
use clap::Parser;
use tree_sitter::Tree;

#[derive(clap::ValueEnum, Debug, Default, Clone, PartialEq, Eq)]
pub enum OnParseError {
    Ignore,
    #[default]
    Warn,
    Error,
}

impl std::fmt::Display for OnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnParseError::Ignore => write!(f, "ignore"),
            OnParseError::Warn => write!(f, "warn"),
            OnParseError::Error => write!(f, "error"),
        }
    }
}

fn handle_parse_errors(path: &str, tree: &Tree, on_parse_error: &OnParseError) {
    let node = tree.root_node();
    match on_parse_error {
        OnParseError::Ignore => (),
        OnParseError::Warn if !node.has_error() => (),
        OnParseError::Error if !node.has_error() => (),
        OnParseError::Warn => {
            eprintln!("[warn] Parse error in {path}");
        }
        OnParseError::Error => {
            eprintln!("[error] Parse error in {path}");
            process::exit(1);
        }
    }
}

/// Generate Datalog facts from source code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Behavior on parse errors
    #[arg(long, default_value_t = OnParseError::Warn, value_name = "CHOICE")]
    on_parse_error: OnParseError,

    /// Output directory
    #[arg(short, long, default_value = ".", value_name = "OUT_DIR")]
    pub output_directory: PathBuf,

    /// Source code to consume; if empty, parse from stdin
    #[arg(value_name = "SRC_FILE")]
    pub source_files: Vec<String>,
}

fn read_file(file: &str) -> Result<String> {
    fs::read_to_string(file).with_context(|| format!("Failed to read file {file}"))
}

fn parse(language: tree_sitter::Language, code: &str) -> Result<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(language)
        .context("Failed to set tree-sitter parser language")?;
    parser.parse(code, None).context("Failed to parse code")
}

fn stdin_string() -> Result<String> {
    let mut stdin_str: String = String::new();
    io::stdin().read_to_string(&mut stdin_str)?;
    Ok(stdin_str)
}

fn create_consumer(output_directory: &Path) -> Result<super::wide::WideCsvConsumer> {
    // TODO(lb): Create consumer based on config
    // For now, just use the wide CSV consumer as the default
    Ok(super::wide::WideCsvConsumer::new(
        output_directory.join("node.csv"),
        output_directory.join("field.csv"),
        output_directory.join("child.csv"),
    )?)
}

pub fn main(language: tree_sitter::Language) -> Result<()> {
    let args = Args::parse();
    let mut fc = create_consumer(&args.output_directory)?;
    if args.source_files.is_empty() {
        let content = stdin_string()?;
        let tree = parse(language, &content)?;
        handle_parse_errors("<stdin>", &tree, &args.on_parse_error);
        super::facts(&mut fc, content.as_bytes(), tree).unwrap();
    }
    for path in args.source_files {
        let content = read_file(&path)?;
        let tree = parse(language, &content)?;
        handle_parse_errors(&path, &tree, &args.on_parse_error);
        super::facts(&mut fc, content.as_bytes(), tree).unwrap();
    }
    Ok(())
}
