extern mod extra;
use extra::getopts::groups::{getopts,optopt};
use std::io::fs::File;
use std::path::Path;
mod common;

fn head_file(filename: &str, linecount: int, bytecount: int) {
    std::io::io_error::cond.trap(|e| {
        if e.kind != std::io::EndOfFile {
            std::io::stderr().write_line(format!("head: {:s}: {:s}", filename, e.desc));
        }
    }).inside(|| {
        let mut f = File::open(&Path::new(filename));
        if f.is_none() {
            return;
        }
        head_reader(filename, f.get_mut_ref(), linecount, bytecount);
    });
}

fn head_reader(filename: &str, f: &mut std::io::Reader, linecount: int, bytecount: int) {
    std::io::io_error::cond.trap(|e| {
        if e.kind != std::io::EndOfFile {
            std::io::stderr().write_line(format!("head: {:s}: {:s}", filename, e.desc));
        }
    }).inside(|| {
        let mut stdout = std::io::stdout();
        if bytecount > 0 {
            if !f.eof() {
                let head = f.read_bytes(bytecount as uint);
                stdout.write(head);
            }
        } else {
            let mut buf = [0u8, ..1024];
            let mut count = linecount;
            while count > 0 {
                match f.read(buf) {
                    Some(len) => {
                        for i in range(0, len) {
                            if buf[i] == '\n' as u8 {
                                count -= 1;
                                if count == 0 {
                                    // print until current point
                                    stdout.write(buf.slice(0, i+1));
                                    break;
                                }
                            }
                        }
                        if count > 0 {
                            stdout.write(buf)
                        }
                    }
                    _ => {break;}
                }
            }
        }
    })
}

fn main() {
    let opts = ~[
        optopt("n", "", "number of lines", "lines"),
        optopt("c", "", "number of bytes", "bytes"),
    ];
    let args = std::os::args();
    let mut stderr = std::io::stderr();
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            stderr.write_line(f.to_err_msg());
            common::print_usage("head [-n lines | -c bytes] [file ...]", opts);
            fail!();
        }
        Ok(m) => { m }
    };

    let c = match matches.opt_str("c") {
        Some(s) => match from_str::<int>(s) {
            Some(n) if n > 0 => n,
            _ => {fail!(format!("illegal byte count -- {}", s));}
        },
        None => -1
    };
    let n = match matches.opt_str("n") {
        Some(s) => match from_str::<int>(s) {
            Some(n) if n > 0 => n,
            _ => {fail!(format!("illegal line count -- {}", s));}
        },
        None => if c > 0 { -1 } else { 10 }
    };
    if n * c >= 0 {
        fail!("can't combine line and byte counts");
    }
    match matches.free {
        [] => {
            head_reader("stdin", & mut std::io::stdin() as &mut Reader, n, c);
        }
        [filename] => {
            head_file(filename, n, c);
        }
        [..filenames] => {
            /* show file header */
            let mut first = 1;
            for filename in filenames.iter() {
                if first != 1 {
                    println("");
                }
                first = 0;
                println!("==> {} <==", *filename);
                head_file(*filename, n, c);
            }
        }
    }
}
