mod cli;
mod crutches;
mod ir;
mod manifest;
mod output;
mod syn_util;
mod template;
mod traverser;
mod web_server;

use crate::cli::App;
use crate::manifest::Manifest;
use clap::Parser;
use output::cytoscape;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let args = App::parse();

    let manifest = Manifest::parse(&args.global_opts.proj)?;
    let ir = traverser::traverse(&args.global_opts, &manifest)?;
    let cytoscape_repr = cytoscape::from_ir(ir);
    let payload =
        serde_json::to_string_pretty(&cytoscape_repr).expect("Failed to pretty-print JSON");

    match args.command {
        cli::Command::Dump(opts) => {
            std::fs::write(&opts.output, payload)?;
            log::info!("The codebase is successfully dumped to {}.", opts.output);
        }
        cli::Command::Serve(opts) => {
            log::info!("Listening on port :{}", opts.server_port);

            let data = template::render_index(
                manifest.package.expect("package name").name,
                &cytoscape_repr,
            )?;

            web_server::serve(data, opts.server_port);

            log::info!("Shutting down server");
        }
    }

    Ok(())
}
