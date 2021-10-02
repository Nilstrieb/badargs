//!
//! # badargs
//!
//! A fully type-safe argument parser without any proc macros!
//!
//! Declare your arguments with structs. You probably want to use the macro for that
//! ```
//! # use badargs::arg;
//! arg!(Force: "force", 'f' -> bool);
//! arg!(OutFile: "output", 'o' -> String);
//! ```
//! Then you call the [`badargs`] function with all of your declared arguments. You probably
//! want to use a macro for that too.
//!
//! You can also use the [`badargs!`] macro if you have many arguments and don't want to nest
//! the tuples manually
//! ```
//! # use badargs::arg;
//! # arg!(Force: "force", 'f' -> bool);
//! # arg!(OutFile: "output", 'o' -> String);
//! let args = badargs::badargs!(Force, OutFile);
//! ```
//! You can then get values using your declared arguments
//! ```
//! # use badargs::arg;
//! # arg!(Force: "force", 'f' -> bool);
//! # arg!(OutFile: "output", 'o' -> String);
//! # let args = badargs::badargs!(Force, OutFile);
//! let force: Option<&bool> = args.get::<Force>();
//! let out_file: Option<&String> = args.get::<OutFile>();
//! ```

mod macros;
mod parse;
mod reporting;
mod schema;

use crate::parse::CliArgs;
use crate::schema::{IntoSchema, Schema, SchemaKind};
use std::any::Any;

pub use error::SchemaError;
pub use macros::*;

pub type Result<T> = std::result::Result<T, SchemaError>;

///
/// Parses the command line arguments based on the provided schema S
///
/// # Panics
///
/// This function panics if an invalid schema is entered
///
pub fn badargs<S>() -> BadArgs
where
    S: IntoSchema,
{
    let arg_schema = Schema::create::<S>().expect("Invalid schema");

    let args = CliArgs::from_args(&arg_schema, std::env::args_os());
    match args {
        Ok(args) => BadArgs { args },
        Err(err) => reporting::report(err, &arg_schema),
    }
}

///
/// Implemented by a user provided type that contains all info for a single command line argument
///
/// This is mostly done using unit structs and the `arg!` macro
///
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
    pub fn get<T>(&self) -> Option<&T::Content>
    where
        T: CliArg,
    {
        let long_name = T::long();
        self.args.get::<T::Content>(long_name)
    }

    /// Get all unnamed additional arguments
    pub fn unnamed(&self) -> &[String] {
        self.args.unnamed()
    }
}

///
/// A type that could be parsed from command line arguments
pub trait CliReturnValue: sealed::SealedCliReturnValue {
    fn kind() -> schema::SchemaKind;
}

macro_rules! impl_cli_return {
    ($(for $ty:ty => $type:ident);+;) => {$(
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
    for isize => IInt;
    for usize => UInt;
    for f64 => Num;
);

mod sealed {
    pub trait SealedCliReturnValue {}
    macro_rules! impl_ {
        ($($name:ty),+) => {$(impl SealedCliReturnValue for $name{})+};
    }
    impl_!(String, bool, usize, isize, f64);
}

mod error {
    use crate::schema::SchemaKind;
    use std::ffi::OsString;

    /// Invalid schema
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum SchemaError {
        NameAlreadyExists(String),
        InvalidSchema(String),
    }

    /// Invalid arguments provided
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum CallError {
        ShortFlagNotFound(char),
        LongFlagNotFound(String),
        ExpectedValue(String, SchemaKind),
        INan(String),
        UNan(String),
        NNan(String),
        CombinedShortWithValue(String),
        InvalidUtf8(OsString),
        HelpPage,
    }
}
