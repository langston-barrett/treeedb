use anyhow::{Context, Result};
use clap::Parser;

/// Generate Soufflé types and relations from tree-sitter grammars
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Emit .printsize directives for each AST relation
    #[arg(long, default_value_t = false)]
    pub printsize: bool,

    /// Output file
    #[arg(short, long, default_value = None)]
    pub output: Option<String>,

    /// Prefix for generated declarations
    #[arg(short, long, default_value = None)]
    pub prefix: Option<String>,
}

pub fn main(node_types: &str) -> Result<()> {
    let args = Args::parse();
    let config = super::GenConfig {
        printsize: args.printsize,
        prefix: args.prefix,
    };
    if let Some(path) = args.output {
        let mut file = std::fs::File::create(&path)
            .with_context(|| format!("Failed to write to file {}", path))?;
        super::r#gen(&config, &mut file, node_types)?;
    } else {
        // https://nnethercote.github.io/perf-book/io.html#locking
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        super::r#gen(&config, &mut lock, node_types)?;
    }
    Ok(())
}
