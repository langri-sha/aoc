use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::usize;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    let (seeds, mappings) =
        read_lines().fold((Vec::new(), Vec::new()), |(seeds, mut mappings), line| {
            let line = line.unwrap();

            if line.is_empty() {
                return (seeds, mappings);
            }

            if line.starts_with("seeds") {
                return (
                    line.split_ascii_whitespace()
                        .skip(1)
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                    mappings,
                );
            }

            if line.chars().next().unwrap().is_ascii_alphabetic() {
                mappings.push(Vec::new());
            } else {
                let last = mappings.last_mut().unwrap();

                last.push(
                    line.split_ascii_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }

            (seeds, mappings)
        });

    find_lowest_location(seeds, mappings)
}

fn find_lowest_location(seeds: Vec<usize>, mappings: Vec<Vec<Vec<usize>>>) -> usize {
    seeds.into_iter().map(|seed| mappings.iter().fold(seed, |seed, mapping| {
        mapping.iter().find_map(|mappings| {
            let [destination, source, length] = mappings[..] else {
                panic!("Invalid mapping: {:?}", mappings);
            };

            if (seed >= source) && (seed <= (source + length)) {
                Some(destination + (seed - source))
            } else {
                None
            }
        }).unwrap_or(seed)
    })).min().unwrap()
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
