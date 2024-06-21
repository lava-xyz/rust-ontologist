# rust-ontologist

## Usage

Run the following command to generate a project structure dump:

```
$ cargo run -- -p <your-cargo-project>
```

Run a static server (make sure `http-server` is installed: `npm install http-server`):

```
$ sudo ./scripts/serve.sh
```

Then just open your browser window at `http://localhost/index.html`:

```
$ ./scripts/open.sh
```

## TODOs

 - Implement various code quality metrics based on graph manipulation.
 - Automatically suggest changes that are likely to improve a project's architecture.
 - Integrate with Git and GitHub: add a UI for commits, issues, PRs, show exact code locations of items, etc.
 - Experiment with nicer-looking but fast UI.
