use std::collections::VecDeque;

pub fn gear_ratios_task1() {
    let raw_engine_schematic = crate::read_file("assets/puzzle3.txt");
    let numbers = EnginePartNumberCells::try_from(raw_engine_schematic).unwrap();

    println!("{}", numbers.sum_part_numbers())
}

/*
 * Since we have a condition, that number is a part number only if it has a symbol somewhere around
 * it, we must consider one line above and one line below the line with the actual number.
 */
#[derive(Debug)]
struct EnginePartNumberCell {
    number: usize,
    lines_around: [String; 3]
}

impl EnginePartNumberCell {
    fn is_part_number(&self) -> bool {
        let symbols = &self.lines_around.join("");

        for s in symbols.chars() {
            match s {
                '0'..='9' => {},
                '.' => {},
                _ => {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug)]
struct EnginePartNumberCells {
    cells: VecDeque<EnginePartNumberCell>
}

impl EnginePartNumberCells {
    fn new() -> Self {
        EnginePartNumberCells {
            cells: VecDeque::new()
        }
    }

    fn sum_part_numbers(self) -> usize {
        self.filter(|c| c.is_part_number())
            .fold(0, |acc, x| acc + x.number )
    }
}

impl Iterator for EnginePartNumberCells {
    type Item = EnginePartNumberCell;

    fn next(&mut self) -> Option<Self::Item> {
        self.cells.pop_front()
    }
}

impl TryFrom<String> for EnginePartNumberCells {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut engine_part_number_cells = EnginePartNumberCells::new();
        let lines: Vec<&str> = value.split("\n").collect();

        for (i, line) in lines.iter().enumerate() {
            let mut start_end: Vec<[usize; 2]> = Vec::new();
            let mut start: Option<usize> = None;
            let mut end: Option<usize> = None;

            for (j, c) in line.chars().enumerate() {
                match c {
                    '0'..='9' => {
                        if start.is_none() {
                            start = Some(j);
                            end = Some(j);

                            continue;
                        }

                        end = Some(j);
                    },
                    _ => {
                        if start.is_some() && end.is_some() {
                            start_end.push([start.unwrap(), end.unwrap()]);

                            start = None;
                            end = None;
                        }
                    }
                }
            }

            // In case the last char of a string was a number.
            if start.is_some() && end.is_some() {
                start_end.push([start.unwrap(), end.unwrap()]);
                start = None;
                end = None;
            }

            for se in start_end {
                let number = lines.get(i).unwrap()[se[0]..=se[1]]
                    .to_string().parse::<usize>().unwrap();

                // We need to adjust indexed to form a box of symbols around the number.
                let mut start_index = se[0];
                let mut end_index = se[1];

                if start_index > 0 {
                    start_index -= 1;
                }

                if end_index < lines.get(i).unwrap().len() - 1 {
                    end_index += 1;
                }

                let mut top_line = ".".repeat(
                    lines.get(i).unwrap()[start_index..=end_index].len()
                );
                let middle_line = lines.get(i).unwrap()[start_index..=end_index].to_string();
                let mut bottom_line = ".".repeat(
                    lines.get(i).unwrap()[start_index..=end_index].len()
                );

                // Finally, we need to get real top and bottom lines if they exist.
                if i > 0 {
                    top_line = lines.get(i - 1)
                        .unwrap()[start_index..=end_index].to_string();
                }

                if i < lines.len() - 1 {
                    bottom_line = lines.get(i + 1)
                        .unwrap()[start_index..=end_index].to_string();
                }

                engine_part_number_cells.cells.push_back(
                    EnginePartNumberCell {
                        number,
                        lines_around: [
                            top_line,
                            middle_line,
                            bottom_line
                        ],
                    }
                );
            }

        }

        Ok(engine_part_number_cells)
    }
}
