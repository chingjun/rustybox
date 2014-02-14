pub fn main(args: &[~str]) {
    match args {
        [_, ~"-n", ..args] => print(args.connect(" ")),
        [_, ..args] => println(args.connect(" ")),
        _ => println(""),
    };
}
