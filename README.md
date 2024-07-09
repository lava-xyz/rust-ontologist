# `rust-ontologist`

Navigating through a big codebase is hard, especially when you are just learning it. IDEs make it simpler, but they do not show the "real" project structure, with all inter-module dependencies and such. `rust-ontologist` comes to the rescue: it generates a structure, or _ontology_, of any Rust project, and visualizes it in an interactive browser window.

`rust-ontologist` is fast enough to handle a project of any scale, including the Rust compiler itself, which we demonstrate below.

## Usage

Clone the repository:

```sh
$ git clone https://github.com/lava-xyz/rust-ontologist.git
$ cd rust-ontologist
$ cargo install --path .
```

Make sure you have `$HOME/.cargo/bin` in your `$PATH`.

### Commands available

```sh
$ rust-ontologist --help

Usage: rust-ontologist [OPTIONS] <COMMAND>

Commands:
  dump
  serve
  help   Print this message or the help of the given subcommand(s)

Options:
  -p, --proj <PROJ>   The cargo project root directory [default: .]
      --enable-edges  Enable edges in the output dump (experimental)
  -h, --help          Print help
  -V, --version       Print version
```

Run the `dump` command to to generate a _project structure dump_ in JSON:

```sh
Usage: rust-ontologist dump [OPTIONS]

Options:
  -o, --output <OUTPUT>  [default: codebase-dump.json]
  -h, --help             Print help
```

Run the `serve` to generate and serve the visualization on port `:8080`:

```sh
Usage: rust-ontologist serve [OPTIONS]

Options:
      --server-port <SERVER_PORT>  [default: 8080]
  -h, --help                       Print help
```

## Gallery

To enable coloured edges, provide the flag `--enable-edges`. Note that not all module dependencies are shown at the moment.

### [`rust-bitcoin/bitcoin`] (edges enabled)

![rust-bitcoin/bitcoin](./media/bitcoin.jpg)

[`rust-bitcoin/bitcoin`]: https://github.com/rust-bitcoin/rust-bitcoin/tree/master/bitcoin

### [`actix-web/actix-web`] (edges enabled)

![actix-web/actix-web](./media/actix-web.jpg)

[`actix-web/actix-web`]: https://github.com/actix/actix-web/tree/master/actix-web

### [`tokio`] (edges disabled)

![tokio](./media/tokio.jpg)

[`tokio`]: https://github.com/tokio-rs/tokio

### [`rust`] (edges enabled)

![rust](./media/rust.jpg)

[`rust`]: https://github.com/rust-lang/rust

## Contributing

Just fork the repository, work in your own branch, and open a pull request on `master`. When submitting changes, please prefer rebasing the branch to keep the commit history as clean as possible.

## Release procedure

 1. Update the `version` field in `Cargo.toml`.
 1. Update `CHANGELOG.md`.
 1. Release the project in [GitHub Releases].

[GitHub Releases]: https://github.com/lava-xyz/rust-ontologist/releases

## TODOs

 - Think about packaging `rust-ontologist` on crates.io. We have this `index.html` file, what to do with it?
 - Show more information in the graph: comments, types, etc. We need hide/unhide functionality for this.
 - Implement various code quality metrics based on graph manipulation.
 - Automatically suggest changes that are likely to improve a project's architecture.
 - Integrate with Git and GitHub: add a UI for commits, issues, PRs, show exact code locations of items, etc.
 - Experiment with nicer-looking but fast UI.
