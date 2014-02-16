use extra::getopts::groups::{getopts,optflag};
use std;
use common;
use std::path::Path;

pub fn main(args: &[~str]) {
    let opts = ~[
        optflag("L", "", "Display the logical current working directory."),
        optflag("P", "", "Display the physical current working directory (all symbolic links resolved)."),
    ];
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            std::io::stderr().write_line(f.to_err_msg());
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
                match std::io::result(|| {
                    let cwd_stat = Path::new(cwd.as_slice()).stat();
                    let dot_stat = Path::new(".").stat();
                    return (cwd_stat, dot_stat);
                }) {
                    Ok((cwd_stat, dot_stat)) => {
                        if cwd_stat.unstable.device == dot_stat.unstable.device && cwd_stat.unstable.inode == dot_stat.unstable.inode {
                            println(*cwd);
                            return;
                        }
                    }
                    _ => {
                    }
                }
                println("err");
            }
            _ => {}
        }
    }
    
    // else print physical path
    let cwd = std::os::getcwd();
    stdout.write(cwd.as_vec());
    stdout.write_char('\n');
}
