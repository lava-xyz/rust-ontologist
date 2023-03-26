//! Working with the [Cargo manifest format].
//!
//! I tried using the `cargo_toml` crate to parse `Cargo.toml` files, but it
//! didn't support workspace inheritance properly.
//!
//! [Cargo manifest format]: https://doc.rust-lang.org/cargo/reference/manifest.html

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::crutches::FlattenResult;

use anyhow::{anyhow, Context};
use glob::glob;
use multipipe::Pipe;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub package: Option<Package>,
    pub lib: Option<Lib>,
    pub workspace: Option<Workspace>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Lib {
    pub name: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

pub struct Target {
    pub name: String,
    pub path: PathBuf,
}

impl Target {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self { name: name.into(), path: path.into() }
    }
}

impl Manifest {
    pub fn parse(entry: impl AsRef<Path>) -> anyhow::Result<Manifest> {
        let path: PathBuf = [entry.as_ref(), &PathBuf::from("Cargo.toml")].iter().collect();

        std::fs::read_to_string(&path)
            .with_context(|| format!("Cannot open {}", path.display()))?
            .pipe_ref(toml::from_str::<Self>)
            .with_context(|| format!("Cannot parse {}", path.display()))?
            .pipe(Ok)
    }

    /// Reads the [package targets] from the manifest.
    ///
    /// Currently, it supports only:
    ///  - Hardcoded `src/main.rs` and `src/lib.rs`, if they do exist.
    ///  - A custom `[lib]` path specified in the manifest file.
    ///
    /// [package targets]: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
    pub fn read_package_targets(
        &self,
        member: impl AsRef<Path>,
    ) -> anyhow::Result<impl Iterator<Item = Target>> {
        let mut targets = vec![];
        self.read_main_target(&mut targets, &member)?;
        self.read_lib_target(&mut targets, &member)?;
        self.read_bin_targets(&mut targets, &member)?;

        Ok(targets.into_iter())
    }

    fn read_main_target(
        &self,
        targets: &mut Vec<Target>,
        member: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        // Always use the default `src/main.rs` if it exists.
        let main_file_path: PathBuf =
            [member.as_ref(), &PathBuf::from("src/main.rs")].iter().collect();
        if main_file_path.exists() {
            targets.push(Target::new("main", package_target_path(main_file_path)?));
        }

        Ok(())
    }

    fn read_lib_target(
        &self,
        targets: &mut Vec<Target>,
        member: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        // If we have a custom path for `[lib]`, we use it. Otherwise, we use the
        // default `src/lib.rs`.
        let lib_file_path: PathBuf = self
            .lib
            .as_ref()
            .and_then(|lib| lib.path.as_ref().map(|path| entry_path(path, &member)))
            .unwrap_or_else(|| [member.as_ref(), &PathBuf::from("src/lib.rs")].iter().collect());

        if lib_file_path.exists() {
            targets.push(Target::new("lib", package_target_path(lib_file_path)?));
        }

        Ok(())
    }

    fn read_bin_targets(
        &self,
        targets: &mut Vec<Target>,
        member: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        //  Walk the `src/bin/` directory and push all the targets from there.
        let _ = (targets, member);

        Ok(())
    }

    /// Returns the list of workspace members plus itself, in case of a package.
    pub fn members(&self, entry: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
        let mut workspace_members = self
            .workspace
            .as_ref()
            .map(|workspace| -> anyhow::Result<_> { workspace.members(&entry) })
            .unwrap_or(Ok(vec![]))?;

        let mut members = vec![];
        members.append(&mut workspace_members);

        if self.package.is_some() {
            members.push(entry.as_ref().to_owned());
        }

        Ok(members)
    }
}

// Constructs a custom manifest path, e.g., in `[lib]`.
fn entry_path(entry: &str, member: impl AsRef<Path>) -> PathBuf {
    let path = PathBuf::from(entry);
    if path.is_absolute() {
        path
    } else {
        [member.as_ref(), &path].iter().collect()
    }
}

// Constructs a package target path from an entry file path.
//
// As an example, `entry_file_path` may be `src/main.rs`, `src/lib.rs`,
// `src/bin/foo.rs`, or `src/bin/foo/main.rs`.
fn package_target_path(entry_file_path: PathBuf) -> anyhow::Result<PathBuf> {
    entry_file_path
        .parent()
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow!("No parent for {}", entry_file_path.display()))
}

impl Workspace {
    /// Returns the list of workspace members.
    pub fn members(&self, entry: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
        let allowed = walk_glob_members(&entry, &self.members)?.into_iter().collect::<HashSet<_>>();
        let excluded =
            walk_glob_members(&entry, &self.exclude)?.into_iter().collect::<HashSet<_>>();
        allowed.difference(&excluded).into_iter().cloned().collect::<Vec<_>>().pipe(Ok)
    }
}

fn walk_glob_members(entry: impl AsRef<Path>, members: &[String]) -> anyhow::Result<Vec<PathBuf>> {
    members
        .iter()
        .map(|member| {
            glob(&format!("{entry}/{member}", entry = entry.as_ref().display()))
                .with_context(|| format!("A pattern error in {member}."))
                .map(|paths| {
                    paths
                        .map(|path| path.with_context(|| format!("Failed to read glob {member}.")))
                        .collect::<anyhow::Result<Vec<_>>>()
                })
                .flatten_result()
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .pipe(Ok)
}
