use std;
use std::io::{stderr,stdout};
use std::path::Path;

pub fn main(args: &[~str]) {
    match args {
        [_] => {
            stderr().write_line("usage: dirname path");
            std::os::set_exit_status(1);
            return;
        }
        [_, ref f, ..] => {
            let p = Path::new(f.as_slice());
            stdout().write(p.dirname());
            stdout().write_char('\n');
        }
        _ => unreachable!()
    }
}

