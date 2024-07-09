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

    /// Start webserver
    #[cfg(feature = "web_server")]
    #[arg(long, default_value = "false")]
    pub spawn_server: bool,

    #[cfg(feature = "web_server")]
    #[arg(long, default_value = "8080")]
    pub server_port: u32,
}
