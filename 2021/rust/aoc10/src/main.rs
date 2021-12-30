type Line = Vec<char>;
type Input = Vec<Line>;

fn parse_input(input: &String) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_matching_opening_bracket_for_closer(ch: char) -> char {
    match ch {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        '>' => '<',
        _ => unreachable!("whut")
    }
}

pub fn part_one(input: &String) -> u32 {
    let openers = String::from("[({<");
    let closers = String::from("])}>");
    // let openers_to_closers = HashMap::
    let parsed = parse_input(input);
    let mut stack: Vec<char> = vec![];
    let mut illegal_score: Vec<u32> = vec![];
    parsed.iter().for_each(|line| {
        for &ch in line.iter() {
            if openers.find(|c| c == ch).is_some() {
                stack.push(ch);
            } else if closers.find(|c| c == ch).is_some() {
                let opener = get_matching_opening_bracket_for_closer(ch);
                let last_ch = stack.pop().unwrap();
                if last_ch != opener {
                    illegal_score.push(match ch {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!()
                    });
                    break;
                }
            } else {
                unreachable!("char not one of accepted brackets");
            }
        };
    });
    illegal_score.iter().sum()
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