//! Utilities related to working with Rust's CST (Concrete Syntax Tree).

use quote::ToTokens;

pub fn is_public_item(item: &syn::Item) -> bool {
    let is_public = |vis| !matches!(vis, &syn::Visibility::Inherited);

    match item {
        syn::Item::Const(item) => is_public(&item.vis),
        syn::Item::Enum(item) => is_public(&item.vis),
        syn::Item::Fn(item) => is_public(&item.vis),
        syn::Item::Mod(item) => is_public(&item.vis),
        syn::Item::Static(item) => is_public(&item.vis),
        syn::Item::Struct(item) => is_public(&item.vis),
        syn::Item::Trait(item) => is_public(&item.vis),
        syn::Item::TraitAlias(item) => is_public(&item.vis),
        syn::Item::Type(item) => is_public(&item.vis),
        syn::Item::Union(item) => is_public(&item.vis),
        syn::Item::Use(item) => is_public(&item.vis),
        _ => false,
    }
}

pub fn format_path(segments: Vec<syn::PathSegment>) -> String {
    syn::Path { leading_colon: None, segments: syn::punctuated::Punctuated::from_iter(segments) }
        .to_token_stream()
        .to_string()
        .replace(' ', "")
}

pub fn path_segment(ident: &str) -> syn::PathSegment {
    syn::PathSegment::from(syn::Ident::new(ident, proc_macro2::Span::call_site()))
}

pub fn flatten_use_tree(tree: &syn::UseTree) -> Vec<syn::Path> {
    match tree {
        syn::UseTree::Path(syn::UsePath { ident, tree, .. }) => {
            let mut uses = flatten_use_tree(tree);
            for use_ in &mut uses {
                use_.segments.insert(
                    0,
                    syn::PathSegment { ident: ident.clone(), arguments: syn::PathArguments::None },
                );
            }
            uses
        }
        syn::UseTree::Group(syn::UseGroup { items, .. }) => {
            items.iter().flat_map(flatten_use_tree).collect()
        }
        syn::UseTree::Name(syn::UseName { ident, .. }) => vec![syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter(vec![syn::PathSegment {
                ident: ident.clone(),
                arguments: syn::PathArguments::None,
            }]),
        }],
        _ => vec![],
    }
}

pub trait PrettyPrint {
    fn pretty_print(self) -> String;
}

impl<T> PrettyPrint for T
where
    T: ToTokens,
{
    fn pretty_print(self) -> String {
        let tokens = self.to_token_stream();
        let item = syn::parse2(tokens.clone()).expect("Must be already validated");
        let file = syn::File { attrs: vec![], items: vec![item], shebang: None };

        // `prettyplease` panics if it fails to pretty-print.
        match std::panic::catch_unwind(|| prettyplease::unparse(&file)) {
            Ok(s) => s.trim().to_string(),
            Err(_e) => {
                log::error!("Failed to pretty-print. Skipping.");
                tokens.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    #[test]
    fn flatten_use_tree() {
        let use_tree = syn::parse_str("use foo::{bar, baz::qux, jar::{a, b, c}};").unwrap();
        let use_tree = match use_tree {
            syn::Item::Use(syn::ItemUse { tree, .. }) => tree,
            _ => unreachable!(),
        };

        let paths = super::flatten_use_tree(&use_tree)
            .into_iter()
            .map(|path| path.to_token_stream().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            paths,
            vec![
                "foo :: bar",
                "foo :: baz :: qux",
                "foo :: jar :: a",
                "foo :: jar :: b",
                "foo :: jar :: c"
            ]
        );
    }
}
