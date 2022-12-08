type Grid = Vec<Vec<usize>>;

fn parse_grid(input: &String) -> Grid {
    let mut grid: Grid = vec![];
    input.lines().for_each(|line| {
        let parsed_line: Vec<usize> = line
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(parsed_line);
    });
    grid
}

fn is_visible(grid: &Grid, (y, x): (usize, usize)) -> bool {
    if y == 0 || y == grid.len() - 1 {
        return true;
    } else if x == 0 || x == grid[0].len() - 1 {
        return true;
    }

    let value = grid[y][x];
    if (0..x).all(|col| grid[y][col] < value) {
        return true;
    };
    if (0..y).all(|row| grid[row][x] < value) {
        return true;
    };
    if ((y + 1)..grid.len()).all(|row| grid[row][x] < value) {
        return true;
    };
    if ((x + 1)..grid[0].len()).all(|col| grid[y][col] < value) {
        return true;
    };

    false
}
pub fn part_one(input: &String) -> usize {
    let grid = parse_grid(input);
    grid.iter().enumerate().fold(0, |total_visible, (y, row)| {
        total_visible
            + row.iter().enumerate().fold(0, |row_visible, (x, _value)| {
                return if is_visible(&grid, (y, x)) {
                    row_visible + 1
                } else {
                    row_visible
                };
            })
    })
}

pub fn part_two(input: &String) -> usize {
    let grid = parse_grid(input);
    grid.iter().enumerate().fold(0, |max, (y, row)| {
        let row_max = row.iter().enumerate().fold(0, |rm, (x, value)| {
            let left = if x == 0 {
                0
            } else {
                (1..x)
                    .rev()
                    .take_while(|col| grid[y][*col] < *value)
                    .count()
                    + 1
            };
            let right = if x == grid[0].len() - 1 {
                0
            } else {
                ((x + 1)..grid[0].len() - 1)
                    .take_while(|col| grid[y][*col] < *value)
                    .count()
                    + 1
            };
            let top = if y == 0 {
                0
            } else {
                (1..y)
                    .rev()
                    .take_while(|row| grid[*row][x] < *value)
                    .count()
                    + 1
            };
            let bottom = if y == grid.len() - 1 {
                0
            } else {
                ((y + 1)..grid.len() - 1)
                    .take_while(|row| grid[*row][x] < *value)
                    .count()
                    + 1
            };
            let score = left * right * top * bottom;
            std::cmp::max(score, rm)
        });
        std::cmp::max(row_max, max)
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
        let example = String::from(
            "30373
25512
65332
33549
35390",
        );
        assert_eq!(part_one(&example), 21);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from(
            "30373
25512
65332
33549
35390",
        );
        assert_eq!(part_two(&example), 8);
    }
}
