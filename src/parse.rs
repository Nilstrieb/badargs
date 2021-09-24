use super::Result;
use crate::schema::Schema;
use crate::ArgError;
use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug, Clone, Default)]
pub struct CliArgs {
    pub isize: HashMap<&'static str, isize>,
    pub usize: HashMap<&'static str, isize>,
    pub string: HashMap<&'static str, String>,
    pub option_string: HashMap<&'static str, Option<String>>,
    pub bool: HashMap<&'static str, bool>,
}

impl CliArgs {
    pub fn from_args(schema: &Schema, args: impl Iterator<Item = String>) -> Result<Self> {
        let mut result = Self::default();

        let mut args = args.peekable();
        while let Some(arg) = args.next() {
            if let Some(shorts) = arg.strip_prefix('-') {
                parse_shorts(schema, &mut result, shorts, &mut args)?;
            } else if let Some(longs) = arg.strip_prefix("--") {
            } else {
                return Err(ArgError::UnnamedArgument);
            }
        }

        Ok(result)
    }
}

fn parse_shorts(
    schema: &Schema,
    results: &mut CliArgs,
    shorts: &str,
    args: &mut Peekable<impl Iterator<Item = String>>,
) -> Result<()> {
    if shorts.len() == 0 {
        return Err(ArgError::SingleMinus);
    }

    for flag_name in shorts.chars() {}

    Ok(())
}

fn expects_value_short(schema: &Schema, name: char) -> bool {
    schema.short('5');
    true
}
