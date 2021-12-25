
pub fn part_one(input: &String) -> usize {
    let mut fishes: Vec<usize> = input.split(",").map(|fish| fish.parse().unwrap()).collect();
    let days = 80;
    for _day in 0..days {
        let mut new_fish: Vec<usize> = vec![];
        for fish in &mut fishes {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }
        fishes.extend(new_fish);
    }
    fishes.len()
}
pub fn part_two(input: &String) -> usize {
    let input_fishes: Vec<usize> = input.split(",").map(|fish| fish.parse().unwrap()).collect();
    let mut fish_by_time_left: Vec<usize> = vec![0; 9];
    input_fishes.iter().for_each(|&fish| {
        fish_by_time_left[fish] += 1;
    });
    let days = 256;
    for _day in 0..days {
        let parents = fish_by_time_left[0];
        for i in 0..(fish_by_time_left.len() - 1) {
            fish_by_time_left[i] = fish_by_time_left[i + 1];
        }
        fish_by_time_left[6] += parents;
        fish_by_time_left[8] = parents;
    }
    fish_by_time_left.iter().sum()
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
        let example = String::from("3,4,3,1,2");
        assert_eq!(part_one(&example), 5934);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("3,4,3,1,2");
        assert_eq!(part_two(&example), 26984457539);
    }
}