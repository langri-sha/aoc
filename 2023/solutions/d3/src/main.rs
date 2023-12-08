use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::usize;

fn main() {
    println!("Part one: {}", part_one());
}

fn part_one() -> usize {
    let re_nums = Regex::new(r"\d+").unwrap();
    let re_symbol = Regex::new(r"[^\.0-9]").unwrap();

    let grid = read_lines().map(|i| i.unwrap()).collect::<Vec<String>>();

    grid.iter()
        .enumerate()
        .fold(HashMap::new(), |mut hits, (row, line)| {
            for cap in re_symbol.captures_iter(line) {
                let col = cap.get(0).unwrap().start();

                [
                    (row - 1, col - 1),
                    (row - 1, col),
                    (row - 1, col + 1),
                    (row, col - 1),
                    (row, col + 1),
                    (row + 1, col - 1),
                    (row + 1, col),
                    (row + 1, col + 1),
                ]
                .iter()
                .filter(|(x, y)| x >= &0 && y >= &0 && x < &grid.len() && y < &line.len())
                .for_each(|(x, y)| {
                    if grid[*x].chars().nth(*y).unwrap().is_ascii_digit() {
                        hits.entry(*x).or_insert_with(HashSet::new).insert(*y);
                    };
                })
            }

            hits
        })
        .iter()
        .map(|(row, col)| {
            re_nums
                .captures_iter(grid[*row].as_str())
                .map(|c| c.get(0).unwrap())
                .fold(Vec::new(), |mut caps, c| {
                    for num in col.iter() {
                        if num >= &&c.start() && num <= &&c.end() {
                            caps.push(c.as_str());
                            break;
                        }
                    }

                    return caps;
                })
                .iter()
                .map(|c| c.parse::<usize>().unwrap())
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
