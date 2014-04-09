use getopts::{getopts,optflag};
use std::io::fs::File;
use std::path::Path;
use std;
use common;

pub fn main(args: &[~str]) {
    let opts = ~[
        optflag("a", "", "Append the output to the files rather than overwriting them."),
    ];
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            common::err_write_line(f.to_err_msg());
            common::print_usage("usage: tee [-a] [file ...]", opts);
            std::os::set_exit_status(1);
            return;
        }
        Ok(m) => { m }
    };
    let is_append = matches.opt_present("a");
    let mut files = ~[~std::io::stdout() as ~Writer];

    // open files
    for filename in matches.free.iter() {
        let mode = if is_append { std::io::Append } else { std::io::Truncate };
        match File::open_mode(&Path::new(filename.as_slice()), mode, std::io::Write) {
            Ok(f) => {files.push(~f as ~Writer);}
            Err(e) => {
                common::err_write_line(format!("tee: {:s}: {:s}", *filename, e.desc));
                std::os::set_exit_status(1);
            }
        };
    }

    // wait from stdin and write to files
    let mut buf = [0u8, ..1024];
    let mut stdin = std::io::stdin();
    loop {
        let len = match stdin.read(buf) {
            Ok(len) => len,
            _ => { return; }
        };
        for f in files.mut_iter() {
            match f.write(buf.slice(0, len)) {
                Err(ref e) if e.kind == std::io::EndOfFile => { break; }
                Err(e) => {
                    common::err_write_line(format!("tee: {:s}", e.desc));
                    std::os::set_exit_status(1);
                }
                _ => {}
            }
        }
    }
}
