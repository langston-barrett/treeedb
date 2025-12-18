use anyhow::Result;

fn main() -> Result<()> {
    treeedbgen_souffle::cli::main(tree_sitter_javascript::NODE_TYPES)
}
