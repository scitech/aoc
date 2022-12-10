#[derive(Debug)]
enum CommandName {
    Addx,
    Noop,
}
#[derive(Debug)]
struct Command {
    name: CommandName,
    param: Option<isize>,
}

fn parse_instructions(input: &String) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let name = match parts.next().unwrap() {
                "addx" => CommandName::Addx,
                "noop" => CommandName::Noop,
                _ => panic!("unrecognized command"),
            };
            let param = match parts.next() {
                Some(number) => Some(number.parse::<isize>().unwrap()),
                None => None,
            };
            Command { name, param }
        })
        .collect()
}

fn run_program(program: Vec<Command>) -> Vec<isize> {
    let mut x = 1;
    let mut cycles: Vec<isize> = vec![];
    program.iter().for_each(|cmd| match cmd.name {
        CommandName::Addx => {
            cycles.push(x);
            x = x + cmd.param.unwrap();
            cycles.push(x);
        }
        CommandName::Noop => {
            cycles.push(x);
        }
    });
    cycles
}

fn sum_cycles(cycles: Vec<isize>) -> isize {
    vec![
        cycles[18] * 20,
        cycles[58] * 60,
        cycles[98] * 100,
        cycles[138] * 140,
        cycles[178] * 180,
        cycles[218] * 220,
    ]
    .iter()
    .sum()
}

pub fn part_one(input: &String) -> isize {
    let program = parse_instructions(input);
    let cycles = run_program(program);
    sum_cycles(cycles)
}

fn print_sprite(cycles: Vec<isize>) -> String {
    let mut crt: Vec<Vec<&str>> = vec![
        vec!["."; 40],
        vec!["."; 40],
        vec!["."; 40],
        vec!["."; 40],
        vec!["."; 40],
        vec!["."; 40],
    ];
    let mut sprite_range = 0..=2;
    cycles.iter().enumerate().for_each(|(idx, x_val)| {
        let row = idx / 40;
        let col = idx % 40;
        let contains = sprite_range.contains(&(col as isize));
        if contains {
            crt[row][col] = "#";
        }
        sprite_range = (x_val - 1)..=(x_val + 1);
    });
    crt.iter().for_each(|row| {
        println!("{:?}", row.join(""));
    });
    crt.iter()
        .map(|row| row.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn part_two(input: &String) -> String {
    let program = parse_instructions(input);
    let cycles = run_program(program);
    print_sprite(cycles)
}

fn main() {
    let contents = std::fs::read_to_string("input").unwrap();
    let result = part_one(&contents);
    println!("part 1");
    println!("{:?}", result);
    println!("part 2:");
    part_two(&contents);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn first() {
        let example = String::from(
            "noop
addx 3
addx -5
noop
addx 1",
        );
        let parsed = parse_instructions(&example);
        let cycles = run_program(parsed);
        assert_eq!(cycles[0], 1);
        assert_eq!(cycles[1], 1);
        assert_eq!(cycles[2], 4);
        assert_eq!(cycles[3], 4);
        assert_eq!(cycles[4], -1);
        assert_eq!(cycles[5], -1);
        assert_eq!(cycles[6], -1);
        assert_eq!(cycles[7], 0);
    }

    #[test]
    fn it_works() {
        let example = String::from(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        );
        assert_eq!(part_one(&example), 13140);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        );
        assert_eq!(
            part_two(&example),
            String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            )
        );
    }
}
