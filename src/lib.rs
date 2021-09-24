mod macros;
mod schema;

use crate::parse::CliArgs;
use crate::schema::{IntoSchema, SchemaKind};

pub use error::ArgError;
pub use macros::*;

pub type Result<T> = std::result::Result<T, ArgError>;

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

pub trait CliArg {
    type Content: CliReturnValue;
    fn long() -> &'static str;
    fn short() -> Option<&'static str>;
}

#[derive(Debug, Clone, Default)]
pub struct BadArgs {
    args: CliArgs,
}

impl BadArgs {
    pub fn get<T>(&self) -> &T::Content
    where
        T: CliArg,
    {
        todo!()
    }
}

pub fn badargs<S>() -> Result<BadArgs>
where
    S: IntoSchema,
{
    let arg_schema = schema::parse_schema::<S>()?;

    let args = CliArgs::from_args(arg_schema, std::env::args_os())?;

    Ok(BadArgs { args })
}

mod error {
    #[derive(Debug, Clone)]
    pub enum ArgError {
        InvalidUtf8,
        NameAlreadyExists(&'static str),
        InvalidSchema(String),
        IdkYet,
    }
}

mod parse {
    use super::Result;
    use crate::schema::Schema;
    use std::collections::HashMap;
    use std::ffi::OsString;

    #[derive(Debug, Clone, Default)]
    pub struct CliArgs {
        pub isize: HashMap<&'static str, isize>,
        pub usize: HashMap<&'static str, isize>,
        pub string: HashMap<&'static str, String>,
        pub option_string: HashMap<&'static str, Option<String>>,
        pub bool: HashMap<&'static str, bool>,
    }

    impl CliArgs {
        pub fn from_args(_schema: Schema, args: impl Iterator<Item = OsString>) -> Result<Self> {
            let mut result = Self::default();
            let mut args = args;
            while let Some(_arg) = args.next() {}

            Ok(result)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::CliArg;

    struct OutFile;
    impl CliArg for OutFile {
        type Content = Option<String>;

        fn long() -> &'static str {
            "output"
        }

        fn short() -> Option<&'static str> {
            Some("o")
        }
    }

    #[test]
    fn get_single_schema() {}
}
