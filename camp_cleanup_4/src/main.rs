use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    read_file(&args[1]);
}

fn read_file(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("Could not read file");
    BufReader::new(file)
}
