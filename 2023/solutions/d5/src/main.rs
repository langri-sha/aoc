use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;
use std::path::Path;
use std::usize;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    let (seeds, mappings) = read_seeds_mappings();

    seeds
        .into_iter()
        .map(|seed| {
            mappings.iter().fold(seed, |seed, mapping| {
                mapping
                    .iter()
                    .find_map(|(range, destination)| {
                        if (seed >= range.start) && (seed <= range.end) {
                            Some(destination + (seed - range.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
}

fn read_seeds_mappings() -> (Vec<usize>, Vec<Vec<(Range<usize>, usize)>>) {
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

                last.push({
                    let [destination, source, length] = line
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()[..]
                    else {
                        panic!("Invalid mapping: {:?}", line);
                    };

                    (source..source + length, destination)
                });
            }

            (seeds, mappings)
        });

    let mappings = mappings.iter().map(|mapping| {
        let mut cloned = mapping.clone();

        cloned.sort_by(|(range_a, _), (range_b, _)| range_a.start.cmp(&range_b.start));
        cloned
    }).collect::<Vec<_>>();

    (seeds, mappings)
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
