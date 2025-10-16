use anyhow::Result;

fn language() -> tree_sitter::Language {
    let ptr = tree_sitter_crystal::LANGUAGE.into_raw();
    unsafe { std::mem::transmute(ptr()) }
}

fn main() -> Result<()> {
    treeedb::cli::main(language())
}
