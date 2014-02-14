use std::os;
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

fn main() {
    match os::args() {
        [_] => {
            io::io_error::cond.trap(|e| {
                if e.kind != io::EndOfFile {
                    io::stderr().write_line(format!("cat: stdin: {:s}", e.desc));
                }
            }).inside(|| {
                copy_to_stdout(& mut io::stdin() as &mut Reader)
            });
        }
        [_, ..filenames] => {
            for fname in filenames.iter() {
                io::io_error::cond.trap(|e| {
                    io::stderr().write_line(format!("cat: {:s}: {:s}", *fname, e.desc));
                }).inside(|| {
                    let mut f = File::open(&Path::new(fname.as_slice()));
                    if f.is_none() {
                        return;
                    }
                    copy_to_stdout(f.get_mut_ref());
                });
            }
        }
        _ => unreachable!()
    }
}
