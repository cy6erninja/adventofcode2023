use std::str::FromStr;

use regex::Regex;

use crate::read_file;

#[derive(Debug)]
struct IslandIslandAlmanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>, 
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    dest_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl Map {
    fn is_in_range(&self, value: usize) -> bool {
        self.source_range_start <= value && value < self.source_range_start + self.range_length
    }

    fn convert(&self, value: usize) -> usize {
        // Assume that map does not influence the value.
        let result = value;

        if self.is_in_range(value) {
            return self.dest_range_start + (value - self.source_range_start);
        }

        result
    }
}

impl IslandIslandAlmanac {
    fn get_seed_locations(&self) -> Vec<usize> {
        self.seeds.iter().map(|seed| {
            let mut result = *seed;
            // println!("seed: {}", result);
            for map in &self.seed_to_soil {
                result = map.convert(*seed);
            }

            // We need to apply only first map that can convert the value and ignore the rest.
            result = match &self.seed_to_soil.iter().find(|map| map.is_in_range(result)) {
                Some(map) => map.convert(result),
                None => result,
            };

            // println!("soil: {}", result);

            result = match &self.soil_to_fertilizer.iter().find(|map| map.is_in_range(result)) {
                Some(map) => map.convert(result),
                None => result,
            };

            // println!("fertilizer: {}", result);

            result = match &self.fertilizer_to_water.iter().find(|map| map.is_in_range(result)) {
                Some(map) => map.convert(result),
                None => result,
            };

            // println!("water: {}", result);

            result = match &self.water_to_light.iter().find(|map| map.is_in_range(result)) {
                Some(map) => map.convert(result),
                None => result,
            };
           
            // println!("light: {}", result);

            result = match &self.light_to_temperature.iter().find(|map| map.is_in_range(result)) {
                Some(map) => map.convert(result),
                None => result,
            };

            // println!("temperature: {}", result);

            result = match &self.temperature_to_humidity.iter().find(|map| map.is_in_range(result)) {
                Some(map) => map.convert(result),
                None => result,
            };

            // println!("humidity: {}", result);

            result = match &self.humidity_to_location.iter().find(|map| map.is_in_range(result)) {
                Some(map) => map.convert(result),
                None => result,
            };

            // println!("localtion: {}", result);

            result
        }).collect::<Vec<usize>>()
    }
}

impl From<Vec<usize>> for Map {
    fn from(v: Vec<usize>) -> Self {
        Map {
            dest_range_start: v[0],
            source_range_start: v[1],
            range_length: v[2],
        }
    }
}

impl FromStr for IslandIslandAlmanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac = IslandIslandAlmanac {
            seeds: Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        };

        let mut last_bucket = &mut Vec::new();
        for line in s.lines() {
            println!("!!!!!!!{}", line);
        
            let line = line.trim();

            // Skip empty lines.
            if line.is_empty() {continue};

            let re_title = r"^([\S ]+):(.+)*$";
            let re_nums = r"^(\d+) (\d+) (\d+)$";
            let caps = match Regex::new(re_title).unwrap().captures(line) {
                Some(caps) => caps,
                None => match Regex::new(re_nums).unwrap().captures(line) {
                    Some(caps) => caps,
                    None => continue,
                }
            };

            match &caps[1] {
                "seeds" => {
                    almanac.seeds = caps[2].trim().split_whitespace().map(|word| {
                        println!("---->{}", word);
                        word.parse::<usize>().unwrap()
                   }).collect::<Vec<usize>>()
                },
                "seed-to-soil map" => last_bucket = &mut almanac.seed_to_soil,
                "soil-to-fertilizer map" => last_bucket = &mut almanac.soil_to_fertilizer,
                "fertilizer-to-water map" => last_bucket = &mut almanac.fertilizer_to_water,
                "water-to-light map" => last_bucket = &mut almanac.water_to_light,
                "light-to-temperature map" => last_bucket = &mut almanac.light_to_temperature,
                "temperature-to-humidity map" => last_bucket = &mut almanac.temperature_to_humidity,
                "humidity-to-location map" => last_bucket = &mut almanac.humidity_to_location,
                _ => {
                    let map: Map = vec![caps[1].parse().unwrap(), caps[2].parse().unwrap(), caps[3].parse().unwrap()].into();
                    last_bucket.push(map);
                },
            };
        }

        Ok(almanac)
    }

}

impl PartialEq for IslandIslandAlmanac {
    fn eq(&self, other: &Self) -> bool {
        self.seeds == other.seeds
            && self.seed_to_soil == other.seed_to_soil
            && self.soil_to_fertilizer == other.soil_to_fertilizer
            && self.fertilizer_to_water == other.fertilizer_to_water
            && self.water_to_light == other.water_to_light
            && self.light_to_temperature == other.light_to_temperature
            && self.temperature_to_humidity == other.temperature_to_humidity
            && self.humidity_to_location == other.humidity_to_location
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.dest_range_start == other.dest_range_start
            && self.source_range_start == other.source_range_start
            && self.range_length == other.range_length
    }
}

pub fn part1() {
    let input_puzzle = read_file("assets/puzzle5.txt");
    let mut almanac = IslandIslandAlmanac::from_str(&input_puzzle).unwrap();

    println!("Min location is: {:?}", almanac.get_seed_locations().iter().min().unwrap());
}

