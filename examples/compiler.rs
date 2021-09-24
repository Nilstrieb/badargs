use badargs::arg;
use badargs::CliArg;

arg!(OutFile: "output", "o" -> Option<String>);
arg!(Force: "force", "f" -> bool);
arg!(OLevel: "optimize" -> usize);

fn main() {
    let args = badargs::badargs::<(OutFile, (Force, OLevel))>().unwrap();

    let _outfile = args.get::<OutFile>();
    let _force = args.get::<Force>();
    let _o_level = args.get::<OLevel>();
}
