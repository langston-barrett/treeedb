use anyhow::Result;

fn main() -> Result<()> {
    treeedbgen_souffle::cli::main(tree_sitter_ocaml::OCAML_NODE_TYPES)
}
