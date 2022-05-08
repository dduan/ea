use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let file = env::args().nth(1).unwrap();
    _ = io::stdout().write(&fs::read(file).unwrap());
}
