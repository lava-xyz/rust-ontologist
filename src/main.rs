mod crutches;
mod ir;
mod manifest;
mod output;
mod syn_util;
mod traverser;

use output::cytoscape;

use clap::Parser;

use crate::manifest::Manifest;

/// A Rust codebase visualizer.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The cargo project root directory.
    #[arg(short, long)]
    proj: String,

    /// The name of the output dump file.
    #[arg(short, long, default_value = "codebase-dump.json")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let Args { proj: entry, output } = Args::parse();
    pretty_env_logger::init();

    let manifest = Manifest::parse(&entry)?;

    let ir = traverser::traverse(&entry, &manifest)?;
    let cytoscape_repr = cytoscape::from_ir(ir);
    std::fs::write(
        &output,
        serde_json::to_string_pretty(&cytoscape_repr).expect("Failed to pretty-print JSON"),
    )?;
    log::info!("The codebase is successfully dumped to {output}.");
    Ok(())
}
