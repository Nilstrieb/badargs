use badargs::{CliArg, CliArgInfo, Template};
use std::collections::HashMap;

#[derive(Default)]
struct OutFile;

fn main() {
    let args = badargs::badargs(Template {
        options: {
            let mut map = HashMap::new();
            map.insert(
                Box::new(OutFile),
                CliArgInfo {
                    name: "output".to_string(),
                    allow_short: true,
                    takes_value: true,
                },
            );
            map
        },
    });

    let outfile = args.get::<OutFile>();
}
