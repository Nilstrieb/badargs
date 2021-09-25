use crate::error::CallError;
use crate::schema::{Schema, SchemaKind, SchemaKindType};
use std::any::Any;
use std::collections::HashMap;
use std::iter::Peekable;

type Result<T> = std::result::Result<T, CallError>;

#[derive(Debug, Default)]
pub struct CliArgs {
    args: HashMap<&'static str, Box<dyn Any>>,
}

impl CliArgs {
    pub fn from_args(schema: &Schema, args: impl Iterator<Item = String>) -> Result<Self> {
        let mut result = Self::default();

        let mut args = args.peekable();
        while let Some(arg) = args.next() {
            if let Some(shorts) = arg.strip_prefix('-') {
                parse_shorts(schema, &mut result, shorts, &mut args)?;
            } else if let Some(_longs) = arg.strip_prefix("--") {
            } else {
                return Err(CallError::UnnamedArgument);
            }
        }

        Ok(result)
    }

    /// Get a value from the map, expecting it to have type T
    /// Important: T should never be Option, making thisfh sjfhsekld fjkdsaljföoilkaesdf jikasoeldöojfliköesdafjisdolkyafj idrs
    pub fn get<T: Any>(&self, long: &str) -> Option<&T> {
        let any = self.args.get(long)?;
        any.downcast_ref()
    }

    fn insert(&mut self, long: &'static str, value: Box<dyn Any>) {
        self.args.insert(long, value);
    }
}

fn parse_shorts(
    schema: &Schema,
    results: &mut CliArgs,
    shorts: &str,
    args: &mut Peekable<impl Iterator<Item = String>>,
) -> Result<()> {
    // there are kinds of short arguments
    // single shorts that takes values: `-o main`
    // multiple flags combined: `-xzf`
    // combining these is invalid: `-xo main`

    let mut chars = shorts.chars();

    let first_flag = chars.next();

    if let Some(flag) = first_flag {
        let command = schema
            .short(flag)
            .ok_or_else(|| CallError::ShortFlagNotFound(flag))?;

        let inner_kind = match command.kind {
            SchemaKind::Required(inner) => inner,
            SchemaKind::Optional(inner) => inner,
        };

        match inner_kind {
            SchemaKindType::String => {
                let next = args
                    .next()
                    .ok_or_else(|| CallError::ExpectedValue(command.long.to_string()))?;
                results.insert(command.long, Box::new(next));
            }
            _ => todo!(),
        }
    } else {
        return Err(CallError::SingleMinus);
    }

    for _flag_name in chars {}

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::arg;
    use crate::schema::Schema;

    arg!(OutFile: "output", 'o' -> Option<String>);
    arg!(Input: "input", 'i' -> String);
    arg!(Force: "force", 'f' -> bool);
    arg!(SetUpstream: "set-upstream" -> String);

    fn schema() -> Schema {
        Schema::create::<((OutFile, Input), (Force, SetUpstream))>().unwrap()
    }

    fn parse_args(args: &str) -> Result<CliArgs> {
        CliArgs::from_args(&schema(), args.split_whitespace().map(|s| s.to_owned()))
    }

    #[test]
    #[ignore]
    fn single_short_flag() {
        let args = parse_args("-f").unwrap();
        assert_eq!(args.get::<bool>("force"), Some(&true))
    }

    #[test]
    fn single_string_arg() {
        let args = parse_args("-i stdin").unwrap();
        assert_eq!(args.get::<String>("input"), Some(&"stdin".to_string()))
    }
}
