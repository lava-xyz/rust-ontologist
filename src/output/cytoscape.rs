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

pub fn from_ir(packages: impl Iterator<Item = Package>) -> Repr {
    let mut elements = vec![];

    for package in packages {
        gen_package(&mut elements, package);
    }

    Repr { elements: remove_invalid_edges(&elements) }
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

fn gen_package(elements: &mut Vec<Element>, package: Package) {
    let package_name = &package.name;
    log::trace!("Generating package {package_name}.");

    elements.push(Element {
        data: Data::new_vertex(&package.name, &package.name, ""),
        classes: "l-package".to_owned(),
    });

    for crate_ in &package.crates {
        let crate_name = &crate_.name;
        log::trace!("Generating crate {crate_name}.");

        let crate_id = format!("{package_name}::{crate_name}");
        gen_vertex(elements, "crate", crate_name, package_name);
        gen_module(elements, crate_, &crate_id);
    }
}

fn gen_module(elements: &mut Vec<Element>, module: &Mod, parent: &str) {
    for item in &module.items.mods {
        let name = &item.name;
        gen_vertex(elements, "mod", name, parent);
        gen_module(elements, item, &format!("{parent}::{name}"));
    }
    for item in &module.items.consts {
        let name = &item.name;
        gen_vertex(elements, "const", name, parent);
    }
    for item in &module.items.enums {
        let name = &item.name;
        gen_vertex(elements, "enum", name, parent);
    }
    for item in &module.items.fns {
        let name = &item.name;
        gen_vertex(elements, "fn", name, parent);
    }
    for item in &module.items.statics {
        let name = &item.name;
        gen_vertex(elements, "static", name, parent);
    }
    for item in &module.items.structs {
        let name = &item.name;
        gen_vertex(elements, "struct", name, parent);
    }
    for item in &module.items.traits {
        let name = &item.name;
        gen_vertex(elements, "trait", name, parent);
    }
    for item in &module.items.trait_aliases {
        let name = &item.name;
        gen_vertex(elements, "trait", name, parent);
    }
    for item in &module.items.types {
        let name = &item.name;
        gen_vertex(elements, "type", name, parent);
    }
    for item in &module.items.unions {
        let name = &item.name;
        gen_vertex(elements, "union", name, parent);
    }
    for dep in &module.deps {
        gen_edge(elements, parent, dep);
    }
    // TODO: uses.
}

fn gen_vertex(
    elements: &mut Vec<Element>,
    name_prefix: &str,
    name: impl Into<String>,
    parent: impl Into<String>,
) {
    let name = name.into();
    let parent = parent.into();

    elements.push(Element {
        data: Data::new_vertex(
            format!("{parent}::{name}"),
            format!("{name_prefix} {name}"),
            parent,
        ),
        classes: "l".to_owned(),
    });
}

fn gen_edge(elements: &mut Vec<Element>, source: impl Into<String>, target: impl Into<String>) {
    let source = source.into();
    let target = target.into();

    elements.push(Element {
        data: Data::new_edge(format!("{source}-{target}"), source, target),
        classes: "l".to_owned(),
    });
}
