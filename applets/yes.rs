use std::io::println;

pub fn main(args: &[~str]) {
    let default = ~"y";
    let s = match args {
        [_, ref s, ..] => s,
        [_] => &default,
        _ => unreachable!()
    };
    loop {
        println(*s);
    }
}
