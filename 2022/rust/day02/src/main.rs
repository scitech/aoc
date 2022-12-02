#[derive(Debug)]
struct Round(Shape, Shape);

impl Round {
	fn parse(line: &str) -> Result<Round, String> {
		let parts: Vec<&str> = line.split(" ").collect();
		if parts.len() != 2 {
			return Err(String::from("bad format"));
		}
		let me = match parts[0] {
			"A" => Ok(Shape::Rock),
			"B" => Ok(Shape::Paper),
			"C" => Ok(Shape::Scissors),
			val => Err(format!("unknown shape {:?}", val)),
		}?;
		let them = match parts[1] {
			"X" => Ok(Shape::Rock),
			"Y" => Ok(Shape::Paper),
			"Z" => Ok(Shape::Scissors),
			val => Err(format!("unknown shape {:?}", val)),
		}?;
		return Ok(Round(me, them))
	}

	fn score_outcome(&self, my_shape: &Shape) -> usize {
		if Shape::beats(my_shape, &self.0) { 6 } else if Shape::beats(&self.0, my_shape) { 0 } else { 3 }
	}

	fn find_expected_shape(&self) -> Shape {
		// (too lazy to factor out the common parts of fn parse)
		match &self.1 {
			// lose 
			Shape::Rock => Shape::loser(&self.0),
			// draw
			Shape::Paper => self.0.clone(),
			// win
			Shape::Scissors => Shape::winner(&self.0),
		}
	}

}

#[derive(Debug, Clone, PartialEq)]
enum Shape {
	Rock,
	Paper,
	Scissors,
}

impl Shape {
	fn score(shape: &Shape) -> usize {
		match shape {
			Shape::Rock => 1,
			Shape::Paper => 2,
			Shape::Scissors => 3,
		}
	}

	fn beats(a: &Shape, b: &Shape) -> bool {
		*b == Shape::loser(a)
	}
	fn winner(opponent: &Shape) -> Shape {
		match opponent {
			Shape::Rock => Shape::Paper,
			Shape::Paper => Shape::Scissors,
			Shape::Scissors => Shape::Rock,
		}
	}
	fn loser(opponent: &Shape) -> Shape {
		match opponent {
			Shape::Rock => Shape::Scissors,
			Shape::Paper => Shape::Rock,
			Shape::Scissors => Shape::Paper,
		}
	}
}

pub fn part_one(input: &String) -> usize {
	let rounds: Vec<Round> = input
		.split("\n")
		.map(|line| Round::parse(line).unwrap())
		.collect();
	
	rounds.iter().fold(0, |total, r| {
		total + Shape::score(&r.1) + r.score_outcome(&r.1)
	})
}
pub fn part_two(input: &String) -> usize {
	let rounds: Vec<Round> = input
		.split("\n")
		.map(|line| Round::parse(line).unwrap())
		.collect();
	
	rounds.iter().fold(0, |total, r| {
		let my_shape = r.find_expected_shape();
		total + Shape::score(&my_shape) + r.score_outcome(&my_shape)
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
		let example = String::from("A Y
B X
C Z");
		assert_eq!(part_one(&example), 15);
	}
	#[test]
	fn it_works_v2() {
		let example = String::from("A Y
B X
C Z");
		assert_eq!(part_two(&example), 12);
	}
}