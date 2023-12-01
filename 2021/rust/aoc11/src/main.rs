#[derive(Debug)]
struct Octopus {
    value: u32
}

const GRID_SIZE: usize = 10;

struct Octopuses {
    grid: Vec<Vec<Octopus>>
}

fn find_neighbors(point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    if point.0 > 0 {
        if point.1 < GRID_SIZE - 1 {
            result.push((point.0 - 1, point.1 + 1))
        }
        if point.1 > 0 {
            result.push((point.0 - 1, point.1 - 1))
        }
    } else if point.0 < GRID_SIZE - 1 {
        if point.1 < GRID_SIZE - 1 {
            result.push((point.0 + 1, point.1 + 1))
        }
        if point.1 > 0 {
            result.push((point.0 + 1, point.1 - 1))
        }
    }
    return result;
}

impl Octopuses {
    fn step(&mut self) {
        let mut flashed_coords: Vec<(usize, usize)> = vec![];
        for (y, row) in self.grid.iter_mut().enumerate()  {
            for (x, oct) in row.iter_mut().enumerate() {
                oct.advance();
                if oct.value > 9 {
                    let mut neighbors = find_neighbors((y, x));
                    flashed_coords.append(&mut neighbors);
                }
            }
        }
        for coord in &flashed_coords {
            self.grid[coord.1][coord.0].value += 1;
        }
        for row in self.grid.iter_mut()  {
            for oct in row.iter_mut() {
                if oct.value > 9 {
                    oct.value = 0;
                }
            }
        }
    }

    fn run(&mut self) {
        for _ in 0..2 {
            self.step();
            self.print();
        }
    }

    fn print(&self) {
        for row in &self.grid {
            for col in row {
                print!("{}", col.value);
            }
            print!("\n");
        }
        print!("\n\n")
    }
}

impl From<char> for Octopus {
    fn from(c: char) -> Octopus {
        let value = c.to_digit(10).unwrap();
        Octopus {
            value
        }
    }
}
impl Octopus {
    fn advance(&mut self) {
        if self.value == 9 {
            self.value = 0
        } else {
            self.value += 1
        }
        
    }
}

// struct Position {
//     x: isize,
//     y: isize,
//     octopus: Octopus,
// }
fn parse_input(input: &String) -> Octopuses {
    let mut grid: Vec<Vec<Octopus>> = vec![];
    for input_line in input.lines() {
        let mut row = vec![];
        for input_char in input_line.chars() {
            row.push(Octopus::from(input_char));
        }
        grid.push(row);
    }
    return Octopuses { grid };
}
pub fn part_one(input: &String) -> isize {
    let mut octopuses = parse_input(input);
    octopuses.run();
    0
}

fn main() {
    let contents = std::fs::read_to_string("input").unwrap();
    let result = part_one(&contents);
    println!("part 1");
    println!("{:?}", result);
    // let result = part_two(&contents);
    // println!("part 2");
    // println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = String::from("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526");
        assert_eq!(part_one(&example), 1656);
    }
    // #[test]
    // fn it_works_v2() {
//         let example = String::from("5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526");
    //     assert_eq!(part_two(&example), 168);
    // }
}
