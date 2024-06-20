mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::fs::read;
use std::path::Path;

fn main() {
    day5::part2();
}

fn read_file(filepath: &str) -> String {
    let puzzle_input_filepath = Path::new(filepath);
    let content = read(puzzle_input_filepath).unwrap();
    let str_content = String::from_utf8(content).unwrap();

    str_content
}
