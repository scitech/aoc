use std::collections::HashSet;

pub fn part_one(input: &String) -> usize {
    let mut window: HashSet<char> = HashSet::new();
    let mut result = 0;
    for (idx, _ch) in input.chars().enumerate() {
        if idx < 4 {
            continue;
        } else {
            let prev_four = &input[(idx-4)..idx];
            window.extend(prev_four.chars());
            if window.len() == 4 {
                result = idx;
                window.clear();
                break;
            }
            window.clear();
        }
    }
    result
}
pub fn part_two(input: &String) -> usize {
    let mut window: HashSet<char> = HashSet::new();
    let mut result = 0;
    for (idx, _ch) in input.chars().enumerate() {
        if idx < 14 {
            continue;
        } else {
            let prev_four = &input[(idx-14)..idx];
            window.extend(prev_four.chars());
            if window.len() == 14 {
                result = idx;
                window.clear();
                break;
            }
            window.clear();
        }
    }
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
        let example = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part_one(&example), 7);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part_two(&example), 19);
    }
}