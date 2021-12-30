pub fn part_one(input: &String) -> isize {
    println!("{}", input);
    0
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
        let example = String::from("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]");
        assert_eq!(part_one(&example), 26397);
    }
    // #[test]
    // fn it_works_v2() {
    //     let example = String::from("16,1,2,0,4,2,7,1,2,14");
    //     assert_eq!(part_two(&example), 168);
    // }
  }