use phf::phf_map;
use std::fs::File;
use std::io::{prelude::*, BufReader};

static MOVE_DEFINITION: phf::Map<char, char> = phf_map! {
    'A' => 'X',
    'B' => 'Y',
    'C' => 'Z',
};

static WHO_WINS: phf::Map<char, char> = phf_map! {
    'X' => 'Z',
    'Y' => 'X',
    'Z' => 'Y',
}

fn main() {
    parse_lines(read_file("input"));
}

fn read_file(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("Could not read file");
    BufReader::new(file)
}

fn parse_lines(reader: BufReader<File>) {
    
}