use anyhow::Result;

fn main() -> Result<()> {
    treeedb::cli::main(tree_sitter_java::language())
}
