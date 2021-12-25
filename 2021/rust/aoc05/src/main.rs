#[derive(Debug)]
struct Coord {
    x: u16,
    y: u16
}

impl Coord {
    fn parse(raw_coord: &str) -> Result<Coord, String> {
        let mut pos = raw_coord.split(",");
        let x: u16 = match pos.next() {
            Some(raw_pos) => raw_pos.parse().or(Err(format!("cant parse number for x, got {}", raw_pos))),
            None => Err(String::from("line parsing error: missing x coord"))
        }?;
        let y: u16 = match pos.next() {
            Some(raw_pos) => raw_pos.parse().or(Err(format!("cant parse number for y, got {}", raw_pos))),
            None => Err(String::from("line parsing error: missing y coord"))
        }?;
        Ok(Coord{x: x, y: y})
    }
}
#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord
}

impl Line {
    fn parse(raw_line: &str) -> Result<Line, String> {
        let mut coords = raw_line.split(" -> ");
        let start = match coords.next() {
            Some(raw_coord) => Coord::parse(raw_coord),
            None => Err(String::from("line parsing error: missing first coord"))
        }?;
        let end = match coords.next() {
            Some(raw_coord) => Coord::parse(raw_coord),
            None => Err(String::from("line parsing error: missing second coord"))
        }?;
        Ok(Line{ start: start, end: end })
    }
}

// rows
// [
//     [ 0 , 0, 0, ],
//     [ 0 , 0, 0, ],
// ]
// #[derive(Debug)]
struct OceanFloor {
    grid: Vec<Vec<usize>>
}
impl std::fmt::Display for OceanFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.grid {
            write!(f, "{:?}\n", row)?;
        }
        Ok(())
    }
}
impl OceanFloor {
    fn new(lines: &Vec<Line>) -> Result<OceanFloor, String> {
        let line_with_max_x = lines.iter()
            .max_by_key(|line| std::cmp::max(line.start.x, line.end.x));
        let max_x = match line_with_max_x {
            Some(line) => Ok(std::cmp::max(line.start.x, line.end.x)),
            None => Err(String::from("no max x item"))
        }?;
        let line_with_max_y = lines.iter()
            .max_by_key(|line| std::cmp::max(line.start.y, line.end.y));
        let max_y = match line_with_max_y {
            Some(line) => Ok(std::cmp::max(line.start.y, line.end.y)),
            None => Err(String::from("no max y item"))
        }?;

        
        Ok(OceanFloor{grid: vec![vec![0; max_x as usize + 1]; max_y as usize + 1]})
    }

    fn plot(&mut self, lines: &Vec<Line>) {
        for line in lines {
            let y1 = std::cmp::min(line.start.y, line.end.y);
            let x1 = std::cmp::min(line.start.x, line.end.x);
            let y2 = std::cmp::max(line.start.y, line.end.y);
            let x2 = std::cmp::max(line.start.x, line.end.x);

            if y1 == y2 {
                for x in x1..=x2 {
                    let row: &mut Vec<usize> = &mut self.grid[y1 as usize];
                    row[x as usize] += 1;
                }
            } else if x1 == x2 {
                for y in y1..=y2 {
                    let row: &mut Vec<usize> = &mut self.grid[y as usize];
                    row[x1 as usize] += 1;
                }
            }
        }
    }
    fn plot_with_diags(&mut self, lines: &Vec<Line>) {
        for line in lines {

            if line.start.y == line.end.y {
                for x in std::cmp::min(line.start.x, line.end.x)..=std::cmp::max(line.start.x, line.end.x) {
                    let row = &mut self.grid[line.start.y as usize];
                    row[x as usize] += 1;
                }
            } else if line.start.x == line.end.x {
                for y in std::cmp::min(line.start.y, line.end.y)..=std::cmp::max(line.start.y, line.end.y) {
                    let row = &mut self.grid[y as usize];
                    row[line.start.x as usize] += 1;
                }
            } else if (line.end.x as isize - line.start.x as isize).abs() == (line.end.y as isize - line.start.y as isize).abs() {
                let mut cursor = Coord{ x: line.start.x, y: line.start.y};
                while cursor.x != line.end.x {
                    let row = &mut self.grid[cursor.y as usize];
                    row[cursor.x as usize] += 1;
                    if cursor.x < line.end.x {
                        cursor.x += 1;
                    } else {
                        cursor.x -= 1;
                    }
                    if cursor.y < line.end.y {
                        cursor.y += 1;
                    } else {
                        cursor.y -= 1;
                    }
                }
                let row = &mut self.grid[cursor.y as usize];
                row[cursor.x as usize] += 1;
            }
        }
    }

    fn overlapping_lines_count(&self) -> usize {
        let mut count = 0;
        for row in &self.grid {
            for &cell in row {
                if cell > 1 {
                    count += 1;
                }
            }
        }
        return count;
    }
}

pub fn part_one(input: &String) -> usize {
    let parsed_lines: Vec<Line> = input.lines().map(|raw_line| Line::parse(raw_line).unwrap()).collect();
    let mut ocean = OceanFloor::new(&parsed_lines).unwrap();
    ocean.plot(&parsed_lines);
    ocean.overlapping_lines_count()
}
pub fn part_two(input: &String) -> usize {
    let parsed_lines: Vec<Line> = input.lines().map(|raw_line| Line::parse(raw_line).unwrap()).collect();
    let mut ocean = OceanFloor::new(&parsed_lines).unwrap();
    ocean.plot_with_diags(&parsed_lines);
    ocean.overlapping_lines_count()
}

fn main() {
    let contents = std::fs::read_to_string("input").unwrap();
    let result = part_one(&contents);
    println!("part 1");
    println!("{:?}", result);
    let result = part_two(&contents);
    println!("part 2");
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        let example = String::from("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2");
        assert_eq!(part_one(&example), 5);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2");
        assert_eq!(part_two(&example), 12);
    }
}