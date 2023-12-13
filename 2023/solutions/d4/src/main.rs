use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    parse_cards()
        .map(|c| 2usize.pow(c.try_into().unwrap()) / 2)
        .sum::<usize>()
}

fn part_two() -> usize {
    parse_cards()
        .enumerate()
        .fold(HashMap::new(), |mut wins, (i, c)| {
            *wins.entry(i).or_insert(0usize) += 1;

            (i..i + c).for_each(|j| {
                *wins.entry(j + 1).or_insert(0) += *wins.entry(i).or_insert(1);
            });

            wins
        })
        .values()
        .sum::<usize>()
}

fn parse_cards() -> impl Iterator<Item = usize> {
    read_lines().map(|line| {
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
                [a, b] => a.intersection(b).count(),
                _ => panic!("Invalid input"),
            })
            .sum::<_>()
    })
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    return reader.lines();
}
