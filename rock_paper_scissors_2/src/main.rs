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
};

static PLAY_POINTS: phf::Map<char, u32> = phf_map! {
    'X' => 1,
    'Y' => 2,
    'Z' => 3,
};

fn main() {
    let score = parse_lines(read_file("input"));
    println!("Anticipated score: {score}");
}

fn read_file(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("Could not read file");
    BufReader::new(file)
}

fn parse_lines(reader: BufReader<File>) -> u32 {
    let mut total_score: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        let (other_move, player_move) = determine_moves(line);

        total_score += PLAY_POINTS[&player_move];

        total_score += get_points_from_winning(other_move, player_move);
    }

    total_score
}

fn determine_moves(line: &str) -> (char, char) {
    let mut info = line.splitn(2, ' ');

    let first_move = info.next().unwrap();
    let mut first_move = first_move.chars().next().unwrap();
    first_move = MOVE_DEFINITION[&first_move];

    let second_move = info.next().unwrap();
    let second_move = second_move.chars().next().unwrap();

    (first_move, second_move)
}

fn get_points_from_winning(other_move: char, player_move: char) -> u32 {
    if player_move == other_move {
        3
    } else if WHO_WINS[&player_move] == other_move {
        6
    } else {
        0
    }
}
