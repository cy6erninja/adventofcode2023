mod day1;
mod day2;

use std::fs::read;
use std::path::Path;

fn main() {
    day2::conundrum_part2()
}

fn read_file(filepath: &str) -> String {
    let puzzle_input_filepath = Path::new(filepath);
    let content = read(puzzle_input_filepath).unwrap();
    let str_content = String::from_utf8(content).unwrap();

    str_content
}
