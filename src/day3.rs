use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Pointer};

pub fn gear_ratios_task1() {
    let raw_engine_schematic = crate::read_file("assets/puzzle3.txt");
    let numbers = EnginePartNumberCells::try_from(raw_engine_schematic).unwrap();

    println!("{}", numbers.sum_part_numbers())
}

pub fn gear_ratios_task2() {
    let raw_engine_schematic = crate::read_file("assets/puzzle3.txt");
    let numbers = EnginePartNumberCells::try_from(raw_engine_schematic).unwrap();

    let mut gear_ratio = 0;
    for first in numbers.cells.iter() {
        for second in numbers.cells.iter() {
            if first == second {
                continue;
            }

            if first.get_intersection(second) {
                gear_ratio += first.number * second.number;
            }
        }
    }

    // Loop goes over each number twice, so we need to divide the result by 2.
    println!("GEAR RATIO: {}", gear_ratio / 2);
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    top_right: Point,
    bottom_right: Point,
    bottom_left: Point
}



impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("{:?}", self.top_left);
        println!("{:?}", self.top_right);
        println!("{:?}", self.bottom_right);
        println!("{:?}", self.bottom_left);

        Ok(())
    }
}

/*
 * Since we have a condition, that number is a part number only if it has a symbol somewhere around
 * it, we must consider one line above and one line below the line with the actual number.
 */
#[derive(Debug)]
struct EnginePartNumberCell {
    number: usize,
    lines_around: [String; 3],
    area: Rectangle
}

impl Display for EnginePartNumberCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        println!("");
        for line in self.lines_around.iter() {
            println!("{}", line);
        }

        println!("{}", self.area);

        Ok(())
    }
}

#[derive(Debug)]
struct EnginePartNumberCells {
    cells: VecDeque<EnginePartNumberCell>
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

    fn get_intersection(&self,  cmp: &EnginePartNumberCell) -> bool {
        if self.area.bottom_right.x >= cmp.area.top_left.x
            && self.area.bottom_right.y >= cmp.area.top_left.y
            && self.area.top_left.x <= cmp.area.bottom_right.x
            && self.area.top_left.y <= cmp.area.bottom_right.y {

            let area = &self.area;
            let local_area = self.to_local_coordinates();

            // Looking for overlap area.
            // We create overlap in local coordinates in relation to self so that we could find symbols lying on the intersection.
            let overlap = Rectangle {
                top_left: Point {
                    x: max(self.area.top_left.x, cmp.area.top_left.x) + local_area.top_left.x - area.top_left.x,
                    y: max(self.area.top_left.y, cmp.area.top_left.y) + local_area.top_left.y - area.top_left.y
                },
                top_right: Point {
                    x: min(self.area.top_right.x, cmp.area.top_right.x) + local_area.top_right.x - area.top_right.x,
                    y: max(self.area.top_right.y, cmp.area.top_right.y) + local_area.top_right.y - area.top_right.y
                },
                bottom_right: Point {
                    x: min(self.area.bottom_right.x, cmp.area.bottom_right.x) + local_area.bottom_right.x - area.bottom_right.x,
                    y: min(self.area.bottom_right.y, cmp.area.bottom_right.y) + local_area.bottom_right.y - area.bottom_right.y
                },
                bottom_left: Point {
                    x: max(self.area.bottom_left.x, cmp.area.bottom_left.x) + local_area.bottom_left.x - area.bottom_left.x,
                    y: min(self.area.bottom_left.y, cmp.area.bottom_left.y) + local_area.bottom_left.y - area.bottom_left.y
                },
            };

            for line in overlap.top_right.y..=overlap.bottom_right.y {
                if self.lines_around[line][overlap.top_left.x..=overlap.top_right.x].contains('*') {
                    return true;
                }
            }
        }

        false

    }

    // We store lines inside the Rectanble and we store its boundaries in relation to the whole schematic. i
    // This function converts those boundaries to local coordinates.
    fn to_local_coordinates(&self) -> Rectangle {
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            top_right: Point { x: self.area.top_right.x - self.area.top_left.x, y: self.area.top_right.y - self.area.top_left.y},
            bottom_right: Point { x: self.area.bottom_right.x - self.area.top_left.x, y: self.area.bottom_right.y - self.area.top_left.y},
            bottom_left: Point { x: self.area.bottom_left.x - self.area.top_left.x, y: self.area.bottom_left.y - self.area.top_left.y}
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
       self.top_left == other.top_left && self.top_right == other.top_right
        && self.bottom_right == other.bottom_right && self.bottom_left == other.bottom_left
    }
}

impl PartialEq<Self> for EnginePartNumberCell {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area
    }
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
        // let mut lines: Vec<String> = value.split("\n").map(|l| l.to_string()).collect();
        let lines: Vec<&str> = value.split("\n").collect();
        let mut modified_lines: Vec<String> = Vec::new();
        
        // Surround all lines with dots to make it easier to form a box around the number.
        let line_len = lines.first().unwrap().len();

        modified_lines.push(".".repeat(line_len + 2));
        for l in lines {
            modified_lines.push(format!(".{}.", l));
        }
        modified_lines.push(".".repeat(line_len + 2));

        for (i, line) in modified_lines.iter().enumerate() {
            // Every line can contain multiple part numbers. We need to locate them and save start position and end position of each.
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
                        // If we have found a symbol other than number, we understand that previously found number has ended.
                        if start.is_some() && end.is_some() {
                            start_end.push([start.unwrap(), end.unwrap()]);

                            start = None;
                            end = None;
                        }
                    }
                }
            }

            /*  
             *  In case the last char of a string was a number, the second branch of previous match didn't execute,
             *  but we need to save the last found number to the vector anyway.
            */
            if start.is_some() && end.is_some() {
                start_end.push([start.unwrap(), end.unwrap()]);
                start = None;
                end = None;
            }

            // We know where our numbers start and end, now we need to form a box of symbols around them.
            for se in start_end {
                let mut area = Rectangle {
                    top_left: Point { x: 0, y: 0 },
                    top_right: Point { x: 0, y: 0 },
                    bottom_right: Point { x: 0, y: 0 },
                    bottom_left: Point { x: 0, y: 0 },
                };

                let number = modified_lines.get(i).unwrap()[se[0]..=se[1]]
                    .to_string().parse::<usize>().unwrap();

                // We need to adjust indexed to form a box of symbols around the number.
                let mut start_index = se[0];
                let mut end_index = se[1];

                start_index -= 1;
                end_index += 1;

                area.top_left = Point { x: start_index, y: i - 1 };
                area.top_right = Point { x: end_index, y: i - 1 };

                let middle_line = modified_lines.get(i).unwrap()[start_index..=end_index].to_string();
                area.bottom_right = Point { x: end_index, y: i + 1 };
                area.bottom_left = Point { x: start_index, y: i + 1 };
                let top_line = modified_lines.get(i - 1)
                    .unwrap()[start_index..=end_index].to_string();

                let bottom_line = modified_lines.get(i + 1)
                    .unwrap()[start_index..=end_index].to_string();

                engine_part_number_cells.cells.push_back(
                    EnginePartNumberCell {
                        number,
                        lines_around: [
                            top_line,
                            middle_line,
                            bottom_line
                        ],

                        area,
                    }
                );
            }

        }

        Ok(engine_part_number_cells)
    }
}
