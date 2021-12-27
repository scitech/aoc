use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn charsets_from_signal_output(input: &str) -> Vec<HashSet<char>> {
    input.split_whitespace()
        .map(|word| HashSet::from_iter(word.chars()))
        .collect()
}

fn simple_digit_from_chars(chars: &HashSet<char>) -> Option<isize> {
    match chars.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None
    }
}

fn count_easy_digits(counts: &mut HashMap<isize, isize>, charsets: Vec<HashSet<char>>) -> &mut HashMap<isize, isize> {
    charsets.iter().fold(counts, |acc, curr| {
        let simple_digit = simple_digit_from_chars(curr);
        if let Some(digit) = simple_digit {
            if let Some(digit_count) = acc.get_mut(&digit) {
                *digit_count += 1;
            } else {
                acc.insert(digit, 1);
            }
        }
        acc
    })
}

pub fn part_one(input: &String) -> isize {
    let outputs = input.lines().map(|line| {
        let mut split = line.split(" | ");
        let _signal_pattern = split.next();
        let output_value = split.next().unwrap();
        charsets_from_signal_output(output_value)
    });

    let mut counts = HashMap::new();
    outputs.for_each(|output| {
        count_easy_digits(&mut counts, output);
    });
    counts.values().sum()
}

fn identify_digit(charsets: &Vec<HashSet<char>>, output: &HashSet<char>) -> char {
    let one = charsets.iter().find(|&item| item.len() == 2).unwrap();
    let four = charsets.iter().find(|&item| item.len() == 4).unwrap();
    let seven = charsets.iter().find(|&item| item.len() == 3).unwrap();
    let eight = charsets.iter().find(|&item| item.len() == 7).unwrap();
    let three = charsets.iter().find(|&item| (item.len() == 5) && item.is_superset(seven)).unwrap();
    let six = charsets.iter().find(|&item| (item.len() == 6) && !item.is_superset(one) && !item.is_superset(seven) && !item.is_superset(four)).unwrap();
    let nine = charsets.iter().find(|&item| {
        (item != six) && (item.len() == 6) && item.is_superset(seven) && item.is_superset(three) && item.is_superset(one) && item.is_superset(four)
    }).unwrap();
    let zero = charsets.iter().find(|&item| item.len() == 6 && item != six && item != nine).unwrap();
    let five = charsets.iter().find(|&item| (item.len() == 5) && item != three && nine.is_superset(item)).unwrap();
    let two = charsets.iter().find(|&item| (item.len() == 5) && !nine.is_superset(item)).unwrap();
    
    

    match output {
        _x if output == one => '1',
        _x if output == two => '2',
        _x if output == three => '3',
        _x if output == four => '4',
        _x if output == five => '5',
        _x if output == six => '6',
        _x if output == seven => '7',
        _x if output == eight => '8',
        _x if output == nine => '9',
        _x if output == zero => '0',
        x => {
            println!("{:?}", x);
            unreachable!()
        },
    }
}

fn number_val_from_signal_line(charsets: Vec<HashSet<char>>, outputs: Vec<HashSet<char>>) -> isize {
    outputs.iter().fold(String::from(""), |mut sum, curr_output| {
        let current_val = identify_digit(&charsets, curr_output);
        sum.push(current_val);
        sum
    }).parse().unwrap()
}

pub fn part_two(input: &String) -> isize {
    input.lines().fold(0, |sum, line| {
        let mut split = line.split(" | ");
        let signal_pattern = split.next().unwrap();
        let output_value = split.next().unwrap();
        let number = number_val_from_signal_line(charsets_from_signal_output(signal_pattern), charsets_from_signal_output(output_value));
        sum + number
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
        let example = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        assert_eq!(part_one(&example), 26);
    }

    #[test]
    fn test_identify_digit() {
        let example_text = String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");
        let sets = charsets_from_signal_output(&example_text);
        assert_eq!(identify_digit(&sets, &HashSet::from_iter("acedgfb".chars())), '8');
        assert_eq!(identify_digit(&sets, &HashSet::from_iter("cdfbe".chars())), '5');
        assert_eq!(identify_digit(&sets, &HashSet::from_iter("gcdfa".chars())), '2');
        assert_eq!(identify_digit(&sets, &HashSet::from_iter("dab".chars())), '7');
        assert_eq!(identify_digit(&sets, &HashSet::from_iter("cdfgeb".chars())), '6');
        assert_eq!(identify_digit(&sets, &HashSet::from_iter("cefabd".chars())), '9');
    }

    #[test]
    fn it_works_v2() {
        let example = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        assert_eq!(part_two(&example), 61229);
    }
}