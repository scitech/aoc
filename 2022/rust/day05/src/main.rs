fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines().peekable();
    let first = lines.peek().unwrap();
    let num_stacks = (first.len() + 1) / 4;
    let mut stacks = vec![vec![]; num_stacks];

    lines.rev().for_each(|line| {
        line.chars().enumerate().skip(1).chunks(4).for_each(|(chunk_idx, chunk)| {
            let ch = chunk[0];
            if ch.is_alphabetic() {
                stacks[chunk_idx].push(ch);
            }
        });
    });
    stacks
}

fn parse_move(mv: &str) -> [usize; 3] {
    let split_parts: Vec<&str> = mv.split(" ").collect();
    [
        split_parts[1].parse::<usize>().unwrap(),
        split_parts[3].parse::<usize>().unwrap() - 1,
        split_parts[5].parse::<usize>().unwrap() - 1,
    ]
}

fn do_moves(moves: &str, stacks: &mut Vec<Vec<char>>) {
    moves.lines().for_each(|mv| {
        let [amount, source_index, target_index] = parse_move(mv);
        (0..amount).for_each(|_time| {
            let item = stacks[source_index].pop().unwrap();
            stacks[target_index].push(item);
        })
    })
}

fn do_moves_2(moves: &str, stacks: &mut Vec<Vec<char>>) {
    moves.lines().for_each(|mv| {
        let [amount, source_index, target_index] = parse_move(mv);
        let length = stacks[source_index].len();
        let mut items = stacks[source_index].split_off(length - amount);
        stacks[target_index].append(&mut items);
    })
}

pub fn part_one(input: &String) -> String {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = parse_stacks(split_input[0]);
    do_moves(split_input[1], &mut stacks);
    let last_chars = stacks.iter().map(|stack| stack[stack.len() - 1]).collect();
    last_chars
}

pub fn part_two(input: &String) -> String {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = parse_stacks(split_input[0]);
    do_moves_2(split_input[1], &mut stacks);
    let last_chars = stacks.iter().map(|stack| stack[stack.len() - 1]).collect();
    last_chars
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
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!(part_one(&example), "CMZ");
    }
    #[test]
    fn it_works_v2() {
        let example = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!(part_two(&example), "MCD");
    }
}
