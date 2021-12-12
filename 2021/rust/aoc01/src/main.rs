use std::env;
use std::fs;

enum Delta {
    None,
    Increase,
    Decrease
}

struct Measurement (usize, Delta);

fn determine_delta(before: usize, after: usize) -> Delta {
    return if before == 0 || before == after {
        Delta::None
    } else if before > after {
        Delta::Decrease
    } else {
        Delta::Increase
    };
}

fn count_increases(measurements: Vec<Measurement>) -> usize {
    let mut increases = 0;
    for measurement in measurements {
        increases = match measurement.1 {
            Delta::Increase => { increases + 1},
            _ => { increases }
        }
    }
    return increases;
}

fn parse_measurements(input: String) -> Vec<Measurement> {
    let mut results = vec![];
    let mut prev = 0;
    for line in input.split("\n") {
        let parsed: usize = line.parse().unwrap();
        let delta = determine_delta(prev, parsed);
        prev = parsed;
        results.push(Measurement(parsed, delta))
    }
    return results
}

fn part_one(input: String) {
    println!("part 1");
    let measurements = parse_measurements(input);
    let increases = count_increases(measurements);
    println!("{}", increases);
}

fn parse_measurements_v2(input: String) -> Vec<Measurement> {
    let mut results = vec![];
    let mut prev = 0;
    let mut window: Vec<usize> = vec![];
    let mut sum = 0;
    for line in input.split("\n") {
        let parsed: usize = line.parse().unwrap();
        if window.len() == 3 {
            window.remove(0);
        }
        window.push(parsed);
        if window.len() == 3 {
            sum = window.iter().sum();
        }
        let delta = determine_delta(prev, sum);
        prev = sum;
        results.push(Measurement(parsed, delta))
    }
    return results
}

fn part_two(input: String) {
    println!("part 2");
    let measurements = parse_measurements_v2(input);
    let increases = count_increases(measurements);
    println!("{}", increases);
}

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input");
    let contents = fs::read_to_string(path).unwrap();
    part_one(contents);

    path = env::current_dir().unwrap();
    path.push("input");
    let contents = fs::read_to_string(path).unwrap();
    part_two(contents);
}
