use badargs::arg;

arg!(OutFile: "output", 'o' -> String);
arg!(Force: "force", 'f' -> bool);
arg!(OLevel: "optimize" -> usize);

fn main() {
    let args = badargs::badargs!(OutFile, Force, OLevel);

    let outfile = args.get::<OutFile>();
    let force = args.get::<Force>();
    let o_level = args.get::<OLevel>();

    println!("output:  {:?}", outfile);
    println!("force:   {:?}", force);
    println!("o-level: {:?}", o_level);
}
