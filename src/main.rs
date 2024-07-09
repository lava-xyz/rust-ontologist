mod cli;
mod crutches;
mod ir;
mod manifest;
mod output;
mod syn_util;
mod traverser;
#[cfg(feature = "web_server")]
mod web_server;

use clap::Parser;
use output::cytoscape;

use crate::manifest::Manifest;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    pretty_env_logger::init();

    let manifest = Manifest::parse(&args.proj)?;

    let ir = traverser::traverse(&args, &manifest)?;
    let cytoscape_repr = cytoscape::from_ir(ir);
    std::fs::write(
        &args.output,
        serde_json::to_string_pretty(&cytoscape_repr).expect("Failed to pretty-print JSON"),
    )?;
    log::info!("The codebase is successfully dumped to {}.", args.output);

    #[cfg(feature = "web_server")]
    if args.spawn_server {
        log::info!("Server starting at port :8080");
        web_server::serve();
        log::info!("Shutting down server");
    }

    Ok(())
}
