fn is_lowest_adjacent_pos(map: &Vec<Vec<u32>>, (y, x): (isize, isize)) -> bool {
    let val = map[y as usize][x as usize];
    let above = if y == 0 { None } else { Some((y - 1, x)) };
    let below = if y == (map.len() as isize - 1) { None } else {Some((y + 1, x))};
    let left = if x == 0 { None } else { Some((y, x - 1))};
    let right = if x == (map[y as usize].len() as isize - 1) { None } else { Some((y, x + 1))};
    let adacent_positions = vec![above, below, left, right];
    let positions_that_exist = adacent_positions.iter().filter(|pos| pos.is_some()).map(|pos| pos.unwrap());
    positions_that_exist.fold(true, |is_lowest, position| {
        let val_at_adjacent_pos = map[position.0 as usize][position.1 as usize];
        let is_lower = val < val_at_adjacent_pos;
        is_lowest && is_lower
    })
}

fn parse_to_2d_array(input: &String) -> Vec<Vec<u32>> {
    input.lines().map(|line| {
        line.chars().map(|ch| ch.to_digit(10).unwrap()).collect()
    }).collect()
}

pub fn part_one(input: &String) -> u32 {
    let map = parse_to_2d_array(input);
    let mut risk_levels = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_lowest_adjacent_pos(&map, (y as isize, x as isize)) {
                risk_levels.push(map[y as usize][x as usize] + 1)
            }
        }
    }
    risk_levels.iter().sum()
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
        let example = String::from("2199943210
3987894921
9856789892
8767896789
9899965678");
        assert_eq!(part_one(&example), 15);
    }
    // #[test]
    // fn it_works_v2() {
    //     let example = String::from("16,1,2,0,4,2,7,1,2,14");
    //     assert_eq!(part_two(&example), 168);
    // }
  }