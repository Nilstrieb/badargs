# badargs

A zero-dependency full type-safe argument parser.

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

Boolean values can only be `None` or `Some`.  
The other values can be `None` or `Some(_)`