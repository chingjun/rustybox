mod cat;
mod dirname;
mod echo;
mod head;
mod tee;
mod yes;

pub fn find_applet(name: &str) -> Option<fn(&[~str])> {
    match name {
        "cat" => Some(cat::main),
        "dirname" => Some(dirname::main),
        "echo" => Some(echo::main),
        "head" => Some(head::main),
        "tee" => Some(tee::main),
        "yes" => Some(yes::main),
        _ => None
    }
}
