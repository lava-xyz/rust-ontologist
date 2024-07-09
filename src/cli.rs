use clap::{crate_authors, crate_description, crate_name, crate_version, Args, Parser, Subcommand};

/// A Rust codebase visualizer.
#[derive(Parser, Debug, Clone)]
#[command(name = crate_name!())]
#[command(author = crate_authors!())]
#[command(about = crate_description!())]
#[command(version = crate_version!())]
pub struct App {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    Dump(DumpOptions),
    Serve(ServeOptions),
}

#[derive(Args, Clone, Debug)]
pub struct GlobalOpts {
    /// The cargo project root directory.
    #[arg(short, long, default_value = ".")]
    pub proj: String,

    /// Enable edges in the output dump (experimental).
    #[arg(long, default_value = "false")]
    pub enable_edges: bool,
}

#[derive(Args, Clone, Debug)]
pub struct DumpOptions {
    #[arg(short, long, default_value = "codebase-dump.json")]
    pub output: String,
}

#[derive(Args, Clone, Debug)]
pub struct ServeOptions {
    #[arg(long, default_value = "8080")]
    pub server_port: u32,
}
