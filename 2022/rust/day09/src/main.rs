use std::collections::{HashMap, HashSet};

struct Move {
    direction: String,
    size: usize,
}

type Instructions = Vec<Move>;

fn parse_instructions(input: &String) -> Instructions {
    return input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let dir = parts.next().unwrap();
            let size = parts.next().unwrap().parse().unwrap();
            Move {
                direction: String::from(dir),
                size,
            }
        })
        .collect();
}

fn move_tail(head: [isize; 2], tail: [isize; 2]) -> [isize; 2] {
    let diff = [tail[0] - head[0], tail[1] - head[1]];
    if tail[0] == head[0] && diff[1].abs() == 2 {
        let new_y = if tail[1] < head[1] {
            tail[1] + 1
        } else {
            tail[1] - 1
        };
        [tail[0], new_y]
    } else if tail[1] == head[1] && diff[0].abs() == 2 {
        let new_x = if tail[0] < head[0] {
            tail[0] + 1
        } else {
            tail[0] - 1
        };
        [new_x, tail[1]]
    } else if ((tail[0] - head[0]).abs() + (tail[1] - head[1]).abs()) > 2 {
        let new_x = if tail[0] < head[0] {
            tail[0] + 1
        } else {
            tail[0] - 1
        };
        let new_y = if tail[1] < head[1] {
            tail[1] + 1
        } else {
            tail[1] - 1
        };
        [new_x, new_y]
    } else {
        tail
    }
}
fn process_instructions(moves: Instructions, n: usize) -> usize {
    let vectors = HashMap::from([("R", [1, 0]), ("U", [0, 1]), ("L", [-1, 0]), ("D", [0, -1])]);
    let mut tail_positions = HashSet::new();
    let mut rope = vec![[0, 0]; n];
    moves.iter().for_each(|mv| {
        (0..mv.size).for_each(|_| {
            let head_diff = vectors.get(mv.direction.as_str()).unwrap();
            rope[0] = [rope[0][0] + head_diff[0], rope[0][1] + head_diff[1]];
            (0..rope.len() - 1).for_each(|i| {
                rope[i + 1] = move_tail(rope[i], rope[i + 1]);
            });
            tail_positions.insert(rope[rope.len() - 1]);
        });
    });
    tail_positions.len()
}
pub fn part_one(input: &String) -> usize {
    let moves = parse_instructions(input);
    let result = process_instructions(moves, 2);
    result
}
pub fn part_two(input: &String) -> usize {
    let moves = parse_instructions(input);
    let result = process_instructions(moves, 10);
    result
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
        let example = String::from(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(part_one(&example), 13);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(part_two(&example), 1);
    }
    #[test]
    fn it_works_v2_2() {
        let example = String::from(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );
        assert_eq!(part_two(&example), 36);
    }
}
