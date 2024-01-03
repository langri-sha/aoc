use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    let mut lines: Vec<Vec<char>> = read_lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<_>>())
        .fold(Vec::new(), |mut acc, line| {
            if line.iter().all(|c| c == &' ') {
                acc.push(line.clone());
            }

            acc.push(line);

            acc
        });

    (0..lines.first().unwrap().len()).for_each(|n| {
        if lines.iter().all(|line| line[n] == '.') {
            for line in lines.iter_mut() {
                line.insert(n, '.');
            }
        }
    });

    for line in lines.iter() {
        println!("{}", line.iter().collect::<String>());
    }

    10
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
