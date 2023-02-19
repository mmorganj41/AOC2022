use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_file("input");
    let max = parse_lines(reader);
    println!("Max Calories: {max}");
}

fn read_file(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("Could not read file");
    BufReader::new(file)
}

fn parse_lines(reader: BufReader<File>) -> i32 {
    let mut max = 0;
    let mut current_total = 0;

    for line in reader.lines() {
        let line = line.expect("Can't read line");
        let line = line.trim();
        match line.parse::<i32>() {
            Ok(n) => current_total += n,
            Err(_) => {
                if current_total > max {
                    max = current_total
                };
                current_total = 0;
            }
        }
    }

    max
}
