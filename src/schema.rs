//!
//! Generates CLI argument schemas based on generic types
//!
//! This makes the interface of this crate fully type-safe! (and kind of cursed)

use super::Result;
use crate::{CliArg, CliReturnValue, SchemaError};
use std::collections::HashMap;

///
/// The type of value the argument returns
///
/// This could *maybe* also be solved with trait objects but lets keep this for now
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SchemaKind {
    String,
    Bool,
    IInt,
    UInt,
    Num,
}

///
/// A single command in the schema
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct SchemaCommand {
    pub kind: SchemaKind,
    pub long: &'static str,
    pub short: Option<char>,
}

///
/// A runtime representation of the schema type
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Schema {
    longs: HashMap<&'static str, SchemaCommand>,
    shorts: HashMap<char, SchemaCommand>,
}

impl Schema {
    /// Creates the `Schema` from the generic parameter `S`
    pub fn create<S>() -> Result<Self>
    where
        S: IntoSchema,
    {
        let mut schema = Schema::default();
        S::add_schema(&mut schema)?;
        Ok(schema)
    }

    fn add_command(&mut self, long_name: &'static str, command: SchemaCommand) -> Result<()> {
        if self.longs.insert(long_name, command).is_some() {
            Err(SchemaError::NameAlreadyExists(long_name.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn short(&self, name: char) -> Option<&SchemaCommand> {
        self.shorts.get(&name)
    }

    pub fn long(&self, name: &str) -> Option<&SchemaCommand> {
        self.longs.get(name)
    }

    pub fn arguments(&self) -> impl Iterator<Item = &SchemaCommand> {
        self.longs.values()
    }

    fn add_short_command(&mut self, short_name: char, command: SchemaCommand) -> Result<()> {
        if self.shorts.insert(short_name, command).is_some() {
            Err(SchemaError::NameAlreadyExists(short_name.to_string()))
        } else {
            Ok(())
        }
    }
}

///
/// This trait allows a type to be added to the schema
///
/// Any type that implements `CliArg` automatically gets this trait implementation for free
///
/// This has to be a separate trait because it's also implemented by the tuple, allowing for
/// multiple arguments
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

/// Create the Schema from the CliArg type
impl<T> IntoSchema for T
where
    T: CliArg,
{
    fn add_schema(schema: &mut Schema) -> Result<()> {
        let short = T::short();
        let command = SchemaCommand {
            kind: T::Content::kind(),
            long: T::long(),
            short,
        };
        if let Some(short) = short {
            schema.add_short_command(short, command)?;
        }
        schema.add_command(T::long(), command)
    }
}

#[cfg(test)]
mod test {
    use crate::arg;
    use crate::schema::{Schema, SchemaCommand, SchemaKind};

    arg!(OutFile: "output", 'o' -> String);
    arg!(Force: "force", 'f' -> bool);
    arg!(SetUpstream: "set-upstream" -> String);
    arg!(OutFile2: "output", 'o' -> String);

    #[test]
    fn one_command_schema() {
        let schema = Schema::create::<OutFile>().unwrap();
        let out_file = SchemaCommand {
            kind: SchemaKind::String,
            long: "output",
            short: Some('o'),
        };
        assert_eq!(schema.longs.get("output"), Some(&out_file));
        assert_eq!(schema.shorts.get(&'o'), Some(&out_file));
        assert_eq!(schema.longs.get("o"), None);
    }

    #[test]
    fn two_command_schema() {
        let schema = Schema::create::<(OutFile, Force)>().unwrap();
        let out_file = SchemaCommand {
            kind: SchemaKind::String,
            long: "output",
            short: Some('o'),
        };
        let force = SchemaCommand {
            kind: SchemaKind::Bool,
            long: "force",
            short: Some('f'),
        };

        assert_eq!(schema.longs.get("output"), Some(&out_file));
        assert_eq!(schema.shorts.get(&'o'), Some(&out_file));
        assert_eq!(schema.longs.get("o"), None);

        assert_eq!(schema.longs.get("force"), Some(&force));
        assert_eq!(schema.shorts.get(&'f'), Some(&force));
        assert_eq!(schema.longs.get("f"), None);
    }

    #[test]
    fn three_command_schema() {
        let schema = Schema::create::<(OutFile, (Force, SetUpstream))>().unwrap();
        let out_file = SchemaCommand {
            kind: SchemaKind::String,
            long: "output",
            short: Some('o'),
        };
        let force = SchemaCommand {
            kind: SchemaKind::Bool,
            long: "force",
            short: Some('f'),
        };
        let set_upstream = SchemaCommand {
            kind: SchemaKind::String,
            long: "set-upstream",
            short: None,
        };

        assert_eq!(schema.longs.get("output"), Some(&out_file));
        assert_eq!(schema.shorts.get(&'o'), Some(&out_file));
        assert_eq!(schema.longs.get("o"), None);

        assert_eq!(schema.longs.get("force"), Some(&force));
        assert_eq!(schema.shorts.get(&'f'), Some(&force));
        assert_eq!(schema.longs.get("f"), None);

        assert_eq!(schema.longs.get("set-upstream"), Some(&set_upstream));
    }

    #[test]
    fn double_error() {
        let schema = Schema::create::<(OutFile, OutFile2)>();
        assert!(schema.is_err());
    }
}
