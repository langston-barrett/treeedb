use anyhow::Result;
use clap::Parser;

mod cli;

fn main() -> Result<()> {
    let _args = cli::Args::parse();
    println!("{}", treeedbgen_souffle::gen(tree_sitter_java::NODE_TYPES)?);
    Ok(())
}
