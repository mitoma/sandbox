use std::io::{self, Read};
use kono::tabstops::TabstopsLines;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    println!("{}", TabstopsLines::new(buffer).to_string());
}
