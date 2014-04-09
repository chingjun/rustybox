use std::io::{print,println};

pub fn main(args: &[~str]) {
    match args {
        [_, ref firstarg, ..args] if *firstarg == ~"-n" => print(args.connect(" ")),
        [_, ..args] => println(args.connect(" ")),
        _ => println(""),
    };
}
