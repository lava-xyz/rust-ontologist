//! The IR (Intermediate Representation) of a project structure.

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Package {
    pub name: String,
    pub crates: Vec<Mod>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Mod {
    pub name: String,
    pub items: ItemCollection,
    pub deps: Vec<String>,
}

impl Mod {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), items: Default::default(), deps: vec![] }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ItemCollection {
    pub consts: Vec<Const>,
    pub enums: Vec<Enum>,
    pub fns: Vec<Fn>,
    pub mods: Vec<Mod>,
    pub statics: Vec<Static>,
    pub structs: Vec<Struct>,
    pub traits: Vec<Trait>,
    pub trait_aliases: Vec<TraitAlias>,
    pub types: Vec<Type>,
    pub unions: Vec<Union>,
    pub uses: Vec<Use>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Const {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Enum {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fn {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Static {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Struct {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Trait {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TraitAlias {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Type {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Union {
    pub name: String,
    pub repr: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Use {
    pub repr: String,
}
