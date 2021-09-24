#[macro_export]
macro_rules! arg {
    ($name:ident: $long:literal, $short:literal -> $result:ty) => {
        arg!(@$name: ($long, ::std::option::Option::Some($short)) -> $result);
    };
    ($name:ident: $long:literal -> $result:ty) => {
        arg!(@$name: ($long, ::std::option::Option::None) -> $result);
    };
    (@$name:ident: ($long:literal, $short:expr) -> $result:ty) => {
        #[derive(Default)]
        struct $name;

        impl $crate::CliArg for $name {
            type Content = $result;

            fn long() -> &'static str {
                $long
            }

            fn short() -> Option<&'static str> {
                $short
            }
        }
    };
}
