use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> isize {
    get_histories().fold(0isize, |acc, history| {
        let mut tails: Vec<_> = Vec::new();
        let mut next: Vec<_> = history.clone();

        while !next.iter().all(|&v| v == 0) {
            tails.push(next.last().unwrap().to_owned());

            next = next.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        }

        acc + tails.iter().sum::<isize>()
    })
}

fn part_two() -> isize {
    get_histories().fold(0isize, |acc, history| {
        let mut heads: Vec<_> = Vec::new();
        let mut next: Vec<_> = history.clone();

        while !next.iter().all(|&v| v == 0) {
            heads.push(next.first().unwrap().to_owned());

            next = next.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        }

        acc + heads
            .iter()
            .rev()
            .fold(0isize, |acc, &v| v - acc)
    })
}

fn get_histories() -> impl Iterator<Item = Vec<isize>> {
    read_lines().map(|line| {
        line.unwrap()
            .split(" ")
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>()
    })
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
