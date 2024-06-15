use regex::Regex;


#[derive(Debug)]
struct ScratchCard {
    number: u32,
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

impl ScratchCard {
    fn points(&self) -> u32 {
        let mut matches: u32 = 0;
        for num in &self.winning_numbers {
            if self.playing_numbers.contains(num) {
                matches += 1;
            }
        }

        if matches > 0 {
            return 2_u32.pow(matches - 1);
        }

        0
    }
}

impl TryFrom<String> for ScratchCard {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
       let re = Regex::new(r"^Card\s*(\d+):\s*(.*)\s+\|\s*(.*)\s*$").unwrap(); 
       let captures = re.captures(&value).unwrap();
       if captures.len() < 3 {
           return Err("Invalid ScratchCard format");
       }

       let number = captures[1].parse::<u32>().unwrap();
       let playing_numbers = captures[2]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
       let winning_numbers = captures[3]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
       
       Ok(
        Self {
            number,
            winning_numbers,
            playing_numbers,
        }
       )
    }
}


pub fn count_scratchcard_points() {
    let puzzle: String = crate::read_file("assets/puzzle4.txt");
    let lines: Vec<&str> = puzzle.split("\n").collect();

    let scratchcards: Vec<ScratchCard> = lines.into_iter()
        .map(|line| ScratchCard::try_from(line.to_string()).unwrap()).collect();

    println!("{:?}", scratchcards.into_iter().map(|card| card.points()).sum::<u32>());
}
