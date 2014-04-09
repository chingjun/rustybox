use getopts::{getopts,optflag};
use std;
use std::path::Path;
use common;

pub fn main(args: &[~str]) {
    let opts = ~[
        optflag("a", "", "List all instances of executables found (instead of just the first one of each)."),
        optflag("s", "", "No output."),
    ];
    let usage = "which [-as] program ...";
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            common::err_write_line(f.to_err_msg());
            common::print_usage(usage, opts);
            std::os::set_exit_status(1);
            return;
        }
        Ok(m) => { m }
    };

    let silent = matches.opt_present("s");
    let showall = matches.opt_present("a");

    let pathenv = match std::os::getenv("PATH") {
        Some(s) => s,
        None => {
            std::os::set_exit_status(1);
            return;
        }
    };
    let paths: ~[Path] = pathenv.split(':').map(|x| { Path::new(x) }).collect();

    let mut stdout = std::io::stdout();

    match matches.free.as_slice() {
        [] => {
            common::print_usage(usage, opts);
            std::os::set_exit_status(1);
            return;
        }
        [..cmds] => {
            for cmd in cmds.iter() {
                let mut found = false;
                for path in paths.iter() {
                    let mut p = path.clone();
                    p.push(cmd.as_slice());
                    if check_path(&p) {
                        found = true;
                        if !silent {
                            let _ = stdout.write(p.as_vec()); //TODO should handle return value?
                            let _ = stdout.write_char('\n'); //TODO should handle return value?
                        }
                        if !showall {
                            break;
                        }
                    }
                }
                if !found {
                    std::os::set_exit_status(1);
                }
            }
        }
    }
}

fn check_path(p: &Path) -> bool {
    if ! p.exists() { return false; }
    match p.stat() {
        Ok(filestat) => {
            if filestat.kind == std::io::TypeFile && filestat.perm & 0111 != 0 {
                return true;
            }
        }
        _ => {}
    }
    return false;
}
