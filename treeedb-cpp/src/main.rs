use anyhow::Result;

fn main() -> Result<()> {
    treeedb::cli::main(tree_sitter_cpp::LANGUAGE)
}
