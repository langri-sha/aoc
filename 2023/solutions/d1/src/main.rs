use regex::Regex;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
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

fn part_two() -> i32 {
    let mut numbers = Vec::new();

    for line in read_lines() {
        match line {
            Ok(ln) => numbers.push(get_number(replace_words(ln.as_str()).as_str())),
            Err(e) => panic!("Error: {}", e),
        }
    }

    numbers.iter().sum::<i32>()
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    return reader.lines();
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
        }
        _ => {
            result.push(numbers.pop_front().unwrap());
            result.push(numbers.pop_back().unwrap());
        }
    }

    return result
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i32>()
        .unwrap();
}

fn replace_words(line: &str) -> String {
    let mut result = String::new();
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let re = Regex::new(r"(\d+|\D+)").unwrap();

    for cap in re.captures_iter(line) {
        if cap[0].parse::<i32>().is_ok() {
            result = result + &cap[0];
            continue;
        }

        for i in 0..cap[0].len() {
            for word in words {
                if cap[0][i..].starts_with(&word) {
                    let position = words.iter().position(|&w| w == word).unwrap();

                    result = result + &(position + 1).to_string();
                    continue;
                }
            }
        }
    }

    result
}
