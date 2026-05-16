use std::process::Command;

pub fn run() {
    Command::new("cmd")
        .args(["/C", "cls"])
        .status()
        .unwrap();
}
