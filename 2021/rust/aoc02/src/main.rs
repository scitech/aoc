use std::env;
use std::fs;

struct Position {
    x: isize,
    y: isize,
    aim: isize,
}

impl Position {
    fn new() -> Position {
        Position{ x: 0, y: 0, aim: 0 }
    }

    fn do_move(&mut self, delta: Move) {
        match delta.direction {
            Direction::Forward => {
                self.x += delta.size;
            },
            Direction::Down => {
                self.y += delta.size;
            },
            Direction::Up => {
                self.y -= delta.size;
            },
        }
    }

    fn do_move_v2(&mut self, delta: Move) {
        match delta.direction {
            Direction::Forward => {
                self.x += delta.size;
                self.y += delta.size * self.aim;
            },
            Direction::Down => {
                self.aim += delta.size;
            },
            Direction::Up => {
                self.aim -= delta.size;
            },
        }
    }
}

enum Direction {
    Forward,
    Down,
    Up
}

struct Move {
    direction: Direction,
    size: isize
}

impl Move {
    fn parse(line: &str) -> Result<Move, String> {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() != 2 {
            return Err(String::from("moves should be a word and a number"));
        }

        let direction = match parts[0] {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(String::from("unknown direction")),
        }?;
        let amount: isize = parts[1].parse().unwrap();
        return Ok(Move{ direction: direction, size: amount });
    }
}

pub fn part_one(input: &String) -> isize {
    let mut pos = Position::new();
    for raw_line in input.split("\n") {
        let move_line = Move::parse(raw_line).unwrap();
        pos.do_move(move_line);
    }
    return pos.x * pos.y
}
pub fn part_two(input: &String) -> isize {
    let mut pos = Position::new();
    for raw_line in input.split("\n") {
        let move_line = Move::parse(raw_line).unwrap();
        pos.do_move_v2(move_line);
    }
    return pos.x * pos.y
}

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input");
    let contents = fs::read_to_string(path).unwrap();
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
        let example = String::from("forward 5
down 5
forward 8
up 3
down 8
forward 2");
        assert_eq!(part_one(&example), 150);
    }

    #[test]
    fn it_works_v2() {
        let example = String::from("forward 5
down 5
forward 8
up 3
down 8
forward 2");
        assert_eq!(part_two(&example), 900);
    }
}