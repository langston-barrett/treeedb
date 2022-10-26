use clap::Parser;

/// Generate Datalog facts from Java source code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {}
