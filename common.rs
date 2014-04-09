use getopts::OptGroup;
use std::io::stdio;

pub fn print_usage(usage: &str, opts: &[OptGroup]) {
    err_write_line(format!("Usage: {}", usage));
    for o in opts.iter() {
        match *o {
            OptGroup{short_name: ref s, long_name: ref l,  desc: ref d, ..} => {
                if *l == ~"" {
                    err_write_line(format!("\t-{}: {}", *s, *d))
                } else if *s == ~"" {
                    err_write_line(format!("\t--{}: {}", *l, *d))
                } else {
                    err_write_line(format!("\t-{}, --{}: {}", *s, *l, *d))
                }
            }
        };
    }
}

pub fn err_write_line(s: &str) {
    let _ = stdio::stderr().write_line(s);
}
