use std::ops::RangeInclusive;

fn range_from_assignment(assignment: &str) -> RangeInclusive<usize> {
	let mut nums = assignment.split("-");
	let start: usize = nums.next().unwrap().parse().unwrap();
	let end: usize = nums.next().unwrap().parse().unwrap();
	start..=end
}

fn ranges_from_pair(line: &str) -> Vec<RangeInclusive<usize>> {
	line
		.split(",")
		.map(|elf| range_from_assignment(elf))
		.collect()
}

pub fn part_one(input: &String) -> usize {
	let rows_with_containing_assignment: Vec<bool> = input
		.split("\n")
		.map(|line| {
			let ranges = ranges_from_pair(line);
			let first = &ranges[0];
			let second = &ranges[1];
			let first_contains_second = first.start() <= second.start() && first.end() >= second.end();
			let second_contains_first = second.start() <= first.start() && second.end() >= first.end();
			first_contains_second || second_contains_first
		})
		.collect();
	rows_with_containing_assignment.iter().fold(0, |total, row| {
		return if *row { total + 1 } else { total }
	})
}

pub fn part_two(input: &String) -> usize {
	let rows_with_containing_assignment: Vec<bool> = input
		.split("\n")
		.map(|line| {
			let ranges = ranges_from_pair(line);
			let first_contains_second = ranges[0].contains(ranges[1].start()) ||  ranges[0].contains(ranges[1].end());
			let second_contains_first = ranges[1].contains(ranges[0].start()) ||  ranges[1].contains(ranges[0].end());
			first_contains_second || second_contains_first
		})
		.collect();
	rows_with_containing_assignment.iter().fold(0, |total, row| {
		return if *row { total + 1 } else { total }
	})
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
		let example = String::from("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
		assert_eq!(part_one(&example), 2);
	}
	#[test]
	fn it_works_v2() {
		let example = String::from("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
	    assert_eq!(part_two(&example), 4);
	}
}