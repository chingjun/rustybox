use extra::getopts::groups::{getopts,optflag};
use std;
use std::io::fs::File;
use std::path::Path;
use common;

pub fn main(args: &[~str]) {
    let opts = ~[
        optflag("c", "", "The number of bytes in each input file is written to the standard output."),
        optflag("w", "", "The number of words in each input file is written to the standard output."),
        optflag("l", "", "The number of lines in each input file is written to the standard output."),
    ];
    let mut stderr = std::io::stderr();
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            stderr.write_line(f.to_err_msg());
            common::print_usage("wc [-clw] [file ...]", opts);
            std::os::set_exit_status(1);
            return;
        }
        Ok(m) => { m }
    };

    let (print_char, print_word, print_line) = match (matches.opt_present("c"), matches.opt_present("w"), matches.opt_present("l")) {
        (false, false, false) => (true, true, true),
        (c,w,l) => (c,w,l)
    };

    match matches.free {
        [] => {
            let result = wc_reader("", & mut std::io::stdin() as &mut Reader);
            print_result("", result, print_char, print_word, print_line);
        }
        [..filenames] => {
            let mut total: (u64, u64, u64) = (0,0,0);

            for filename in filenames.iter() {
                let mut f = match std::io::result(|| { File::open(&Path::new(filename.as_slice())) }) {
                    Err(e) => {
                        std::io::stderr().write_line(format!("wc: {:s}: {:s}", *filename, e.desc));
                        return;
                    }
                    Ok(f) => f
                };
                let result = wc_reader(*filename, f.get_mut_ref());
                print_result(*filename, result, print_char, print_word, print_line);
                total = match (total, result) {
                    ((a1,b1,c1), (a2,b2,c2)) => (a1+a2, b1+b2, c1+c2)
                }
            }

            if filenames.len() > 1 {
                print_result("total", total, print_char, print_word, print_line);
            }
        }
    }
}

fn wc_reader(filename: &str, reader: &mut std::io::Reader) -> (u64, u64, u64) {
    let mut buf = [0u8, ..4096];

    let mut linecount = 0;
    let mut charcount = 0;
    let mut wordcount = 0;
    let mut inword = false;

    while !reader.eof() {
        let len = match std::io::result(|| { reader.read(buf) }) {
            Err(e) => {
                if e.kind != std::io::EndOfFile {
                    std::io::stderr().write_line(format!("wc: {:s}: {:s}", filename, e.desc));
                }
                break;

            }
            Ok(Some(l)) => l,
            _ => break
        };

        for i in range(0, len) {
            let c = buf[i];
            charcount += 1;

            if c == '\n' as u8 {
                linecount += 1;
            }

            if !inword {
                if c >= 33 /* printable non-space character in ascii */ {
                    inword = true;
                    wordcount += 1;
                }
            } else {
                if c <= 32 {
                    inword = false;
                }
            }
        }
    }

    return (charcount, wordcount, linecount);
}

fn print_result(filename: &str, result: (u64, u64, u64), printc: bool, printw: bool, printl: bool) {
    match result {
        (c, w, l) => {
            if printl {
                print!(" {:7u}", l);
            }
            if printw {
                print!(" {:7u}", w);
            }
            if printc {
                print!(" {:7u}", c);
            }
            println!(" {:s}", filename);
        }
    }
}
