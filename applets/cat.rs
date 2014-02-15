use std;
use std::io;
use std::io::fs::File;
use std::path::Path;

fn copy_to_stdout(f: &mut std::io::Reader) {
    let mut buf = [0u8, ..1024];
    let mut out = io::stdout();
    while !f.eof() {
        let len = f.read(buf);
        if len.is_some() {
            out.write(buf.slice(0, len.unwrap()))
        } else {
            break;
        }
    }
}

pub fn main(args: &[~str]) {
    match args {
        [_] => {
            match io::result(|| {
                copy_to_stdout(&mut io::stdin() as &mut Reader);
            }) {
                Err(e) => {
                    if e.kind != io::EndOfFile {
                        io::stderr().write_line(format!("cat: stdin: {:s}", e.desc));
                        std::os::set_exit_status(1);
                    }
                }
                _ => {}
            }
        }
        [_, ..filenames] => {
            for fname in filenames.iter() {
                match io::result(|| {
                    let mut f = File::open(&Path::new(fname.as_slice()));
                    if f.is_none() {
                        return;
                    }
                    copy_to_stdout(f.get_mut_ref());
                }) {
                    Err(e) => {
                        io::stderr().write_line(format!("cat: {:s}: {:s}", *fname, e.desc));
                        std::os::set_exit_status(1);
                    }
                    _ => {}
                }
            }
        }
        _ => unreachable!()
    }
}
