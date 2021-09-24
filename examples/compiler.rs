use badargs::arg;

arg!(OutFile: "output", 'o' -> Option<String>);
arg!(Force: "force", 'f' -> bool);
arg!(OLevel: "optimize" -> usize);

fn main() {
    let args = badargs::badargs::<(OutFile, (Force, OLevel))>().unwrap();

    let outfile = args.get::<OutFile>();
    let force = args.get::<Force>();
    let o_level = args.get::<OLevel>();
}
