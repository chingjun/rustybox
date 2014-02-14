extern mod extra;
use extra::getopts::groups::{getopts,optflag};
use std::io::fs::File;
use std::path::Path;
mod common;
fn main() {
    let opts = ~[
        optflag("a", "", "Append the output to the files rather than overwriting them."),
    ];
    let args = std::os::args();
    let mut stderr = std::io::stderr();
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            stderr.write_line(f.to_err_msg());
            common::print_usage("usage: tee [-a] [file ...]", opts);
            fail!();
        }
        Ok(m) => { m }
    };
    let is_append = matches.opt_present("a");
    let mut files = ~[~std::io::stdout() as ~Writer];

    // open files
    for filename in matches.free.iter() {
        match std::io::result(|| {
            match File::open_mode(&Path::new(filename.as_slice()), if is_append { std::io::Append } else { std::io::Truncate }, std::io::Write) {
                Some(f) => {files.push(~f as ~Writer);}
                None => {}
            }
        }) {
            Err(e) => {
                std::io::stderr().write_line(format!("tee: {:s}: {:s}", *filename, e.desc));
            }
            _ => {}
        };
    }

    // wait from stdin and write to files
    let mut buf = [0u8, ..1024];
    let mut stdin = std::io::stdin();
    while !stdin.eof() {
        match std::io::result(|| {
            let len = stdin.read(buf);
            if len.is_some() {
                for f in files.mut_iter() {
                    f.write(buf.slice(0, len.unwrap()))
                }
            }
        }) {
            Err(e) => {
                if e.kind != std::io::EndOfFile {
                    std::io::stderr().write_line(format!("tee: {:s}", e.desc));
                } else {
                    break;
                }
            }
            _ => {}
        }
    }
}
