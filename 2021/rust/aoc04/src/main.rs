#[derive(Debug)]
struct Board {
    rows: Vec<Vec<u8>>
}

fn does_row_win(row: &Vec<u8>, draws: &Vec<u8>) -> bool {
    row.iter().all(|&cell| draws.iter().any(|&item| item == cell))
}

fn find_winning_draw_pos(row: &Vec<u8>, draws: &Vec<u8>) -> Option<usize> {
    if does_row_win(row, draws) {
        let last_draw_in_row = draws.iter().rposition(|d| row.iter().any(|el| el == d))?;
        return Some(last_draw_in_row)
    }

    None
}

impl Board {
    fn new(lines: Vec<&str>) -> Board {
        let parsed_numbers = lines.iter()
        .map(|line| {
            let split = (*line).split_whitespace();
            return split.map(|n| {
                return (*n).trim().parse().unwrap()
            }).collect()
        }).collect();
        Board{rows: parsed_numbers}
    }

    fn find_winning_row(&self, draws: &Vec<u8>) -> Option<Vec<u8>> {
        self.rows.clone().into_iter()
            .find(|row| {
                return does_row_win(row, draws)
            })
    }

    fn find_winning_col(&self, draws: &Vec<u8>) -> Option<Vec<u8>> {
        let row_size = self.rows[0].len();
        let cols_as_rows: Vec<Vec<u8>> = (0..row_size).map(|col_index| self.rows.iter().map(|row| row[col_index]).collect()).collect();
        cols_as_rows.into_iter().find(|row| does_row_win(row, draws))
    }

    fn find_winner(&self, draws: &Vec<u8>) -> Option<Vec<u8>> {
        let fn_find_winning_col = || self.find_winning_col(draws);
        self.find_winning_row(draws).or_else(fn_find_winning_col)
    }

    fn sum_uncalled(&self, calls: &Vec<u8>) -> usize {
        let uncalled: Vec<usize> = self.rows.iter()
            .map(|row| {
                let filtered = row.iter().filter(|&&row_item| !calls.iter().any(|&call| call == row_item));
                let sum = filtered.fold(0, |sum, &item| sum + item as usize);
                return sum;
            }).collect();
        return uncalled.iter().sum();
    }
    
}

#[derive(Debug)]
struct Day4Input {
    draws: Vec<u8>,
    boards: Vec<Board>
}

struct Winner { 
    draw: u8,
    uncalled_sum: usize
}
impl Day4Input {
    fn find_winner(&self) -> Option<Winner> {
        let mut results: Vec<Option<Vec<u8>>> = vec![];
        for i in 0..self.draws.len() {
            let draws_to_this_point = self.draws[0..=i].to_vec();
            results = self.boards.iter()
                .map(|board| board.find_winner(&draws_to_this_point)).collect();
            if results.iter().any(|thing| thing.is_some()) {
                break;
            }
        }
        
        let winning_position = results.iter().position(|winner| winner.is_some())?;

        let winning_row = results[winning_position].as_ref()?;
        let winning_board = &self.boards[winning_position];
        let winning_draw_pos = find_winning_draw_pos(&winning_row, &self.draws)?;
        let winning_draw = self.draws[winning_draw_pos];
        let calls = self.draws[0..=winning_draw_pos].to_vec();
        let uncalled_sum = winning_board.sum_uncalled(&calls);
        Some(Winner{ draw: winning_draw, uncalled_sum: uncalled_sum})
    }
    fn find_last_winner(&self) -> Option<Winner> {
        let mut winners: Vec<(usize, Vec<u8>)> = vec![];
        for i in 0..self.draws.len() {
            let draws_to_this_point = self.draws[0..=i].to_vec();
            for x in 0..self.boards.len() {
                let board = &self.boards[x];
                if let Some(winner) = board.find_winner(&draws_to_this_point) {
                    if !winners.iter().any(|(index, _row)| *index == x) {
                        winners.push((x, winner));
                    }
                }
            }
            if winners.len() == self.boards.len() {
                break;
            }
        }
        
        let (winning_position, winning_row) = winners.last()?;
        let winning_board = &self.boards[*winning_position];
        let winning_draw_pos = find_winning_draw_pos(winning_row, &self.draws)?;
        let winning_draw = self.draws[winning_draw_pos];
        let calls = self.draws[0..=winning_draw_pos].to_vec();
        let uncalled_sum = winning_board.sum_uncalled(&calls); 
        Some(Winner{ draw: winning_draw, uncalled_sum: uncalled_sum})
    }

    fn parse(input: &String) -> Day4Input {
        let mut lines_iter = input.lines();
        let header = lines_iter.next().unwrap();
        let mut boards: Vec<Board> = vec![];
        while let Some(line) = lines_iter.next() {
            if line == "" {
                let board_text: Vec<&str> = vec![
                    lines_iter.next().unwrap(),
                    lines_iter.next().unwrap(),
                    lines_iter.next().unwrap(),
                    lines_iter.next().unwrap(),
                    lines_iter.next().unwrap(),
                ];
                let board = Board::new(board_text);
                boards.push(board);
            }
        }
        let parsed_header: Vec<u8> = header.split(",").map(|n| n.trim().parse().unwrap()).collect();
        Day4Input{draws: parsed_header, boards: boards}
    }
}

pub fn part_one(input: &String) -> usize {
    let parsed_input = Day4Input::parse(input);
    let winner = parsed_input.find_winner().unwrap();
    return winner.uncalled_sum * winner.draw as usize;
}
pub fn part_two(input: &String) -> usize {
    let parsed_input = Day4Input::parse(input);
    let winner = parsed_input.find_last_winner().unwrap();
    return winner.uncalled_sum * winner.draw as usize;
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
        let example = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");
        assert_eq!(part_one(&example), 4512);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");
        assert_eq!(part_two(&example), 1924);
    }
}