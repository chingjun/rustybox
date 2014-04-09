use getopts::{getopts,optflag};
use std;
use std::io::println;
use common;
use std::path::Path;

pub fn main(args: &[~str]) {
    let opts = ~[
        optflag("L", "", "Display the logical current working directory."),
        optflag("P", "", "Display the physical current working directory (all symbolic links resolved)."),
    ];
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            common::err_write_line(f.to_err_msg());
            common::print_usage("pwd: usage: pwd [-LP]", opts);
            std::os::set_exit_status(1);
            return;
        }
        Ok(m) => { m }
    };

    let mut stdout = std::io::stdout();

    if !matches.opt_present("P") {
        match std::os::getenv("PWD") {
            Some(ref cwd) if cwd[0] == '/' as u8 => {
                match (Path::new(cwd.as_slice()).stat(), Path::new(".").stat()) {
                    (Ok(cwd_stat), Ok(dot_stat)) => {
                        if cwd_stat.unstable.device == dot_stat.unstable.device && cwd_stat.unstable.inode == dot_stat.unstable.inode {
                            println(*cwd);
                            return;
                        }
                    }
                    _ => {
                    }
                }
                println("err"); //TODO
            }
            _ => {}
        }
    }
    
    // else print physical path
    let cwd = std::os::getcwd();
    let _ = stdout.write(cwd.as_vec()); //TODO should handle return value?
    let _ = stdout.write_char('\n'); //TODO should handle return value?
}
