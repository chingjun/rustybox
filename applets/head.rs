use getopts::{getopts,optopt};
use std::io::fs::File;
use std::path::Path;
use std;
use std::io;
use std::io::println;
use common;

fn head_file(filename: &str, linecount: int, bytecount: int) -> io::IoResult<()> {
    let mut f = try!(File::open(&Path::new(filename)));
    head_reader(&mut f, linecount, bytecount)
}

fn head_reader<R: std::io::Reader> (f: &mut R, linecount: int, bytecount: int) -> io::IoResult<()> {
    let mut stdout = std::io::stdout();
    let mut buf = [0u8, ..1024];
    if bytecount > 0 {
        let len = match f.read(buf) {
            Ok(len) => len,
            Err(ref e) if e.kind == std::io::EndOfFile => 0,
            Err(e) => { return Err(e); }
        };
        try!(stdout.write(buf.slice(0, len)));
    } else {
        let mut buf = [0u8, ..1024];
        let mut count = linecount;
        while count > 0 {
            let len = try!(f.read(buf));
            for i in range(0, len) {
                if buf[i] == '\n' as u8 {
                    count -= 1;
                    if count == 0 {
                        // print until current point
                        try!(stdout.write(buf.slice(0, i+1)));
                        break;
                    }
                }
            }
            if count > 0 {
                try!(stdout.write(buf));
            }
        }
    }
    return Ok(());
}

pub fn main(args: &[~str]) {
    let opts = ~[
        optopt("n", "", "number of lines", "lines"),
        optopt("c", "", "number of bytes", "bytes"),
    ];
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            common::err_write_line(f.to_err_msg());
            common::print_usage("head [-n lines | -c bytes] [file ...]", opts);
            std::os::set_exit_status(1);
            return;
        }
        Ok(m) => { m }
    };

    let c = match matches.opt_str("c") {
        Some(s) => match from_str::<int>(s) {
            Some(n) if n > 0 => n,
            _ => {
                common::err_write_line(format!("illegal byte count -- {}", s));
                std::os::set_exit_status(1);
                return;
            }
        },
        None => -1
    };
    let n = match matches.opt_str("n") {
        Some(s) => match from_str::<int>(s) {
            Some(n) if n > 0 => n,
            _ => {
                common::err_write_line(format!("illegal line count -- {}", s));
                std::os::set_exit_status(1);
                return;
            }
        },
        None => if c > 0 { -1 } else { 10 }
    };
    if n * c >= 0 {
        common::err_write_line("can't combine line and byte counts");
        std::os::set_exit_status(1);
        return;
    }
    match matches.free.as_slice() {
        [] => {
            let filename = "stdin";
            match head_reader(& mut std::io::stdin(), n, c) {
                Err(e) => {
                    if e.kind != std::io::EndOfFile {
                        common::err_write_line(format!("head: {:s}: {:s}", filename, e.desc));
                        std::os::set_exit_status(1);
                    }
                }
                _ => {}
            }
        }
        [..filenames] => {
            /* show file header */
            let mut first = true;
            let multiple = filenames.len() > 1;
            for filename in filenames.iter() {
                if !first  {
                    println("");
                }
                first = false;
                if multiple {
                    println!("==> {} <==", *filename);
                }
                match head_file(*filename, n, c) {
                    Err(e) => {
                        if e.kind != std::io::EndOfFile {
                            common::err_write_line(format!("head: {:s}: {:s}", *filename, e.desc));
                            std::os::set_exit_status(1);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
