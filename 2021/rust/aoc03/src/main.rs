use std::env;
use std::fs;

#[derive(Debug)]
struct Summary {
    text: String,
    ones: u32,
    zeros: u32,
    oxgen_rating: char,
    co2_rating: char
}

impl Summary {
    fn add_digit(&mut self, ch: char) {
        if ch != '1' && ch != '0' {
            panic!("expected only 1 or 0");
        }

        self.text.push(ch);
        if ch == '1' {
            self.ones += 1;
        } else if ch == '0' {
            self.zeros += 1;
        }
    }
    fn set_oxgen_rating(&mut self) {
        self.oxgen_rating = if self.ones < self.zeros { '0' } else { '1' };
    }
    fn set_co2_rating(&mut self) {
        self.co2_rating = if self.ones < self.zeros { '1' } else { '0' }
    }
}

pub fn part_one(input: &String) -> usize {
    let split = input.split("\n");
    let length = match split.clone().next() {
        Some(val) => val.len(),
        None => 0
    };
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for index in 0..length {
        let mut summary = Summary { text: String::from(""), ones: 0, zeros: 0, oxgen_rating: '0', co2_rating: '0' };
        for raw_line in split.clone() {
            let ch = raw_line.chars().nth(index).unwrap();
            summary.add_digit(ch);
        }
        if summary.ones > summary.zeros {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_val = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_val = usize::from_str_radix(&epsilon, 2).unwrap();
    return gamma_val * epsilon_val;
}

fn narrow_for_value(lines: Vec<&str>, which: &str) -> String {
    let size = lines.clone().into_iter().next().unwrap().len();
    let mut cloned = lines.clone();
    for i in 0..size {
        let summaries = set_ratings(&cloned);
        let critiera = get_crit_strings(summaries);
        let target = match which {
            "co2" => critiera.0,
            "ox" => critiera.1,
            _ => unreachable!()
        };
        let target_char = target.chars().nth(i).unwrap();
        cloned = cloned
            .into_iter()
            .filter(|line| { 
                line.chars().nth(i).unwrap() == target_char
            }).collect();
        if cloned.len() <= 1 {
            break;
        }
    }
    return String::from(*cloned.first().unwrap())
}

fn set_ratings(lines: &Vec<&str>) -> Vec<Summary> {
    let line_length = lines[0].len();
    let mut summaries: Vec<Summary> = vec![];
    for index in 0..line_length {
        let mut summary = Summary { text: String::from(""), ones: 0, zeros: 0, oxgen_rating: '0', co2_rating: '0' };
        for raw_line in lines.clone() {
            let ch = raw_line.chars().nth(index).unwrap();
            summary.add_digit(ch);
        }

        summary.set_oxgen_rating();
        summary.set_co2_rating();
        summaries.push(summary);
    }
    return summaries;
}

fn get_crit_strings(summaries: Vec<Summary>) -> (String, String) {
    let co2_crit_string = summaries
        .iter()
        .map(|sum| sum.co2_rating)
        .collect();
    let oxgen_crit_string = summaries
        .iter()
        .map(|sum| sum.oxgen_rating)
        .collect();
    return (co2_crit_string, oxgen_crit_string);
}

pub fn part_two(input: &String) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let co2 = narrow_for_value(lines.clone(), "co2");
    let oxgen = narrow_for_value(lines.clone(), "ox");

    let co2_val = usize::from_str_radix(&co2, 2).unwrap();
    let oxgen_val = usize::from_str_radix(&oxgen, 2).unwrap();
    co2_val * oxgen_val
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
        let example = String::from("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010");
        assert_eq!(part_one(&example), 198);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010");
        assert_eq!(part_two(&example), 230);
    }
}