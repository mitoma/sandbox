mod tabstops;

use std::io::{self, Read};
use tabstops::parse_tabstops_lines;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    println!("{}", parse_tabstops_lines(buffer).to_string());
}
