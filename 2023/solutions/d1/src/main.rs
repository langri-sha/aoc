use regex::Regex;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Part one: {}", part_one());
}

fn part_one() -> i32 {
    let mut numbers = Vec::new();

    for line in read_lines() {
        match line {
            Ok(ln) => numbers.push(get_number(ln.as_str())),
            Err(e) => panic!("Error: {}", e),
        }
    }

    numbers.iter().sum::<i32>()
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    return reader.lines()
}

fn get_number(line: &str) -> i32 {
    let re = Regex::new(r"\d").unwrap();

    let mut numbers = VecDeque::new();
    let mut result = Vec::new();

    for cap in re.captures_iter(line) {
        numbers.push_back(cap[0].parse::<i32>().unwrap());
    }

    match numbers.len() {
        0 => (),
        1 => {
            result.push(numbers[0]);
            result.push(numbers[0]);
        },
        _ => {
            result.push(numbers.pop_front().unwrap());
            result.push(numbers.pop_back().unwrap());
        },
    }

    return result.iter().map(|&n| n.to_string()).collect::<Vec<String>>().join("").parse::<i32>().unwrap();
}
