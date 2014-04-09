use std;
use std::io::fs::File;
use std::path::Path;
use common;

pub fn main(args: &[~str]) {
    match args {
        [_] => {
            match std::io::util::copy(&mut std::io::stdin(), &mut std::io::stdout()) {
                Err(e) => {
                    if e.kind != std::io::EndOfFile {
                        common::err_write_line(format!("cat: stdin: {:s}", e.desc));
                        std::os::set_exit_status(1);
                    }
                }
                _ => {}
            };
        }
        [_, ..filenames] => {
            for fname in filenames.iter() {
                let mut f = match File::open(&Path::new(fname.as_slice())) {
                    Ok(f) => f,
                    Err(e) => {
                        common::err_write_line(format!("cat: {:s}: {:s}", *fname, e.desc));
                        std::os::set_exit_status(1);
                        continue;
                    }
                };
                match std::io::util::copy(&mut f, &mut std::io::stdout()) {
                    Err(e) => {
                        if e.kind != std::io::EndOfFile {
                            common::err_write_line(format!("cat: {:s}: {:s}", *fname, e.desc));
                            std::os::set_exit_status(1);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => unreachable!()
    }
}
