use extra::getopts::groups::{getopts,optflag};
use std;
use std::path::Path;
use common;

pub fn main(args: &[~str]) {
    let opts = ~[
        optflag("p", "", "Create intermediate directories as required."),
    ];
    let mut stderr = std::io::stderr();
    let usage = "mkdir [-p] directory ...";
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
        match std::io::result(|| {
            if (recursive) {
                std::io::fs::mkdir_recursive(&Path::new(d.as_slice()), std::io::UserDir);
            } else {
                std::io::fs::mkdir(&Path::new(d.as_slice()), std::io::UserDir);
            }
        }) {
            Err(e) => {
                std::io::stderr().write_line(format!("mkdir: {:s}: {:s}", *d, e.desc));
                std::os::set_exit_status(1);
            }
            _ => {}
        }
    }
}
