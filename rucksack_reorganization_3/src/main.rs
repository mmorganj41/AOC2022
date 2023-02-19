use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let priority = parse_lines(read_file(&args[1]));
    println!("Total priority: {}", priority);
}

fn read_file(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("Could not read file");

    BufReader::new(file)
}

fn parse_lines(reader: BufReader<File>) -> u32 {
    let mut total_priority = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        let (first_pouch, second_pouch) = line.split_at(line.len() / 2);

        total_priority += calculate_priority(first_pouch, second_pouch);
    }

    total_priority
}

fn calculate_priority(first_pouch: &str, second_pouch: &str) -> u32 {
    let mut priority = 0;

    let mut first_pouch_items: HashMap<char, bool> = HashMap::new();
    for char in first_pouch.chars() {
        if !first_pouch_items.contains_key(&char) {
            first_pouch_items.insert(char, true);
        }
    }

    for char in second_pouch.chars() {
        if first_pouch_items.contains_key(&char) {
            //remove in case of duplication
            first_pouch_items.remove(&char);
            if char.is_uppercase() {
                // because ascii A is 65 and points is 27
                priority += char as u32 - (65 - 27);
            } else if char.is_lowercase() {
                // because ascii a is 97 and points is 1
                priority += char as u32 - (97 - 1);
            }
        }
    }
    priority
}
