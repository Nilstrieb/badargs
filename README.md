Node: badargs is not 1.0 yet, so it may change at any time. Use it with caution.

# badargs

A zero-dependency full type-safe argument parser.

It's correct enough for what it does.

`badargs` handles non Utf8 input by just printing an error and exiting the program gracefully.

# How to use

```rust
use badargs::arg;

arg!(OutFile: "output", 'o' -> String);
arg!(Force: "force", 'f' -> bool);
arg!(OLevel: "optimize" -> usize);

fn main() {
    let args = badargs::badargs!(OutFile, Force, OLevel);

    let outfile = args.get::<OutFile>();
    let force = args.get::<Force>();
    let o_level = args.get::<OLevel>();

    println!("output:     {:?}", outfile);
    println!("force:      {:?}", force);
    println!("o-level:    {:?}", o_level);
    println!("other args: {:?}", args.unnamed())
    
}
```

Use the `badargs::arg!` macro to declare arguments like this:  
`arg!(Binding, long_name, optional_short_name -> return_type)`

The following return types are currently available:
* String
* bool
* isize
* usize
* f64

Boolean values can only be `None` or `Some(true)`.  
The other values can be `None` or `Some(_)`

# Todo

Automatic `--help` handling

(Maybe) adding metadata, for example for `--version`

# Why doesn't badargs have x?

If you want a fully featured, even more type safe argument parser, use [Clap](https://github.com/clap-rs/clap), or [structopt](https://github.com/TeXitoi/structopt).

These do have a lot of dependencies and/or proc macros, so they are a lot more heavy compilation wise. Badargs is perfect for you if you don't want or need that.
