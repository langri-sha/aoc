use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> isize {
    read_lines()
        .map(|line| {
            line.unwrap()
                .split(" ")
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold(0isize, |acc, history| {
            let mut tails: Vec<_> = Vec::new();
            let mut next: Vec<_> = history.clone();

            while !next.iter().all(|&v| v == 0) {
                tails.push(next.last().unwrap().to_owned());

                next = next
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<_>>();
            }

            acc + tails.iter().sum::<isize>()
        })
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
