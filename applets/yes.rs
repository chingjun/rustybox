use std;

pub fn main(args: &[~str]) {
    let default = ~"y";
    let s = match args {
        [_, ref s, ..] => s,
        [_] => &default,
        _ => unreachable!()
    };
    let mut cont = true;
    std::io::io_error::cond.trap(|_| {
        cont = false;
    }).inside(|| {
        while cont {
            println(*s);
        }
    });
}
