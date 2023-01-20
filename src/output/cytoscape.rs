use displaydoc::Display;
use serde::Serialize;

use crate::ir::{Mod, Package};

#[derive(Serialize)]
#[serde(transparent)]
pub struct Repr {
    pub elements: Vec<Element>,
}

#[derive(Clone, Serialize)]
pub struct Element {
    pub data: Data,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub classes: String,
}

#[derive(Clone, Serialize)]
pub struct Data {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub source: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub target: String,
}

impl Data {
    fn new_vertex(
        id: impl Into<String>,
        name: impl Into<String>,
        parent: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            parent: parent.into(),
            source: "".to_owned(),
            target: "".to_owned(),
        }
    }

    fn new_edge(
        id: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: "".to_owned(),
            parent: "".to_owned(),
            source: source.into(),
            target: target.into(),
        }
    }

    fn is_edge(&self) -> bool {
        !self.source.is_empty() && !self.target.is_empty()
    }
}

struct Ctx {
    elements: Vec<Element>,
    color_gen: ColorGenerator,
}

#[derive(Default)]
struct ColorGenerator {
    current: EdgeColor,
    i: usize,
}

#[derive(Default, Clone, Copy, Display)]
enum EdgeColor {
    /// red
    #[default]
    Red,
    /// green
    Green,
    /// blue
    Blue,
    /// violet
    Violet,
    /// orange
    Orange,
    /// purple
    Purple,
    /// plum
    Plum,
}

impl ColorGenerator {
    fn update(&mut self) {
        use EdgeColor::*;
        self.current = match self.i % 7 {
            0 => Red,
            1 => Green,
            2 => Blue,
            3 => Violet,
            4 => Orange,
            5 => Purple,
            _ => Plum,
        };
        self.i += 1;
    }
}

pub fn from_ir(packages: impl Iterator<Item = Package>) -> Repr {
    let mut ctx = Ctx { elements: vec![], color_gen: Default::default() };

    for package in packages {
        gen_package(&mut ctx, package);
    }

    Repr { elements: remove_invalid_edges(&ctx.elements) }
}

// Removes edges that point to non-existent vertices. This might happen if the
// traverser encounters paths that it cannot "resolve".
fn remove_invalid_edges(elements: &[Element]) -> Vec<Element> {
    elements
        .iter()
        .filter(|elem| {
            if elem.data.is_edge() {
                let source = &elem.data.source;
                let target = &elem.data.target;

                return elements.iter().any(|elem| elem.data.id == *source)
                    && elements.iter().any(|elem| elem.data.id == *target);
            }

            true
        })
        .cloned()
        .collect()
}

fn gen_package(ctx: &mut Ctx, package: Package) {
    let package_name = &package.name;
    log::trace!("Generating package {package_name}.");

    ctx.elements.push(Element {
        data: Data::new_vertex(&package.name, &package.name, ""),
        classes: "vertex-package".to_owned(),
    });

    for crate_ in &package.crates {
        let crate_name = &crate_.name;
        log::trace!("Generating crate {crate_name}.");

        let crate_id = format!("{package_name}::{crate_name}");
        gen_vertex(ctx, "crate", crate_name, package_name);
        gen_module(ctx, crate_, &crate_id);
        ctx.color_gen.update();
        log::trace!("{} for {}", ctx.color_gen.current, crate_id);
    }
}

fn gen_module(ctx: &mut Ctx, module: &Mod, parent: &str) {
    for item in &module.items.mods {
        let name = &item.name;
        gen_vertex(ctx, "mod", name, parent);
        gen_module(ctx, item, &format!("{parent}::{name}"));
    }
    for item in &module.items.consts {
        let name = &item.name;
        gen_vertex(ctx, "const", name, parent);
    }
    for item in &module.items.enums {
        let name = &item.name;
        gen_vertex(ctx, "enum", name, parent);
    }
    for item in &module.items.fns {
        let name = &item.name;
        gen_vertex(ctx, "fn", name, parent);
    }
    for item in &module.items.statics {
        let name = &item.name;
        gen_vertex(ctx, "static", name, parent);
    }
    for item in &module.items.structs {
        let name = &item.name;
        gen_vertex(ctx, "struct", name, parent);
    }
    for item in &module.items.traits {
        let name = &item.name;
        gen_vertex(ctx, "trait", name, parent);
    }
    for item in &module.items.trait_aliases {
        let name = &item.name;
        gen_vertex(ctx, "trait", name, parent);
    }
    for item in &module.items.types {
        let name = &item.name;
        gen_vertex(ctx, "type", name, parent);
    }
    for item in &module.items.unions {
        let name = &item.name;
        gen_vertex(ctx, "union", name, parent);
    }
    for dep in &module.deps {
        gen_edge(ctx, parent, dep);
    }
    // TODO: uses.
}

fn gen_vertex(ctx: &mut Ctx, kind: &str, name: impl Into<String>, parent: impl Into<String>) {
    let name = name.into();
    let parent = parent.into();

    ctx.elements.push(Element {
        data: Data::new_vertex(format!("{parent}::{name}"), format!("{kind} {name}"), parent),
        classes: format!("vertex-{kind} vertex-non-package"),
    });
}

fn gen_edge(ctx: &mut Ctx, source: impl Into<String>, target: impl Into<String>) {
    let source = source.into();
    let target = target.into();

    let color = ctx.color_gen.current;

    ctx.elements.push(Element {
        data: Data::new_edge(format!("{source}-{target}"), source, target),
        classes: format!("edge-{color}"),
    });
}
