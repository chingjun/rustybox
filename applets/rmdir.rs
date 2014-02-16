use extra::getopts::groups::{getopts,optflag};
use std;
use std::path::Path;
use common;

pub fn main(args: &[~str]) {
    let opts = ~[
        optflag("p", "", "Recursively remove all empty directories"),
    ];
    let mut stderr = std::io::stderr();
    let usage = "rmdir [-p] directory ...";
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            stderr.write_line(f.to_err_msg());
            common::print_usage(usage, opts);
            std::os::set_exit_status(1);
            return;
        }
        Ok(m) => { m }
    };
    if matches.free.len() == 0 {
        common::print_usage(usage, opts);
        std::os::set_exit_status(1);
        return;
    }

    let recursive = matches.opt_present("p");

    for d in matches.free.iter() {
        if recursive {
            let mut path = Path::new(d.as_slice());
            loop {
                match std::io::result(|| {
                    std::io::fs::rmdir(&path);
                }) {
                    Err(e) => {
                        std::io::stderr().write_line(format!("rmdir: {:s}: {:s}", path.as_str().unwrap(), e.desc));
                        std::os::set_exit_status(1);
                        break;
                    }
                    _ => {}
                }
                path.pop();
                if !path.clone().pop() {
                    break;
                }
            }
        } else {
            match std::io::result(|| {
                std::io::fs::rmdir(&Path::new(d.as_slice()));
            }) {
                Err(e) => {
                    std::io::stderr().write_line(format!("rmdir: {:s}: {:s}", *d, e.desc));
                    std::os::set_exit_status(1);
                }
                _ => {}
            }
        }
    }
}
