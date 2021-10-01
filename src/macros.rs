#[macro_export]
macro_rules! arg {
    // implicit optional
    ($name:ident: $long:literal, $short:literal -> $result:ty) => {
        arg!(@$name: ($long, ::std::option::Option::Some($short)) -> $result, false);
    };
    ($name:ident: $long:literal -> $result:ty) => {
        arg!(@$name: ($long, ::std::option::Option::None) -> $result, false);
    };
    // required
    ($name:ident: $long:literal, $short:literal -> $result:ty, required) => {
        arg!(@$name: ($long, ::std::option::Option::Some($short)) -> $result, true);
    };
    ($name:ident: $long:literal -> $result:ty, required) => {
        arg!(@$name: ($long, ::std::option::Option::None) -> $result, true);
    };
    (@$name:ident: ($long:literal, $short:expr) -> $result:ty, $required:literal) => {
        #[derive(Default)]
        struct $name;

        impl $crate::CliArg for $name {
            type Content = $result;

            fn long() -> &'static str {
                $long
            }

            fn short() -> Option<char> {
                $short
            }

            fn required() -> bool {
                $required
            }
        }
    };
}
