use crate::error::CallError;
use crate::schema::{Schema, SchemaKind};
use std::any::Any;
use std::collections::HashMap;
use std::ffi::OsString;

type Result<T> = std::result::Result<T, CallError>;

#[derive(Debug, Default)]
pub(crate) struct CliArgs {
    args: HashMap<&'static str, Box<dyn Any>>,
    unnamed: Vec<String>,
}

impl CliArgs {
    pub fn from_args(schema: &Schema, mut args: impl Iterator<Item = OsString>) -> Result<Self> {
        let mut result = Self::default();

        while let Some(arg) = args.next() {
            let arg = arg
                .into_string()
                .map_err(|os_str| CallError::InvalidUtf8(os_str))?;

            if let Some(long) = arg.strip_prefix("--") {
                parse_long(schema, &mut result, long, &mut args)?;
            } else if let Some(shorts) = arg.strip_prefix('-') {
                parse_shorts(schema, &mut result, shorts, &mut args)?;
            } else {
                result.unnamed.push(arg);
            }
        }

        Ok(result)
    }

    /// Get a value from the map, expecting it to have type T
    pub fn get<T: Any>(&self, long: &str) -> Option<&T> {
        let any = self.args.get(long)?;
        any.downcast_ref()
    }

    pub fn unnamed(&self) -> &[String] {
        &self.unnamed
    }

    fn insert(&mut self, long: &'static str, value: Box<dyn Any>) {
        self.args.insert(long, value);
    }
}

fn parse_shorts(
    schema: &Schema,
    results: &mut CliArgs,
    shorts: &str,
    args: &mut impl Iterator<Item = OsString>,
) -> Result<()> {
    // there are kinds of short arguments
    // single shorts that takes values: `-o main`
    // multiple flags combined: `-xzf`
    // combining these is invalid: `-xo main`

    let mut all_shorts = shorts.chars();

    let first_flag = all_shorts.next();

    if let Some(flag) = first_flag {
        let command = schema
            .short(flag)
            .ok_or_else(|| CallError::ShortFlagNotFound(flag))?;

        parse_value(command.kind, results, &command.long, args)?;
    } else {
        // '-' is a valid argument, like the `cat -`
        results.unnamed.push("-".to_string());
    }

    for flag in all_shorts {
        let command = schema
            .short(flag)
            .ok_or_else(|| CallError::ShortFlagNotFound(flag))?;

        if let SchemaKind::Bool = command.kind {
            results.insert(&command.long, Box::new(true));
        } else {
            return Err(CallError::CombinedShortWithValue(command.long.to_string()));
        }
    }

    Ok(())
}

fn parse_long(
    schema: &Schema,
    results: &mut CliArgs,
    long: &str,
    args: &mut impl Iterator<Item = OsString>,
) -> Result<()> {
    let command = schema
        .long(long)
        .ok_or_else(|| CallError::LongFlagNotFound(long.to_string()))?;

    parse_value(command.kind, results, &command.long, args)
}

fn parse_value(
    kind: SchemaKind,
    results: &mut CliArgs,
    long: &'static str,
    args: &mut impl Iterator<Item = OsString>,
) -> Result<()> {
    match kind {
        SchemaKind::String => {
            let string = args
                .next()
                .ok_or_else(|| CallError::ExpectedValue(long.to_string(), kind))?
                .into_string()
                .map_err(|os_str| CallError::InvalidUtf8(os_str))?;
            results.insert(long, Box::new(string));
        }
        SchemaKind::INum => {
            let integer = args
                .next()
                .ok_or_else(|| CallError::ExpectedValue(long.to_string(), kind))?
                .into_string()
                .map_err(|os_str| CallError::InvalidUtf8(os_str))?
                .parse::<isize>()
                .map_err(|_| CallError::INan(long.to_string()))?;
            results.insert(long, Box::new(integer))
        }
        SchemaKind::UNum => {
            let integer = args
                .next()
                .ok_or_else(|| CallError::ExpectedValue(long.to_string(), kind))?
                .into_string()
                .map_err(|os_str| CallError::InvalidUtf8(os_str))?
                .parse::<usize>()
                .map_err(|_| CallError::UNan(long.to_string()))?;
            results.insert(long, Box::new(integer))
        }
        SchemaKind::Bool => {
            results.insert(long, Box::new(true));
        }
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::arg;
    use crate::schema::Schema;

    arg!(OutFile: "output", 'o' -> String);
    arg!(Input: "input", 'i' -> String);
    arg!(SetUpstream: "set-upstream" -> String);

    arg!(Force: "force", 'f' -> bool);
    arg!(Gentle: "gentle", 'g' -> bool);

    arg!(OLevel: "olevel", 'l' -> usize);
    arg!(Iq: "iq", 'q' -> isize);

    fn schema() -> Schema {
        Schema::create::<(
            (OutFile, (Input, (OLevel, Iq))),
            ((Force, Gentle), SetUpstream),
        )>()
        .unwrap()
    }

    fn parse_args(args: &str) -> Result<CliArgs> {
        CliArgs::from_args(
            &schema(),
            args.split_whitespace()
                .map(|s| OsString::from(s.to_owned())),
        )
    }

    #[test]
    fn single_short_flag() {
        let args = parse_args("-f").unwrap();
        assert_eq!(args.get::<bool>("force"), Some(&true))
    }

    #[test]
    fn single_string_arg() {
        let args = parse_args("-i stdin").unwrap();
        assert_eq!(args.get::<String>("input"), Some(&"stdin".to_string()))
    }

    #[test]
    fn two_unnamed() {
        let args = parse_args("hallo welt").unwrap();
        assert_eq!(args.unnamed(), &["hallo", "welt"]);
    }

    #[test]
    fn short_arg_param_and_unnamed() {
        let args = parse_args("-i stdin uwu").unwrap();
        assert_eq!(args.unnamed(), &["uwu"]);
        assert_eq!(args.get::<String>("input"), Some(&"stdin".to_string()))
    }

    #[test]
    fn short_numbers() {
        let args = parse_args("-q -5423 -l 235235").unwrap();
        assert_eq!(args.get::<isize>("iq"), Some(&-5423));
        assert_eq!(args.get::<usize>("olevel"), Some(&235235));
    }

    #[test]
    fn combined_shorts() {
        let args = parse_args("-gf").unwrap();
        assert_eq!(args.get::<bool>("gentle"), Some(&true));
        assert_eq!(args.get::<bool>("force"), Some(&true));
    }

    #[test]
    fn long_flags() {
        let args = parse_args("--force --gentle").unwrap();
        assert_eq!(args.get::<bool>("gentle"), Some(&true));
        assert_eq!(args.get::<bool>("force"), Some(&true));
    }

    #[test]
    fn long_params() {
        let args = parse_args("--output main.c --iq 75").unwrap();
        assert_eq!(args.get::<isize>("iq"), Some(&75));
        assert_eq!(args.get::<String>("output"), Some(&"main.c".to_string()))
    }

    #[test]
    fn many() {
        let args = parse_args("--output main.c --iq 75 hallo -fg random").unwrap();
        assert_eq!(args.unnamed(), &["hallo", "random"]);
        assert_eq!(args.get::<bool>("gentle"), Some(&true));
        assert_eq!(args.get::<bool>("force"), Some(&true));
        assert_eq!(args.get::<String>("output"), Some(&"main.c".to_string()));
        assert_eq!(args.get::<isize>("iq"), Some(&75));
        assert_eq!(args.get::<usize>("olevel"), None);
        assert_eq!(args.get::<String>("input"), None)
    }
}
