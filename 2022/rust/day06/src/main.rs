use std::collections::HashSet;

fn first_unique_chunk_index(input: &String, chunk_size: usize) -> usize {
    let mut window: HashSet<char> = HashSet::new();
    let mut result = 0;
    let (result, _) = input
        .chars()
        .enumerate()
        .find(|(idx, _ch)| {
            let index = *idx;
            if index < chunk_size {
                return false;
            }

            let prev_four = &input[(index - chunk_size)..index];
            window.extend(prev_four.chars());
            if window.len() == chunk_size {
                result = index;
                window.clear();
                return true;
            }
            window.clear();
            return false;
        })
        .unwrap();

    result
}

pub fn part_one(input: &String) -> usize {
    first_unique_chunk_index(input, 4)
}

pub fn part_two(input: &String) -> usize {
    first_unique_chunk_index(input, 14)
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
        let example = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part_one(&example), 7);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part_two(&example), 19);
    }
}
