//!
//! Generates CLI argument schemas based on generic types
//!
//! This makes the interface of this crate fully type-safe! (and kind of cursed)

use super::Result;
use crate::{ArgError, CliArg, CliReturnValue};
use std::collections::HashMap;

///
/// The type of value the argument returns
///
/// This could *maybe* also be solved with trait objects but lets keep this for now
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SchemaKind {
    String,
    OptionString,
    Bool,
    INum,
    UNum,
}

///
/// A single command in the schema
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct SchemaCommand {
    kind: SchemaKind,
}

///
/// A runtime representation of the schema type
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Schema {
    longs: HashMap<&'static str, SchemaCommand>,
    shorts: HashMap<&'static str, SchemaCommand>,
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
        if let Some(_) = self.longs.insert(long_name, command) {
            Err(ArgError::NameAlreadyExists(long_name))
        } else {
            Ok(())
        }
    }

    fn add_short_command(
        &mut self,
        short_name: &'static str,
        command: SchemaCommand,
    ) -> Result<()> {
        if let Some(_) = self.shorts.insert(short_name, command) {
            Err(ArgError::NameAlreadyExists(short_name))
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
        let kind = T::Content::kind();
        let name = T::long();
        let short = T::short();
        let command = SchemaCommand { kind };
        if let Some(short_name) = short {
            schema.add_short_command(short_name, command)?;
        }
        schema.add_command(name, command)
    }
}

#[cfg(test)]
mod test {
    use crate::schema::{Schema, SchemaCommand, SchemaKind};
    use crate::{arg, ArgError};

    arg!(OutFile: "output", "o" -> Option<String>);
    arg!(Force: "force", "f" -> bool);
    arg!(SetUpstream: "set-upstream" -> String);
    arg!(OutFile2: "output", "o" -> Option<String>);

    #[test]
    fn one_command_schema() {
        let schema = Schema::create::<OutFile>().unwrap();
        let out_file = SchemaCommand {
            kind: SchemaKind::OptionString,
        };
        assert_eq!(schema.longs.get("output"), Some(&out_file));
        assert_eq!(schema.shorts.get("o"), Some(&out_file));
        assert_eq!(schema.longs.get("o"), None);
        assert_eq!(schema.shorts.get("output"), None);
    }

    #[test]
    fn two_command_schema() {
        let schema = Schema::create::<(OutFile, Force)>().unwrap();
        let out_file = SchemaCommand {
            kind: SchemaKind::OptionString,
        };
        let force = SchemaCommand {
            kind: SchemaKind::Bool,
        };

        assert_eq!(schema.longs.get("output"), Some(&out_file));
        assert_eq!(schema.shorts.get("o"), Some(&out_file));
        assert_eq!(schema.longs.get("o"), None);
        assert_eq!(schema.shorts.get("output"), None);

        assert_eq!(schema.longs.get("force"), Some(&force));
        assert_eq!(schema.shorts.get("f"), Some(&force));
        assert_eq!(schema.longs.get("f"), None);
        assert_eq!(schema.shorts.get("force"), None);
    }

    #[test]
    fn three_command_schema() {
        let schema = Schema::create::<(OutFile, (Force, SetUpstream))>().unwrap();
        let out_file = SchemaCommand {
            kind: SchemaKind::OptionString,
        };
        let force = SchemaCommand {
            kind: SchemaKind::Bool,
        };
        let set_upstream = SchemaCommand {
            kind: SchemaKind::String,
        };

        assert_eq!(schema.longs.get("output"), Some(&out_file));
        assert_eq!(schema.shorts.get("o"), Some(&out_file));
        assert_eq!(schema.longs.get("o"), None);
        assert_eq!(schema.shorts.get("output"), None);

        assert_eq!(schema.longs.get("force"), Some(&force));
        assert_eq!(schema.shorts.get("f"), Some(&force));
        assert_eq!(schema.longs.get("f"), None);
        assert_eq!(schema.shorts.get("force"), None);

        assert_eq!(schema.longs.get("set-upstream"), Some(&set_upstream));
        assert_eq!(schema.shorts.get("set-upstream"), None);
    }

    #[test]
    fn double_error() {
        let schema = Schema::create::<(OutFile, OutFile2)>();
        assert!(matches!(
            schema,
            // it doesn't matter which one gets reported first
            Err(ArgError::NameAlreadyExists("output")) | Err(ArgError::NameAlreadyExists("o"))
        ));
    }
}
