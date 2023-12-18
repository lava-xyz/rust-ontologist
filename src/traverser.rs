use std::path::PathBuf;

use anyhow::anyhow;
use multipipe::Pipe;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rayon::prelude::*;

use crate::{
    ir::{
        Const, Enum, Fn, ItemCollection, Mod, Package, Static, Struct, Trait, TraitAlias, Type,
        Union, Use,
    },
    manifest::Manifest,
    syn_util::{self, PrettyPrint},
};

// A context for traversing a module.
#[derive(Clone)]
struct Ctx<'a> {
    // Command-line options.
    args: &'a crate::cli::Args,
    // The name of the containing package.
    package_name: String,
    // The name of the containing crate.
    crate_name: String,
    // The current working directory.
    dir: PathBuf,
    // The module name under consideration.
    module_name: String,
}

impl<'a> Ctx<'a> {
    fn new(
        args: &'a crate::cli::Args,
        dir: impl Into<PathBuf>,
        module_name: impl Into<String>,
        package_name: impl Into<String>,
        crate_name: impl Into<String>,
    ) -> Self {
        Self {
            args,
            dir: dir.into(),
            module_name: module_name.into(),
            package_name: package_name.into(),
            crate_name: crate_name.into(),
        }
    }
}

pub fn traverse(
    args: &crate::cli::Args,
    manifest: &Manifest,
) -> anyhow::Result<impl Iterator<Item = Package>> {
    manifest
        .members(&args.proj)?
        .par_iter()
        .filter_map(|member| match traverse_member(member, args) {
            Ok(package) => Some(package),
            Err(e) => {
                let member_display = member.display();
                log::debug!("Failed to traverse member {member_display}: {e}. Skipping.");
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .pipe(Ok)
}

// Traverses a workspace member.
fn traverse_member(member: &PathBuf, args: &crate::cli::Args) -> anyhow::Result<Package> {
    let member_display = member.display();
    log::trace!("Traversing member {member_display}.");

    let manifest = Manifest::parse(member)?;
    let package = manifest.package.as_ref().ok_or_else(|| {
        anyhow!(
            "Workspace member {member_display} must be a package (nested workspaces are not \
             supported by Cargo at the moment)."
        )
    })?;
    let package_name = package.name.replace('-', "_");
    let crates = manifest
        .read_package_targets(&member)?
        .map(|target| {
            let ctx = Ctx::new(args, &target.path, &target.name, &package_name, &target.name);
            traverse_mod(&ctx)?
                .ok_or_else(|| anyhow!("Failed to traverse workspace member {member_display}."))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(Package { name: package_name, crates })
}

fn traverse_mod(ctx: &Ctx) -> anyhow::Result<Option<Mod>> {
    let Ctx { args, dir, module_name, package_name, crate_name } = ctx;
    let (mut file, module_path) = match open_file(ctx) {
        Ok((file, module_path)) => (file, module_path),
        Err(e) => {
            log::debug!(
                "Cannot find module {module_name} in {module_dir}: {e}. Skipping.",
                module_dir = dir.display(),
            );
            return Ok(None);
        }
    };
    let parse_tree = read_parse_tree(&mut file)?;
    drop(file);

    log::trace!("Traversing module {}.", module_path.display());

    let dir = if module_name != "main" && module_name != "lib" {
        [dir, &PathBuf::from(module_name)].iter().collect()
    } else {
        dir.clone()
    };
    let ctx = Ctx::new(args, dir, module_name, package_name, crate_name);

    let mut module = Mod::new(module_name);
    traverse_item_vec(&ctx, &mut module.items, &mut module.deps, parse_tree.items)?;
    Ok(Some(module))
}

fn open_file(Ctx { dir, module_name, .. }: &Ctx) -> anyhow::Result<(std::fs::File, PathBuf)> {
    let new_style_path = [dir, &format!("{module_name}.rs").into()].iter().collect();
    let old_style_path = [dir, &format!("{module_name}/mod.rs").into()].iter().collect();

    let (file, module_path) = match std::fs::File::open(&new_style_path) {
        Ok(file) => (file, new_style_path),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            std::fs::File::open(&old_style_path).map(|file| (file, old_style_path))?
        }
        Err(e) => return Err(e.into()),
    };

    Ok((file, module_path))
}

fn read_parse_tree(file: &mut std::fs::File) -> anyhow::Result<syn::File> {
    std::io::read_to_string(file)?.pipe_ref(syn::parse_file)?.pipe(Ok)
}

fn traverse_item_vec(
    ctx: &Ctx,
    acc: &mut ItemCollection,
    deps: &mut Vec<String>,
    items: Vec<syn::Item>,
) -> anyhow::Result<()> {
    for item in items {
        traverse_item(ctx, acc, deps, item)?;
    }
    Ok(())
}

fn traverse_item(
    ctx: &Ctx,
    acc: &mut ItemCollection,
    deps: &mut Vec<String>,
    item: syn::Item,
) -> anyhow::Result<()> {
    // Skip private items, except module declarations.
    if !syn_util::is_public_item(&item)
        && !matches!(item, syn::Item::Mod(_))
        // Used in computing dependencies.
        && !matches!(item, syn::Item::Use(_))
    {
        return Ok(());
    }

    match item {
        syn::Item::Const(item) => {
            let item = syn::ItemConst { attrs: vec![], ..item };
            acc.consts.push(Const { name: item.ident.to_string(), repr: item.pretty_print() });
        }
        syn::Item::Enum(item) => {
            let item = syn::ItemEnum { attrs: vec![], ..item };
            acc.enums.push(Enum { name: item.ident.to_string(), repr: item.pretty_print() })
        }
        syn::Item::Fn(item) => {
            let item = syn::ItemFn { attrs: vec![], ..item };
            let curly_braces = quote! {{}};
            let proper_syntax =
                TokenStream::from_iter([item.sig.to_token_stream(), curly_braces].into_iter());
            acc.fns
                .push(Fn { name: item.sig.ident.to_string(), repr: proper_syntax.pretty_print() });
        }
        syn::Item::Mod(item) => {
            let item = syn::ItemMod { attrs: vec![], ..item };
            if let Some(new_module) = traverse_item_mod(ctx, item)? {
                acc.mods.push(new_module);
            }
        }
        syn::Item::Static(item) => {
            let item = syn::ItemStatic { attrs: vec![], ..item };
            acc.statics.push(Static { name: item.ident.to_string(), repr: item.pretty_print() })
        }
        syn::Item::Struct(item) => {
            let item = syn::ItemStruct { attrs: vec![], ..item };
            acc.structs.push(Struct { name: item.ident.to_string(), repr: item.pretty_print() })
        }
        syn::Item::Trait(item) => {
            let item = syn::ItemTrait { attrs: vec![], ..item };
            acc.traits.push(Trait { name: item.ident.to_string(), repr: item.pretty_print() });
        }
        syn::Item::TraitAlias(item) => {
            let item = syn::ItemTraitAlias { attrs: vec![], ..item };
            acc.trait_aliases
                .push(TraitAlias { name: item.ident.to_string(), repr: item.pretty_print() })
        }
        syn::Item::Type(item) => {
            let item = syn::ItemType { attrs: vec![], ..item };
            acc.types.push(Type { name: item.ident.to_string(), repr: item.pretty_print() });
        }
        syn::Item::Union(item) => {
            let item = syn::ItemUnion { attrs: vec![], ..item };
            acc.unions.push(Union { name: item.ident.to_string(), repr: item.pretty_print() });
        }
        syn::Item::Use(item) => {
            let item = syn::ItemUse { attrs: vec![], ..item };
            if ctx.args.enable_edges {
                deps.append(&mut traverse_item_use(ctx, &item)?)
            };
            acc.uses.push(Use { repr: item.pretty_print() });
        }
        _ => return Ok(()),
    };

    Ok(())
}

fn traverse_item_mod(ctx: &Ctx, item: syn::ItemMod) -> anyhow::Result<Option<Mod>> {
    match item.content {
        // A public module definition: `pub mod foo { ... }`.
        Some((_brace, items)) if syn_util::is_public_item(&item.clone().into()) => {
            let mut new_module = Mod::new(item.ident.to_string());
            traverse_item_vec(ctx, &mut new_module.items, &mut new_module.deps, items)?;
            Ok(Some(new_module))
        }
        // A private module definition: `mod foo { ... }`.
        Some(_) => Ok(None),
        // A module declaration: `mod foo;`.
        None => {
            let module_name = item.ident.to_string();
            Ok(traverse_mod(&Ctx { module_name, ..ctx.clone() })?)
        }
    }
}

fn traverse_item_use(ctx: &Ctx, item: &syn::ItemUse) -> anyhow::Result<Vec<String>> {
    syn_util::flatten_use_tree(&item.tree)
        .into_iter()
        .filter_map(|path| {
            let mut segments = path.segments.into_iter().collect::<Vec<_>>();
            if let Some(first_segment) = segments.first() {
                // The current crate item.
                if first_segment.ident == "crate" {
                    segments.remove(0);
                    return Some(format!(
                        "{}::{}::{}",
                        ctx.package_name,
                        ctx.crate_name,
                        syn_util::format_path(segments),
                    ));
                }

                // Perhaps some other workspace member.
                segments.insert(1, syn_util::path_segment("lib"));
                return Some(syn_util::format_path(segments));
            }

            None
        })
        .collect::<Vec<_>>()
        .pipe(Ok)
}
