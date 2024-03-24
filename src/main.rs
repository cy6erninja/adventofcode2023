fn main() {
    two::run();
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
        let puzzle_input_filepath = Path::new("assets/puzzle_input.txt");
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

            // if v.len() == 1 {
            //     result += String::from(v.first().unwrap()).parse::<u32>().unwrap();
            //     println!(
            //         "{:?}",
            //         String::from(v.first().unwrap()).parse::<u32>().unwrap()
            //     );
            //
            //
            //     continue;
            // }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}
