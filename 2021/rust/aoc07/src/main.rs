
fn energy_cost_to_pos(numbers: &Vec<isize>, pos: isize) -> isize {
    let diffs: Vec<isize> = numbers.iter().map(|num| num - pos).map(|diff| diff.abs()).collect();
    diffs.iter().sum()
}

pub fn part_one(input: &String) -> isize {
    let nums: Vec<isize> = input.split(",").map(|thing| thing.parse().unwrap()).collect();
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    let mut lowest_energy = energy_cost_to_pos(&nums, min);
    for i in min..=max {
        let candidate_energy = energy_cost_to_pos(&nums, i);
        if candidate_energy < lowest_energy {
            lowest_energy = candidate_energy;
        }
    }

    lowest_energy
}

fn real_fuel_cost(n: isize) -> isize {
    (0..=n).sum()
}

fn energy_costs_to_pos_v2(numbers: &Vec<isize>, pos: isize) -> isize {
    let result = numbers.iter().fold(std::collections::HashMap::new(), |mut all, num| {
        let diff = (num - pos).abs();
        if let Some(count) = all.get_mut(&diff) {
            *count += 1;
        } else {
            all.insert(diff, 1);
        }
        all
    });
    result.iter().map(|(key, value)| {
        let fuel_cost = real_fuel_cost(*key);
        fuel_cost * value
    }).sum()
}

// v. slow, ~40 seconds, but dont care
pub fn part_two(input: &String) -> isize {
    let nums: Vec<isize> = input.split(",").map(|thing| thing.parse().unwrap()).collect();
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    let mut lowest_energy = energy_costs_to_pos_v2(&nums, min);
    for i in min..=max {
        let candidate_energy = energy_costs_to_pos_v2(&nums, i);
        if candidate_energy < lowest_energy {
            lowest_energy = candidate_energy;
        }
    }

    lowest_energy
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
        let example = String::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(part_one(&example), 37);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(part_two(&example), 168);
    }
}