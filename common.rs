use extra::getopts::groups::OptGroup;
use std::io::stdio;

pub fn print_usage(usage: &str, opts: &[OptGroup]) {
    let mut stderr = stdio::stderr();
    stderr.write_line(format!("Usage: {}", usage));
    for o in opts.iter() {
        match *o {
            OptGroup{short_name: ref s, long_name: ref l,  desc: ref d, ..} => {
                if *l == ~"" {
                    stderr.write_line(format!("\t-{}: {}", *s, *d))
                } else if *s == ~"" {
                    stderr.write_line(format!("\t--{}: {}", *l, *d))
                } else {
                    stderr.write_line(format!("\t-{}, --{}: {}", *s, *l, *d))
                }
            }
        }
    }
}

