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
        let cwd = std::os::getenv("PWD");
        // If we're trying to find the logical current directory and that fails, behave as if -P was specified.
        if cwd.is_some() && cwd.as_ref().unwrap()[0] == '/' as u8 {
            let cwd = cwd.unwrap();
            match (Path::new(cwd.as_slice()).stat(), Path::new(".").stat()) {
                (Ok(cwd_stat), Ok(dot_stat)) => {
                    if cwd_stat.unstable.device == dot_stat.unstable.device && cwd_stat.unstable.inode == dot_stat.unstable.inode {
                        println(cwd);
                        return;
                    }
                }
                _ => {}
            }
        }
    }
    
    // else print physical path
    let cwd = std::os::getcwd();
    let _ = stdout.write(cwd.as_vec()); //TODO should handle return value?
    let _ = stdout.write_char('\n'); //TODO should handle return value?
}
