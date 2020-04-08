use kono::tabstops::Lines;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    if let Ok(_) = handle.read_to_string(&mut buffer) {
        println!("{}", Lines::new(buffer).to_string());
    }
}
