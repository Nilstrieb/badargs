use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CliOption {
    Flag(bool),
    Value(String),
}

pub trait CliArg {
    type Content;
}

#[derive(Debug, Clone, Default)]
pub struct Template {
    pub options: HashMap<Box<dyn Any>, CliArgInfo>,
}

pub struct CliArgInfo {
    pub name: String,
    pub allow_short: bool,
    pub takes_value: bool,
}

#[derive(Debug, Clone, Default)]
pub struct BadArgs {
    options: HashMap<TypeId, CliArgInfo>,
}

impl BadArgs {
    pub fn get<T: Default>(&self) -> Option<&CliArgInfo> {
        self.options.get(&T::type_id())
    }
}

pub fn badargs(template: Template) -> BadArgs {
    let options = template
        .options
        .into_iter()
        .map(|(key, value)| (key.type_id(), value))
        .collect();
    BadArgs { options }
}
