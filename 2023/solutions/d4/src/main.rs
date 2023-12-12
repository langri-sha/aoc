use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    read_lines()
        .map(|line| {
            line.unwrap()
                .split(|c| c == ':' || c == '|')
                .skip(1)
                .map(|s| {
                    let re = Regex::new(r"\d+").unwrap();

                    re.captures_iter(s)
                        .map(|c| c[0].parse::<usize>().unwrap())
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>()
                .chunks_exact(2)
                .map(|slice| match slice {
                    [a, b] => 2usize.pow(a.intersection(b).count().try_into().unwrap()) / 2,
                    _ => panic!("Invalid input"),
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    return reader.lines();
}
