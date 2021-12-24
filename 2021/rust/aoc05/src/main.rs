pub fn part_one(input: &String) -> usize {
    println!("{}", input);
    0
}

fn main() {
    let contents = std::fs::read_to_string("input").unwrap();
    let result = part_one(&contents);
    println!("part 1");
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        let example = String::from("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2");
        assert_eq!(part_one(&example), 5);
    }
    // #[test]
    // fn it_works_v2() {
    //     assert_eq!(part_two(&example), 1924);
    // }
}