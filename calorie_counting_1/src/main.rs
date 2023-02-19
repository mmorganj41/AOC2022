use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let reader = read_file("input");
    let max = parse_lines(reader);
    println!("Max Calories: {max:?}");
}

fn read_file(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("Could not read file");
    BufReader::new(file)
}

fn parse_lines(reader: BufReader<File>) -> i32 {
    let mut max = [0; 3];
    let mut current_total = 0;

    for line in reader.lines() {
        let line = line.expect("Can't read line");
        let line = line.trim();
        match line.parse::<i32>() {
            Ok(n) => current_total += n,
            Err(_) => {
                replace_with_higher(current_total, &mut max);
                current_total = 0;
            }
        }
    }

    max.iter().sum()
}

fn replace_with_higher(current: i32, array: &mut [i32]) {
    for i in 0..array.len() {
        if current > array[i] {
            if i >= array.len() - 1 {
                insert_in_place(i, current, array);
            }
        } else {
            if i > 0 {
                insert_in_place(i - 1, current, array);
            }
            break;
        }
    }
}

fn insert_in_place(index: usize, value: i32, array: &mut [i32]) {
    array[..=index].rotate_left(1);
    array[index] = value;
}
