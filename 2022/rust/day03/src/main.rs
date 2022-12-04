fn has_common_char(strs: &[&str]) -> Result<char, String> {
    let [ first, rest @ .. ] = strs else {
		return Err(String::from("bad params"));
	};
    first
        .chars()
        .find(|a_ch| {
            rest.iter().fold(true, |all_have_match, next_str| {
                return all_have_match && next_str.chars().find(|b_ch| a_ch == b_ch).is_some();
            })
        })
        .ok_or(String::from("could not find same char in all strings"))
}

fn get_priority(ch: char) -> usize {
    if ch.is_lowercase() {
        ch as usize - 96
    } else {
        ch as usize - 64 + 26
    }
}

pub fn part_one(input: &String) -> usize {
    input
        .lines()
        .map(|raw_line| {
            get_priority(
                has_common_char(&[
                    &String::from(&raw_line[0..(raw_line.len() / 2)]),
                    &String::from(&raw_line[(raw_line.len() / 2)..raw_line.len()]),
                ])
                .unwrap(),
            )
        })
        .sum()
}

pub fn part_two(input: &String) -> usize {
    input
        .lines()
        // iterator is line-by-line, need to collect to make chunks
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            get_priority(
                has_common_char(&[
                    &String::from(chunk[0]),
                    &String::from(chunk[1]),
                    &String::from(chunk[2]),
                ])
                .unwrap(),
            )
        })
        .sum()
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
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(part_one(&example), 157);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(part_two(&example), 70);
    }
}
