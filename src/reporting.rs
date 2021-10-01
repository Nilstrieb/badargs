use crate::error::CallError;
use crate::schema::SchemaKind;

pub fn report(err: CallError) -> ! {
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
                    SchemaKind::INum => "integer",
                    SchemaKind::UNum => "positive integer",
                }
            )
        }
        CallError::INan(arg) => println!("error: argument '{}' expected a positive integer value, but got an invalid positive integer.", arg),
        CallError::UNan(arg) => println!("error: argument '{}' expected an integer value, but got an invalid integer.", arg),
        CallError::CombinedShortWithValue(arg) => println!("error: using argument expecting value '{}' in position where only flags are allowed", arg),
        CallError::InvalidUtf8(os_str) => println!("error: invalid utf8: '{}'", os_str.to_string_lossy()),
    }

    std::process::exit(1)
}
