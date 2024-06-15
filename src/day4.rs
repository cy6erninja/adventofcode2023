use std::collections::HashSet;

use regex::Regex;


#[derive(Debug)]
struct ScratchCard {
    number: u32,
    // Numbers that are winning in the game.
    winning_numbers: Vec<u32>,
    // My numbers on the card.
    playing_numbers: Vec<u32>,
    // My numbers that have won.
    matching_numbers: Vec<u32>,
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
       let winning_numbers: Vec<u32> = captures[2]
           .split(" ")
           .filter(|s| !s.is_empty())
           .map(|num| num.parse::<u32>().unwrap())
           .collect();
       let playing_numbers: Vec<u32> = captures[3]
           .split(" ")
           .filter(|s| !s.is_empty())
           .map(|num| num.parse::<u32>().unwrap())
           .collect();

        let winning_set: HashSet<&u32> = HashSet::from_iter(winning_numbers.iter());
        let playing_set: HashSet<&u32> = HashSet::from_iter(playing_numbers.iter());
        let matching_numbers: Vec<u32> = winning_set.intersection(&playing_set).map(|num| **num).collect();
       
       Ok(
        Self {
            number,
            winning_numbers,
            playing_numbers,
            matching_numbers
        }
       )
    }
}


// Day 4 Part 1
pub fn count_scratchcard_points() {
    let puzzle: String = crate::read_file("assets/puzzle4.txt");
    let lines: Vec<&str> = puzzle.split("\n").collect();

    let scratchcards: Vec<ScratchCard> = lines.into_iter()
        .map(|line| ScratchCard::try_from(line.to_string()).unwrap()).collect();

    println!("{:?}", scratchcards.into_iter().map(|card| card.points()).sum::<u32>());
}

// Day 4 Part 2
pub fn count_scratchcards() -> Result<(), String> {
    let puzzle: String = crate::read_file("assets/puzzle4.txt");
    let lines: Vec<&str> = puzzle.split("\n").collect();

    let scratchcards: Vec<ScratchCard> = lines.into_iter()
        .map(|line| ScratchCard::try_from(line.to_string()).unwrap()).collect();

    fn get_scratchcards_count(card: &ScratchCard, scratchcards: &[ScratchCard]) -> u32 {
        let matching_numbers = card.matching_numbers.len();
        if card.number as usize >= scratchcards.len() {
            return 1;
        }

        if matching_numbers == 0 {
            return 1;
        }

        if card.number as usize + matching_numbers >= scratchcards.len() {
            return 1 + scratchcards[card.number as usize .. scratchcards.len()]
                .iter().map(|c| 
                    get_scratchcards_count(c, scratchcards)
                ).sum::<u32>();
        }

        1 + scratchcards[card.number as usize .. card.number as usize + matching_numbers]
            .iter().map(|c| 
                get_scratchcards_count(c, scratchcards)
            ).sum::<u32>()
    }
    
    // To calculate the total scratchcards count, we need to count each scratchcard and recursively count the scratchcards that current scratchcard wins.
    let scratchcards_cnt = scratchcards.iter().map(|card| {
        get_scratchcards_count(&card, &scratchcards.as_slice())
    }).sum::<u32>();

    println!("Total scratchcards: {:?}", scratchcards_cnt);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::ScratchCard;

    #[test]
    fn test_scratchcard_try_from() -> Result<(), String> {
        let scratchcard = ScratchCard::try_from("Card 1: 1 2 3 4 5 | 6 7 8 9 10".to_string())?;
        assert_eq!(scratchcard.number, 1);
        assert_eq!(scratchcard.playing_numbers, vec![1, 2, 3, 4, 5]);
        assert_eq!(scratchcard.winning_numbers, vec![6, 7, 8, 9, 10]);

        Ok(())
    }
}
