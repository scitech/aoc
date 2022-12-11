use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    items: Vec<isize>,
    operand: String,
    operator: String,
    test_divisor: isize,
    test_ok_monkey: usize,
    test_fail_monkey: usize,
}

type MonkeyMap = HashMap<usize, Monkey>;

impl Monkey {
    fn parse(monkey_block: &str) -> Result<Monkey, String> {
        let lines: Vec<&str> = monkey_block.split("\n").collect();
        let starting_line = lines[1];
        let starting_items = starting_line.split(": ").skip(1).next().unwrap()
            .split(", ").map(|number| number.parse::<isize>().unwrap() ).collect();

        let operation_line = lines[2];
        let mut operation_parts = operation_line.split(": new = old ").skip(1).next().unwrap()
            .split(" ");
        let operator = String::from(operation_parts.next().unwrap());
        let operand = String::from(operation_parts.next().unwrap());

        let test_divisor = lines[3]
            .split("divisible by ").skip(1).next().unwrap()
            .parse::<isize>().unwrap();
        let test_ok_monkey_a = lines[4]
            .split("throw to monkey ").skip(1).next();
        let test_ok_monkey = test_ok_monkey_a.unwrap()
            .parse::<usize>().unwrap();
        let test_fail_monkey = lines[5]
            .split("throw to monkey ").skip(1).next().unwrap()
            .parse::<usize>().unwrap();

        return Ok(Monkey{
            items: starting_items,
            operator,
            operand,
            test_divisor,
            test_ok_monkey,
            test_fail_monkey
        });
    }

    fn map_op(&self, old: isize) -> isize {
        let parsed_operand = match self.operand.as_str() {
            "old" => old,
            anything_else => anything_else.parse::<isize>().unwrap()
        };
        match self.operator.as_str() {
            "+" => old + parsed_operand,
            "*" => old * parsed_operand,
            _ => panic!("unrecognized operand"),
        }
    }
}


pub fn part_one(input: &String) -> isize {
    let mut monkey_map: MonkeyMap = HashMap::new();
    for (idx, monkey_block) in input.split("\n\n").enumerate() {
        let monkey = Monkey::parse(monkey_block).unwrap();
        monkey_map.insert(idx, monkey);
    }
    println!("{:?}", monkey_map);
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
        let example = String::from(
            "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );
        assert_eq!(part_one(&example), 37);
    }
    // #[test]
    // fn it_works_v2() {
    //     let example = String::from("16,1,2,0,4,2,7,1,2,14");
    //     assert_eq!(part_two(&example), 168);
    // }
}
