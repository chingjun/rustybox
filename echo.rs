fn main() {
    match std::os::args() {
        [_, ~"-n", ..args] => print(args.connect(" ")),
        [_, ..args] => println(args.connect(" ")),
        _ => println(""),
    };
}
