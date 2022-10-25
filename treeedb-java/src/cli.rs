use clap::Parser;

/// Generate Datalog facts from Java source code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Java file(s) to consume; if empty, parse from stdin
    #[arg(value_name = "JAVA_SRC")]
    pub java_files: Vec<String>,
}
