use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let mut lines = read_map().fold(Vec::new(), |mut acc, line| {
        if line.iter().all(|c| c == &'.') {
            acc.push(line.clone());
        }

        acc.push(line);

        acc
    });

    let expanded: Vec<usize> = (0..lines.first().unwrap().len())
        .filter(|n| lines.iter().all(|line| line[*n] == '.'))
        .collect();

    expanded.iter().enumerate().for_each(|(i, n)| {
        lines.iter_mut().for_each(|line| {
            line.insert(i + n, '.');
        });
    });

    let mut galaxies = lines
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, line)| {
            line.iter().enumerate().for_each(|(x, &c)| {
                if c == '#' {
                    acc.push((x, y));
                }
            });

            acc
        });

    galaxies.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    galaxies
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, (x1, y1))| {
            acc + galaxies
                .iter()
                .skip(i + 1)
                .map(|(x2, y2)| x2.abs_diff(*x1) + y2.abs_diff(*y1))
                .sum::<usize>()
        })
}

fn part_two() -> usize {
    let lines: Vec<Vec<char>> = read_map().collect();

    let expanded_lines: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.iter().all(|c| c == &'.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let expanded_columns: Vec<usize> = (0..lines.first().unwrap().len())
        .filter(|n| lines.iter().all(|line| line[*n] == '.'))
        .collect();

    let mut galaxies = lines
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, line)| {
            line.iter().enumerate().for_each(|(x, &c)| {
                if c == '#' {
                    acc.push((x, y));
                }
            });

            acc
        });

    galaxies.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    galaxies
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, (x1, y1))| {
            acc + galaxies
                .iter()
                .skip(i + 1)
                .map(|(x2, y2)| {
                    let x1offset =
                        expanded_columns.iter().filter(|&e| e.lt(&x1)).count() * (1_000_000 - 1);
                    let x2offset =
                        expanded_columns.iter().filter(|&e| e.lt(&x2)).count() * (1_000_000 - 1);
                    let y1offset =
                        expanded_lines.iter().filter(|&e| e.lt(&y1)).count() * (1_000_000 - 1);
                    let y2offset =
                        expanded_lines.iter().filter(|&e| e.lt(&y2)).count() * (1_000_000 - 1);

                    (x2 + x2offset).abs_diff(*x1 + x1offset)
                        + (y2 + y2offset).abs_diff(*y1 + y1offset)
                })
                .sum::<usize>()
        })
}

fn read_map() -> impl Iterator<Item = Vec<char>> {
    read_lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<_>>())
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
