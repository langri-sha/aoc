use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

type Contents = (usize, usize, usize);

fn main() {
    println!("{}", part_one((12, 13, 14)));
}

fn part_one(limit: Contents) -> usize {
    let result =
        read_cubes().enumerate().filter_map(|(game, seen)| {
            if seen.0 <= limit.0 && seen.1 <= limit.1 && seen.2 <= limit.2 {
                return Some(game + 1)
            } else {
                return None
            }
        })
        .sum::<usize>();

    return result;
}

fn read_cubes() -> impl Iterator<Item = Contents> {
    read_lines()
        .map(|line| {
            let mut seen = (0, 0, 0);

            line.unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(|c| c == ',' || c == ';')
                .for_each(|item| {
                    let [value, color, ..] = match item.trim().split(" ").collect::<Vec<&str>>()[..]
                    {
                        [value, color, ..] => [value, color],
                        _ => panic!("Invalid item: {}", item),
                    };

                    match color {
                        "red" => seen.0 = seen.0.max(value.parse::<usize>().unwrap()),
                        "green" => seen.1 = seen.1.max(value.parse::<usize>().unwrap()),
                        "blue" => seen.2 = seen.2.max(value.parse::<usize>().unwrap()),
                        _ => panic!("Invalid color: {}", color),
                    };
                });

            return seen
        })
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    return reader.lines();
}
