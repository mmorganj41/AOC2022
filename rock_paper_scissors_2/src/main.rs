use phf::phf_map;
use std::fs::File;
use std::io::{prelude::*, BufReader};

static MOVE_DEFINITION: phf::Map<char, char> = phf_map! {
    'X' => 'A',
    'Y' => 'B',
    'Z' => 'C',
};

static WHO_WINS: phf::Map<char, char> = phf_map! {
    'A' => 'C',
    'B' => 'A',
    'C' => 'B',
};

static PLAY_POINTS: phf::Map<char, u32> = phf_map! {
    'A' => 1,
    'B' => 2,
    'C' => 3,
};

/// A = Rock, B = Paper, C = Scissors
static MOVE_ARRAY: [char; 3] = ['A', 'B', 'C'];

static SHIFT_BASED_ON_RESULT: phf::Map<char, usize> = phf_map! {
    'X' => 2,
    'Y' => 0,
    'Z' => 1,
};

/// X = Lose, Y = Draw, Z = Win
static SCORE_ARRAY_FROM_RESULT: phf::Map<char, u32> = phf_map! {
    'X' => 0,
    'Y' => 3,
    'Z' => 6,
};

fn main() {
    let score = parse_lines_part_1(read_file("input"));
    println!("Anticipated score for part one: {score}");
    let new_score = parse_lines_part_2(read_file("input"));
    println!("Anticipated score for part two: {new_score}");
}

fn read_file(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("Could not read file");
    BufReader::new(file)
}

fn parse_lines_part_1(reader: BufReader<File>) -> u32 {
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
    let first_move = first_move.chars().next().unwrap();

    let second_move = info.next().unwrap();
    let mut second_move = second_move.chars().next().unwrap();
    second_move = MOVE_DEFINITION[&second_move];

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

fn line_to_move_and_action(line: &str) -> (char, char) {
    let mut info = line.splitn(2, ' ');

    let other_move = info.next().unwrap();
    let other_move = other_move.chars().next().unwrap();

    let action = info.next().unwrap();
    let action = action.chars().next().unwrap();

    (other_move, action)
}

fn get_points_from_action_and_move(other_move: char, action: char) -> u32 {
    let starting_index = MOVE_ARRAY.iter().position(|r| r == &other_move).unwrap();

    let player_index = (starting_index + SHIFT_BASED_ON_RESULT[&action]) % MOVE_ARRAY.len();
    let player_move = MOVE_ARRAY[player_index];

    PLAY_POINTS[&player_move]
}

fn parse_lines_part_2(reader: BufReader<File>) -> u32 {
    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        let (other_move, action) = line_to_move_and_action(line);

        total_score += SCORE_ARRAY_FROM_RESULT[&action];

        total_score += get_points_from_action_and_move(other_move, action);
    }

    total_score
}
