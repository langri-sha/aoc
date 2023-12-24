use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Zip;
use std::path::Path;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    read_races().fold(1usize, |result, (time, distance)| {
        let mut wins = 0usize;

        for i in 1..time {
            if i * (time - i) > distance {
                wins += 1;
            }
        }

        result * wins
    })
}

fn part_two() -> usize {
    let [time, distance] = read_races()
        .collect::<Vec<_>>()
        .iter()
        .fold(
            [Vec::new(), Vec::new()],
            |[mut times, mut distances], (time, distance)| {
                times.push(time);
                distances.push(distance);

                [times, distances]
            },
        )
        .iter()
        .map(|v| {
            v.iter()
                .map(|v| v.to_string())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut wins = 0usize;

    for i in 1..time {
        if i * (time - i) > distance {
            wins += 1;
        }
    }

    wins
}

fn read_races() -> Zip<impl Iterator<Item = usize>, impl Iterator<Item = usize>> {
    let re = Regex::new(r"\d+").unwrap();

    let [times, distances] = read_lines()
        .map(|line| {
            re.find_iter(line.unwrap().as_str())
                .map(|cap| cap.as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    times.into_iter().zip(distances.into_iter())
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
