use crate::error::CallError;
use crate::schema::{Schema, SchemaKind};

pub fn report(err: CallError, schema: &Schema) -> ! {
    match err {
        CallError::ShortFlagNotFound(arg) => println!("error: argument '{}' does not exist.", arg),
        CallError::LongFlagNotFound(arg) => println!("error: argument '{}' does not exist.", arg), 
        CallError::ExpectedValue(arg, kind) => {
            println!(
                "error: argument '{}' expected {} value, but got nothing.",
                arg,
                match kind {
                    SchemaKind::String => "string",
                    SchemaKind::Bool => unreachable!(),
                    SchemaKind::IInt => "integer",
                    SchemaKind::UInt => "positive integer",
                    SchemaKind::Num => "number"
                }
            )
        }
        CallError::INan(arg) => println!("error: argument '{}' expected a positive integer value, but got an invalid positive integer.", arg),
        CallError::UNan(arg) => println!("error: argument '{}' expected an integer value, but got an invalid integer.", arg),
        CallError::NNan(arg) => println!("error: argument '{}' expected a number value, but got an invalid number.", arg),
        CallError::CombinedShortWithValue(arg) => println!("error: using argument expecting value '{}' in position where only flags are allowed", arg),
        CallError::InvalidUtf8(os_str) => println!("error: invalid utf8: '{}'", os_str.to_string_lossy()),
        CallError::HelpPage => {
            println!("Options:");
            for option in schema.arguments() {
                print!("--{} ", option.long);
                if let Some(short) = option.short {
                    print!("(-{}) ", short);
                }
                match option.kind {
                    SchemaKind::String => print!("[Takes a value]"),
                    SchemaKind::Bool => {}
                    SchemaKind::IInt => print!("[Takes an integer]"),
                    SchemaKind::UInt => print!("[Takes a positive integer]"),
                    SchemaKind::Num => print!("[Takes a number]"),
                }
                println!();
            }
            std::process::exit(0);
        }
    }

    std::process::exit(1)
}
