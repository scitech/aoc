fn find_adjacent_positions(map: &Vec<Vec<u32>>, (y, x): (isize, isize)) -> Vec<(isize, isize)> {
    let above = if y == 0 { None } else { Some((y - 1, x)) };
    let below = if y == (map.len() as isize - 1) { None } else {Some((y + 1, x))};
    let left = if x == 0 { None } else { Some((y, x - 1))};
    let right = if x == (map[y as usize].len() as isize - 1) { None } else { Some((y, x + 1))};
    let adjacent_position_options = vec![above, below, left, right];
    adjacent_position_options.iter().filter(|pos| pos.is_some()).map(|pos| pos.unwrap()).collect()
}

fn is_lowest_adjacent_pos(map: &Vec<Vec<u32>>, (y, x): (isize, isize)) -> bool {
    let val = map[y as usize][x as usize];
    let positions_that_exist = find_adjacent_positions(map, (y, x));
    positions_that_exist.iter().fold(true, |is_lowest, position| {
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

fn collect_basin(map: &Vec<Vec<u32>>, position: (isize, isize), basin_pts: &mut Vec<(isize, isize)>) {
    let val = map[position.0 as usize][position.1 as usize];
    let already_in_basin = basin_pts.iter().find(|&&pt| (pt.0 == position.0) && (pt.1 == position.1)).is_some();
    if !already_in_basin {
        basin_pts.push(position);
    }
    let neighbors = find_adjacent_positions(map, position);
    neighbors.iter().for_each(|&neighbor_pos| {
        let neighbor_val = map[neighbor_pos.0 as usize][neighbor_pos.1 as usize];
        if neighbor_val != 9 && neighbor_val > val {
            collect_basin(map, neighbor_pos, basin_pts);
        }
    });
}

pub fn part_two(input: &String) -> usize {
    let map = parse_to_2d_array(input);
    let mut basins = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let yx_coords = (y as isize, x as isize);
            if is_lowest_adjacent_pos(&map, yx_coords) {
                basins.push(yx_coords)
            }
        }
    }

    let mut expanded_basins: Vec<usize> = basins.iter().map(|&basin_center| {
        let mut basin_pts = vec![];
        collect_basin(&map, basin_center, &mut basin_pts);
        basin_pts.len()
    }).collect();
    
    expanded_basins.sort();
    expanded_basins[expanded_basins.len() - 3..expanded_basins.len()].iter().fold(1, |total, curr_len| total * curr_len)
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
        let example = String::from("2199943210
3987894921
9856789892
8767896789
9899965678");
        assert_eq!(part_one(&example), 15);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("2199943210
3987894921
9856789892
8767896789
9899965678");
        assert_eq!(part_two(&example), 1134);
    }
  }