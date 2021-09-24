mod macros;
mod parse;
mod schema;

use crate::parse::CliArgs;
use crate::schema::{IntoSchema, Schema, SchemaKind};

pub use error::ArgError;
pub use macros::*;

pub type Result<T> = std::result::Result<T, ArgError>;

///
/// Parses the command line arguments based on the provided schema S
pub fn badargs<S>() -> Result<BadArgs>
where
    S: IntoSchema,
{
    let arg_schema = Schema::create::<S>()?;

    let args = CliArgs::from_args(&arg_schema, std::env::args())?;

    Ok(BadArgs { args })
}

///
/// Implemented by a user provided type that contains all info for a single command line argument
///
/// This is mostly done using unit structs and the `arg!` macro
///
/// ```
/// # use badargs::arg;
/// arg!(OutFile: "output", 'o' -> Option<String>);
/// // OutFile now implements CliArg
/// ```
pub trait CliArg {
    type Content: CliReturnValue;

    fn long() -> &'static str;
    fn short() -> Option<char>;
}

/// The struct containing parsed argument information
#[derive(Debug, Clone, Default)]
pub struct BadArgs {
    args: CliArgs,
}

impl BadArgs {
    /// Get the content of an argument by providing the type of the argument
    pub fn get<T>(&self) -> &T::Content
    where
        T: CliArg,
    {
        todo!()
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
    for Option<String> => OptionString;
    for bool => Bool;
    for isize => INum;
    for usize => UNum
);

mod sealed {
    pub trait SealedCliReturnValue {}
    macro_rules! impl_ {
        ($($name:ty),+) => {$(impl SealedCliReturnValue for $name{})+};
    }
    impl_!(String, Option<String>, bool, usize, isize);
}

mod error {
    /// The error type for `badargs`
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum ArgError {
        InvalidUtf8,
        NameAlreadyExists(String),
        InvalidSchema(String),
        IdkYet,
        UnnamedArgument,
        SingleMinus,
    }
}
