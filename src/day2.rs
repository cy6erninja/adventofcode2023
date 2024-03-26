use crate::read_file;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use regex::Regex;

pub fn conundrum_part1() {
    let raw_games = read_file("assets/puzzle2.txt");
    let games = Games::try_from(raw_games).unwrap();
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let possible_games = games.filter(|game| game.is_possible(&bag));
    let id_sum: u32 = possible_games.fold(0, |acc, game| acc + game.id);

    // Print final sum of relevant game ids.
    println!("{}", id_sum);
}

pub fn conundrum_part2() {
    let raw_games = read_file("assets/puzzle2.txt");
    let games = Games::try_from(raw_games).unwrap();

    println!("{}", games.sum_min_cubeset_powers());
}

#[derive(Debug, Clone)]
struct CubeSubset {
    red: u32,
    green: u32,
    blue: u32
}

type Bag = CubeSubset;

impl CubeSubset {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0
        }
    }
    fn power(&self) -> u32 {
        return &self.red * &self.green * &self.blue;
    }
}

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

    fn min_cubeset(&self) -> CubeSubset {
        let mut result = self.subsets[0].clone();

        for subset in &self.subsets {
            if result.red < subset.red {
                result.red = subset.red;
            }

            if result.green < subset.green {
                result.green = subset.green;
            }

            if result.blue < subset.blue {
                result.blue = subset.blue;
            }
        }

        println!("{:?}", result);

        result
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

    fn sum_min_cubeset_powers(self) -> u32 {
        self.fold(0, |acc , game| acc + game.min_cubeset().power())
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