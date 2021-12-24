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

pub fn part_one(input: &String) -> usize {
    let parsed_lines: Vec<Line> = input.lines().map(|raw_line| Line::parse(raw_line).unwrap()).collect();
    println!("{:?}", parsed_lines);
    0
}

fn main() {
    let contents = std::fs::read_to_string("input").unwrap();
    let result = part_one(&contents);
    println!("part 1");
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
    // #[test]
    // fn it_works_v2() {
    //     assert_eq!(part_two(&example), 1924);
    // }
}