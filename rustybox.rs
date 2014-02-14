extern mod extra;
mod applets;
mod common;

fn rustybox_main(args: &[~str]) {
    let file_path = std::path::Path::new(args[0].as_slice());
    let name = match file_path.filename_str() {
        Some(n) => n,
        None => { fail!("unknown error"); }
    };
    if name == "rustybox" {
        return rustybox_main(args.tail());
    }
    match applets::find_applet(name) {
        Some(f) => f(args),
        None => { fail!(format!("Applet {} not found!", name)); }
    }
}

fn main() {
    rustybox_main(std::os::args());
}
