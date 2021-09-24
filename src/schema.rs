use super::Result;
use crate::{ArgError, CliArg, CliReturnValue};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SchemaKind {
    String,
    OptionString,
    Bool,
    INum,
    UNum,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaCommand {
    short: Option<&'static str>,
    kind: SchemaKind,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Schema {
    commands: HashMap<&'static str, SchemaCommand>,
}

impl Schema {
    pub fn add_command(&mut self, name: &'static str, command: SchemaCommand) -> Result<()> {
        if let Some(_) = self.commands.insert(name, command) {
            Err(ArgError::NameAlreadyExists(name))
        } else {
            Ok(())
        }
    }
}

pub trait IntoSchema {
    fn add_schema(schema: &mut Schema) -> Result<()>;
}

/// Allow using multiple schema values, these tuples can be nested :D
impl<S1, S2> IntoSchema for (S1, S2)
where
    S1: IntoSchema,
    S2: IntoSchema,
{
    fn add_schema(schema: &mut Schema) -> Result<()> {
        S1::add_schema(schema)?;
        S2::add_schema(schema)
    }
}

pub fn parse_schema<S>() -> Result<Schema>
where
    S: IntoSchema,
{
    let mut schema = Schema::default();
    S::add_schema(&mut schema)?;
    Ok(schema)
}

/// Create the Schema from the CliArg type
impl<T> IntoSchema for T
where
    T: CliArg,
{
    fn add_schema(schema: &mut Schema) -> Result<()> {
        let kind = T::Content::kind();
        let name = T::long();
        let short = T::short();
        schema.add_command(name, SchemaCommand { short, kind })
    }
}
