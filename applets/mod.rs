mod cat;
mod clear;
mod dirname;
mod echo;
mod head;
mod mkdir;
mod pwd;
mod rmdir;
mod sleep;
mod tee;
mod wc;
mod which;
mod yes;

pub fn find_applet(name: &str) -> Option<fn(&[~str])> {
    match name {
        "cat" => Some(cat::main),
        "clear" => Some(clear::main),
        "dirname" => Some(dirname::main),
        "echo" => Some(echo::main),
        "head" => Some(head::main),
        "mkdir" => Some(mkdir::main),
        "pwd" => Some(pwd::main),
        "rmdir" => Some(rmdir::main),
        "sleep" => Some(sleep::main),
        "tee" => Some(tee::main),
        "wc" => Some(wc::main),
        "which" => Some(which::main),
        "yes" => Some(yes::main),
        _ => None
    }
}
