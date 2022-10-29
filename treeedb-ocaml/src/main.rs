use anyhow::Result;

fn main() -> Result<()> {
    treeedb::cli::main(tree_sitter_ocaml::language_ocaml())
}
