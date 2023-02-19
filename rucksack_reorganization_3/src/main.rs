use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let priority = parse_lines(read_file(&args[1]));
    println!("Total priority: {}", priority);
    let badge_priority = get_group_badge(read_file(&args[1]), 3);
    println!("Badge priority: {}", badge_priority);
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

        total_priority += calculate_rucksack_priority(first_pouch, second_pouch);
    }

    total_priority
}

fn calculate_rucksack_priority(first_pouch: &str, second_pouch: &str) -> u32 {
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
            priority += calculate_item_priority(char);
        }
    }
    priority
}

fn calculate_item_priority(char: char) -> u32 {
    if char.is_uppercase() {
        // because ascii A is 65 and points is 27
        return char as u32 - (65 - 27);
    } else if char.is_lowercase() {
        // because ascii a is 97 and points is 1
        return char as u32 - (97 - 1);
    }
    0
}

fn get_group_badge(reader: BufReader<File>, group_size: usize) -> u32 {
    let mut total_priority = 0;

    let mut group_item_map: HashMap<char, bool> = HashMap::new();
    let mut count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if count == 0 {
            // reset Hashmap every first rucksack in a group
            group_item_map = HashMap::new();
            add_items_to_empty_map(line, &mut group_item_map);
            count += 1;
        } else {
            group_item_map = remove_items_missing_from_map(line, &mut group_item_map);
            count += 1;
            if count >= group_size {
                // reset count and calculate priority of badge
                count = 0;
                total_priority +=
                    calculate_item_priority(*group_item_map.keys().next().expect("no char found"))
            }
        }
    }
    total_priority
}

fn add_items_to_empty_map(line: &str, map: &mut HashMap<char, bool>) {
    for char in line.chars() {
        map.insert(char, true);
    }
}

fn remove_items_missing_from_map(line: &str, map: &mut HashMap<char, bool>) -> HashMap<char, bool> {
    let mut output_map = HashMap::new();

    for char in line.chars() {
        if map.contains_key(&char) {
            output_map.insert(char, true);
        }
    }

    output_map
}
