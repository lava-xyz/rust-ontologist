use clap::Parser;

/// A Rust codebase visualizer.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The cargo project root directory.
    #[arg(short, long)]
    pub proj: String,

    /// The name of the output dump file.
    #[arg(short, long, default_value = "codebase-dump.json")]
    pub output: String,

    /// Enable edges in the output dump (experimental).
    #[arg(long, default_value = "false")]
    pub enable_edges: bool,
}
