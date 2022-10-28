use anyhow::Result;

fn main() -> Result<()> {
    treeedbgen_souffle::cli::main(tree_sitter_c::NODE_TYPES)
}
