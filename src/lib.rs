mod macros;
mod parse;
mod schema;

use crate::parse::CliArgs;
use crate::schema::{IntoSchema, Schema, SchemaKind};

pub use error::SchemaError;
pub use macros::*;
use std::any::Any;

pub type Result<T> = std::result::Result<T, SchemaError>;

///
/// Parses the command line arguments based on the provided schema S
pub fn badargs<S>() -> Result<BadArgs>
where
    S: IntoSchema,
{
    let arg_schema = Schema::create::<S>()?;

    let args = CliArgs::from_args(&arg_schema, std::env::args()).expect("todo");

    Ok(BadArgs { args })
}

///
/// Implemented by a user provided type that contains all info for a single command line argument
///
/// This is mostly done using unit structs and the `arg!` macro
///
/// ```
/// # use badargs::arg;
/// arg!(Force: "force", 'f' -> bool);
/// arg!(OutFile: "output", 'o' -> String);
/// // OutFile now implements CliArg
/// ```
// This trait requires any because some dynamic typing is done in the background
pub trait CliArg: Any {
    type Content: CliReturnValue;

    fn long() -> &'static str;
    fn short() -> Option<char>;
}

/// The struct containing parsed argument information
#[derive(Debug, Default)]
pub struct BadArgs {
    args: CliArgs,
}

impl BadArgs {
    /// Get the content of an argument by providing the type of the argument
    pub fn get<T>(&self) -> &T::Content
    where
        T: CliArg,
    {
        let long_name = T::long();
        self.args
            .get::<T::Content>(long_name)
            .expect("it has been validated")
    }
}

///
/// A type that could be parsed from command line arguments
pub trait CliReturnValue: sealed::SealedCliReturnValue {
    fn kind() -> schema::SchemaKind;
}

macro_rules! impl_cli_return {
    ($(for $ty:ty => $type:ident);+) => {$(
        impl CliReturnValue for $ty {
            fn kind() -> SchemaKind {
                SchemaKind::$type
            }
        }
    )+};
}

impl_cli_return!(
    for String => String;
    for bool => Bool;
    for isize => INum;
    for usize => UNum
);

mod sealed {
    pub trait SealedCliReturnValue {}
    macro_rules! impl_ {
        ($($name:ty),+) => {$(impl SealedCliReturnValue for $name{})+};
    }
    impl_!(String, bool, usize, isize);
}

mod error {
    /// Invalid schema
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum SchemaError {
        NameAlreadyExists(String),
        InvalidSchema(String),
    }

    /// Invalid arguments provided
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum CallError {
        SingleMinus,
        ShortFlagNotFound(char),
        ExpectedValue(String),
        INan(String),
        UNan(String),
    }
}
