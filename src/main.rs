use std::fs::{read};
use std::path::Path;

fn main() {
    let puzzle_input_filepath = Path::new("assets/puzzle_input.txt");

    let content = read(puzzle_input_filepath).unwrap();
    let str_content = String::from_utf8(content).unwrap();

    let mut result:u32 = 0;

    for s in str_content.split('\n') {
        println!("{:?}", s);

        let mut v: Vec<String> = Vec::new();
        v.push(String::from(""));

        for x in s.chars() {
            match x {
                '0'..='9' => v.push(String::from(x)),
                _ => {}
            }
        }

        v = v.into_iter().filter(|x| x.len() > 0).collect();

        println!("{:?}", v);

        result += String::from(format!("{}{}", v.first().unwrap(), v.last().unwrap()))
            .parse::<u32>().unwrap();
    }

    println!("{:?}", result);
}