pub fn part2() {
    // TODO: Reimplement part 2 with a different approach to make it faster.
    //
    // Part 2 requires us to treat seeds differently. Every second seed is a range size, while every first seed is the start of the range.
    // The simple solution implemented here, adds all the numbers from all the ranges to a single Vec and then goes over them as in part 1.
    // This is not efficient and it takes long time(probably hours) to calculate the result.
    // I think I came up with a different approach in my mind. The approach sounds as follows:
    // - We have seed ranges on the first line.
    // - Every map that follows(e.g. seed-to-soil map) simply splits the range into smaller ranges along with shifting some values.
    // - Even if some numbers in ranges are duplicated, it does not matter, because we are interested in the minimum value.
    // - After going through all the maps, we will have much more ranges, not of seeds, but of locations.
    // - Eventually, we just need to get all the minimum values from all the ranges and find the minimum value among them.

    let input_puzzle = read_file("assets/puzzle5.txt");
    let mut almanac = IslandIslandAlmanac::from_str(&input_puzzle).unwrap();

    let mut newseeds: Vec<usize> = Vec::new();
    for (i, seed) in almanac.seeds.iter().enumerate() {
        if i % 2 > 0 {
            continue;
        }

        if i+1 < almanac.seeds.len() {
            // println!("Range: {:?}", (*seed..*seed + almanac.seeds[i+1]).collect::<Vec<usize>>());
            // return;
            newseeds.append((*seed..*seed + almanac.seeds[i+1]).collect::<Vec<usize>>().as_mut());
        }
    }
    almanac.seeds = newseeds;
    
    println!("Min location is: {:?}", almanac.get_seed_locations().iter().min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2

            soil-to-fertilizer map:
            0 15 37

            fertilizer-to-water map:
            49 53 8

            water-to-light map:
            88 18 7

            light-to-temperature map:
            45 77 23

            temperature-to-humidity map:
            0 69 1

            humidity-to-location map:
            60 56 37
        ";

        let almanac: IslandIslandAlmanac = input.parse().unwrap();
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            almanac.seed_to_soil,
            vec![
                Map {
                    dest_range_start: 50,
                    source_range_start: 98,
                    range_length: 2,
                },
            ]
        );
        assert_eq!(
            almanac.soil_to_fertilizer,
            vec![
                Map {
                    dest_range_start: 0,
                    source_range_start: 15,
                    range_length: 37,
                },
            ]
        );
        assert_eq!(
            almanac.fertilizer_to_water,
            vec![
                Map {
                    dest_range_start: 49,
                    source_range_start: 53,
                    range_length: 8,
                },
            ]
        );
        assert_eq!(
            almanac.water_to_light,
            vec![
                Map {
                    dest_range_start: 88,
                    source_range_start: 18,
                    range_length: 7,
                },
            ]
        );
        assert_eq!(
            almanac.light_to_temperature,
            vec![
                Map {
                    dest_range_start: 45,
                    source_range_start: 77,
                    range_length: 23,
                },
            ]
        );
        assert_eq!(
            almanac.temperature_to_humidity,
            vec![
                Map {
                    dest_range_start: 0,
                    source_range_start: 69,
                    range_length: 1,
                },
            ]
        );
        assert_eq!(
            almanac.humidity_to_location,
            vec![
                Map {
                    dest_range_start: 60,
                    source_range_start: 56,
                    range_length: 37,
                },
            ]
        );
    }

        #[test]
        fn test_get_seed_locations() {
            let almanac = IslandIslandAlmanac {
                seeds: vec![14],
                seed_to_soil: vec![
                    Map {
                        dest_range_start: 50,
                        source_range_start: 98,
                        range_length: 2,
                    },
                    Map {
                        dest_range_start: 52,
                        source_range_start: 50,
                        range_length: 48,
                    },
                ],
                soil_to_fertilizer: vec![
                    Map {
                        dest_range_start: 0,
                        source_range_start: 15,
                        range_length: 37,
                    },
                    Map {
                        dest_range_start: 37,
                        source_range_start: 52,
                        range_length: 2,
                    },
                    Map {
                        dest_range_start: 39,
                        source_range_start: 0,
                        range_length: 15,
                    },
                ],
                fertilizer_to_water: vec![
                    Map {
                        dest_range_start: 49,
                        source_range_start: 53,
                        range_length: 8,
                    },
                    Map {
                        dest_range_start: 0,
                        source_range_start: 11,
                        range_length: 42,
                    },
                    Map {
                        dest_range_start: 42,
                        source_range_start: 0,
                        range_length: 7,
                    },
                    Map {
                        dest_range_start: 57,
                        source_range_start: 7,
                        range_length: 4,
                    },
                ],
                water_to_light: vec![
                    Map {
                        dest_range_start: 88,
                        source_range_start: 18,
                        range_length: 7,
                    },
                    Map {
                        dest_range_start: 18,
                        source_range_start: 25,
                        range_length: 70,
                    },
                ],
                light_to_temperature: vec![
                    Map {
                        dest_range_start: 45,
                        source_range_start: 77,
                        range_length: 23,
                    },
                    Map {
                        dest_range_start: 81,
                        source_range_start: 45,
                        range_length: 19,
                    },
                    Map {
                        dest_range_start: 68,
                        source_range_start: 64,
                        range_length: 13,
                    },
                ],
                temperature_to_humidity: vec![
                    Map {
                        dest_range_start: 0,
                        source_range_start: 69,
                        range_length: 1,
                    },
                    Map {
                        dest_range_start: 1,
                        source_range_start: 0,
                        range_length: 69,
                    },
                ],
                humidity_to_location: vec![
                    Map {
                        dest_range_start: 60,
                        source_range_start: 56,
                        range_length: 37,
                    },
                    Map {
                        dest_range_start: 56,
                        source_range_start: 93,
                        range_length: 4,
                    },
                ],
            };

            let seed_locations = almanac.get_seed_locations();
            assert_eq!(seed_locations, vec![43]);
        }
}