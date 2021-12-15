///
/// Declare your arguments using this macro.
///
/// Possible patterns:
/// ```
/// use badargs::arg;
///
/// arg!(LongOrShort: "long-or-short", 's' -> bool);
/// arg!(OnlyLong: "only-long" -> bool);
/// arg!(pub OtherModule: "other-module" -> bool);
/// ```
///
///
/// ```
/// use badargs::arg;
///
/// arg!(Force: "force", 'f' -> bool);
/// ```
/// is a shorthand for
/// ```
/// use badargs::{arg, CliArg};
///
/// struct Force;
///
/// impl CliArg for Force {
///     type Content = bool;
///
///     fn long() -> &'static str {
///         "force"
///     }
///
///     fn short() -> Option<char> {
///         Some('f')
///     }
/// }
/// ```
#[macro_export]
macro_rules! arg {
    ($vis:vis $name:ident: $long:literal, $short:literal -> $result:ty) => {
        arg!(@$vis $name: ($long, ::std::option::Option::Some($short)) -> $result);
    };
    ($vis:vis $name:ident: $long:literal -> $result:ty) => {
        arg!(@$vis $name: ($long, ::std::option::Option::None) -> $result);
    };
    (@$vis:vis $name:ident: ($long:literal, $short:expr) -> $result:ty) => {
        #[derive(Default)]
        $vis struct $name;

        impl $crate::CliArg for $name {
            type Content = $result;

            fn long() -> &'static str {
                $long
            }

            fn short() -> Option<char> {
                $short
            }
        }
    };
}

///
/// A shorthand for calling the [`badargs`](crate::badargs()) main function
///
/// This macro lets you specify your arguments in a flat list, and then converts them into
/// nested tuples for you, since that's what's internally used.
/// ```
/// use badargs::arg;
/// arg!(Force: "force", 'f' -> bool);
/// arg!(OutFile: "outfile", 't' -> bool);
/// arg!(SetUpstream: "set-upstream", 'x' -> bool);
///
/// fn main() {
///     let args = badargs::badargs!(Force, OutFile, SetUpstream);
/// }
/// ```
/// will be expanded into
/// ```
/// use badargs::arg;
/// arg!(Force: "force", 'f' -> bool);
/// arg!(OutFile: "outfile", 't' -> bool);
/// arg!(SetUpstream: "set-upstream", 'x' -> bool);
///
/// fn main() {
///     let args = badargs::badargs::<(Force, (OutFile, SetUpstream))>();
/// }
/// ```
/// This only provides a minor benefit for programs with a small amount of args, but is
/// very useful for larger arg amounts.
#[macro_export]
macro_rules! badargs {
    (@inner $head:ty) => {
        $head
    };
    (@inner $head:ty, $($tail:ty),+) => {
        ($head, $crate::badargs!(@inner $($tail),+))
    };
    ($($tail:ty),+) => {
        {
            #[allow(unused_parens)] // allow this because there might only be one arg
            {
                $crate::badargs::<($crate::badargs!(@inner $($tail),+))>()
            }
        }
    };
}
