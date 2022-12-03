struct Rucksack {
	raw: String,
	compartments: [String; 2],
}

fn same_char_finder(a: &String, b: &String) -> Result<char, String> {
	a.chars()
		.find(|a_ch| b.chars().find(|b_ch| a_ch == b_ch).is_some())
		.ok_or(String::from("could not find same char in both strings"))
}

fn get_priority(ch: char) -> usize {
	if ch.is_lowercase() {
		ch as usize - 96
	} else {
		ch as usize - 64 + 26
	}
}

fn score_priorities(sacks: &Vec<Rucksack>) -> usize {
	sacks.iter().fold(0, |total, sack| {
		let ch = same_char_finder(&sack.compartments[0], &sack.compartments[1]).unwrap();
		total + get_priority(ch)
	})
}

pub fn part_one(input: &String) -> usize {
	let rucksacks: Vec<Rucksack> = input.lines().map(|raw_line| Rucksack{ 
		raw: String::from(raw_line), 
		compartments: [
			String::from(&raw_line[0..(raw_line.len() / 2)]),
			String::from(&raw_line[(raw_line.len() / 2)..raw_line.len()]),
		]
	}).collect();
	score_priorities(&rucksacks)
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
		let example = String::from("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
		assert_eq!(part_one(&example), 157);
	}
	// #[test]
	// fn it_works_v2() {
	//     let example = String::from("16,1,2,0,4,2,7,1,2,14");
	//     assert_eq!(part_two(&example), 168);
	// }
}