use std;
use common;

pub fn main(args: &[~str]) {
    match args {
        [_, ref s, ..] => {
            match from_str::<f32>(*s) {
                Some(secs) if secs > 0.0 => {
                    std::io::timer::sleep((secs * 1000.0) as u64);
                }
                _ => {}
            }
        }
        _ => {
            common::err_write_line("usage: sleep seconds");
            std::os::set_exit_status(1);
            return;
        }
    };
}
