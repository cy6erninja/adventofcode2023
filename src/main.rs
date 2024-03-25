use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::fs::read;
use std::path::Path;
use regex::{Captures, Regex};

fn main() {
    day2_cube_conundrum_part1();
}

fn day2_cube_conundrum_part1() {
    struct CubeSubset {
        red: u32,
        green: u32,
        blue: u32
    }

    type Bag = CubeSubset;

    struct Game {
        id: u32,
        subsets: Vec<CubeSubset>
    }

    impl Game {
        fn is_possible(&self, bag: &Bag) -> bool {
            for subset in &self.subsets {
                if bag.red < subset.red || bag.green < subset.green || bag.blue < subset.blue {
                    return false;
                }
            }

            true
        }
    }

    struct Games {
        items: VecDeque<Game>
    }

    impl Games {
        fn new() -> Self {
            Games {
                items: VecDeque::new()
            }
        }
    }

    impl Iterator for Games {
        type Item = Game;

        fn next(&mut self) -> Option<Self::Item> {
           self.items.pop_front()
        }
    }

    impl TryFrom<String> for Games {
        type Error = ();

        fn try_from(value: String) -> Result<Self, Self::Error> {

            let mut games = VecDeque::new();

             for line in value.split('\n') {
                 let mut split_by_coloh = line.split(':');
                 let id_re = Regex::new(r"Game (\d+)").unwrap();
                 let Some(id) = id_re.captures(&split_by_coloh.next().unwrap())
                 // Here we need to return an error.
                     else {panic!("Can not parse game id.")};

                 let mut subsets: Vec<CubeSubset> = Vec::new();

                 for raw_subset in &mut split_by_coloh.next().unwrap().split(';').into_iter() {
                     let red_re = Regex::new(r"(\d+) red").unwrap();
                     let green_re = Regex::new(r"(\d+) green").unwrap();
                     let blue_re = Regex::new(r"(\d+) blue").unwrap();

                     let red = match red_re.captures(raw_subset) {
                         Some(red) =>  *&red[1].parse::<u32>().unwrap_or(0),
                         None => 0,
                     };

                     let green = match green_re.captures(raw_subset) {
                         Some(green) =>  *&green[1].parse::<u32>().unwrap_or(0),
                         None => 0,
                     };

                     let blue = match blue_re.captures(raw_subset) {
                         Some(blue) =>  *&blue[1].parse::<u32>().unwrap_or(0),
                         None => 0,
                     };

                     subsets.push(
                         CubeSubset {
                             red,
                             green,
                             blue,
                         }
                     );
                 }

                 games.push_back(Game {id: *&id[1].parse::<u32>().unwrap() , subsets });
             }

            Ok(Games { items: games })
        }
    }

    impl Debug for Games {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for game in &self.items {
                println!("ID: {}", &game.id);
                for subset in &game.subsets {
                    println!("Red: {}; Green: {}; Blue: {}", subset.red, subset.green, subset.blue);
                }
            }

            Ok(())
        }
    }

    let raw_games = read_file("assets/puzzle2.txt");
    let games = Games::try_from(raw_games).unwrap();
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    // println!("{:?}", games);
    let possible_games = games.filter(|game| game.is_possible(&bag));

    let id_sum: u32 = possible_games.fold(0, |acc, game| acc + game.id);
    // Print final sum of relevant game ids.
    println!("{}", id_sum);
}

fn day1_trebuchet_part2() {

}

fn day1_trebuchet_part1() {

}

fn read_file(filepath: &str) -> String {
    let puzzle_input_filepath = Path::new(filepath);
    let content = read(puzzle_input_filepath).unwrap();
    let str_content = String::from_utf8(content).unwrap();

    str_content
}

mod two {
    use std::fs::read;
    use std::path::Path;

    pub fn run() {
        let puzzle = read_file("assets/puzzle_input.txt");
        let puzzle_only_ints = replace_word_numbers_with_ints(puzzle);
        // let puzzle_only_ints = puzzle;
        calculate_puzzle_answer(puzzle_only_ints);
    }

    fn read_file(filepath: &str) -> String {
        let puzzle_input_filepath = Path::new(filepath);
        let content = read(puzzle_input_filepath).unwrap();
        let str_content = String::from_utf8(content).unwrap();

        str_content
    }

    fn replace_word_numbers_with_ints(mut puzzle: String) -> String {
        let subjects = [
            "zero",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
        ];

        let mut result = String::new();
        // Since we iterate over chars, we need to skip all the chars of the subject if we find one.
        let mut skip_left = 0;

        for (i, ch) in puzzle.chars().enumerate() {
            if skip_left > 0 {
                skip_left -= 1;
            }

            for (j, subj) in subjects.into_iter().enumerate() {
                let j = j as u32;

                if i + subj.len() > puzzle.len() {
                    continue;
                }

                let slice = &puzzle[i..i + subj.len()];
                if slice.eq(subj) {
                    // Don't like this going back and forth with types but it works.
                    result.push(j.to_string().chars().nth(0).unwrap());
                    skip_left += subj.len() - 1;

                    break;
                }
            }

            // If skip_left is 0 at this point, this means, that this char is not a part of subject.
            if skip_left == 0 {
                result.push(ch);
            }
        }

        result
    }

    fn calculate_puzzle_answer(puzzle: String) -> u32 {
        let mut result:u32 = 0;

        for s in puzzle.split('\n') {
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

            println!(
                "{:?}",
                String::from(
                    format!("{}{}", v.first().unwrap(), v.last().unwrap())
                ).parse::<u32>().unwrap()
            );


            result += String::from(format!("{}{}", v.first().unwrap(), v.last().unwrap()))
                .parse::<u32>().unwrap();
        }

        println!("{:?}", result);

        result
    }
}

mod one {
    use std::fs::read;
    use std::path::Path;

    pub fn run() {
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
}
