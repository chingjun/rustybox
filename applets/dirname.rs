use std;
use std::path::Path;
use common;

pub fn main(args: &[~str]) {
    match args {
        [_] => {
            common::err_write_line("usage: dirname path");
            std::os::set_exit_status(1);
            return;
        }
        [_, ref f, ..] => {
            let p = Path::new(f.as_slice());
            let _ = std::io::stdout().write(p.dirname()); //TODO should handle return value?
            let _ = std::io::stdout().write_char('\n'); //TODO should handle return value?
        }
        _ => unreachable!()
    }
}

