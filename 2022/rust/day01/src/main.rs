
fn score_elfs(input: &String) -> Vec<isize> {
	input.split("\n\n")
	.map(|elf| { 
		let elf_foods: Vec<isize> = elf.split("\n").map(|string_elf| string_elf.parse().unwrap()).collect();
		elf_foods.iter().sum()
	}).collect()
}

pub fn part_one(input: &String) -> isize {
	let mut max_elf = 0;
	let elf_scores = score_elfs(input);
	for score in elf_scores {
		if score > max_elf {
			max_elf = score
		}
	}

	max_elf
}
pub fn part_two(input: &String) -> isize {
	let mut elf_scores = score_elfs(input);
	elf_scores.sort();
	let length = elf_scores.len();
	elf_scores[(length - 3)..length].iter().sum()
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
		let example = String::from("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");
		assert_eq!(part_one(&example), 24000);
	}

	#[test]
	fn it_works_v2() {
		let example = String::from("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");
		assert_eq!(part_two(&example), 45000);
	}
}