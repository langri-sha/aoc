use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two())
}

fn part_one() -> usize {
    let (_, pipes, _) = parse_map();

    pipes.len() / 2
}

fn part_two() -> usize {
    let (tiles, pipes, width) = parse_map();

    tiles.iter().enumerate().fold(0usize, |acc, (y, row)| {
        let mut inside = false;

        acc + row
            .iter()
            .enumerate()
            .filter(|(x, tile)| {
                let is_pipe = pipes.contains(&(*x, y));
                inside ^= is_pipe && matches!(*tile, '|' | 'F' | '7');
                inside && !is_pipe
            })
            .count()
    })
}

fn parse_map() -> (Vec<Vec<char>>, HashSet<(usize, usize)>, usize) {
    let mut tiles: Vec<Vec<char>> = read_lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let width = tiles[0].len();
    let height = tiles.len();

    let y = tiles.iter().position(|row| row.contains(&'S')).unwrap();
    let x = tiles[y].iter().position(|tile| tile == &'S').unwrap();

    let sx = x;
    let sy = y;

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = vec![(x, y)];

    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();

        if seen.contains(&(x, y)) {
            continue;
        } else {
            seen.insert((x, y));
        }

        let current = tiles[y][x];

        if y + 1 < height
            && matches!(current, 'S' | '|' | '7' | 'F')
            && matches!(tiles[y + 1][x], 'S' | '|' | 'L' | 'J')
        {
            queue.push((x, y + 1));
        }

        if y > 0
            && matches!(current, 'S' | '|' | 'L' | 'J')
            && matches!(tiles[y - 1][x], 'S' | '|' | '7' | 'F')
        {
            queue.push((x, y - 1));
        }

        if x + 1 < width
            && matches!(current, 'S' | '-' | 'L' | 'F')
            && matches!(tiles[y][x + 1], 'S' | '-' | 'J' | '7')
        {
            queue.push((x + 1, y));
        }

        if x > 0
            && matches!(current, 'S' | '-' | 'J' | '7')
            && matches!(tiles[y][x - 1], 'S' | '-' | 'L' | 'F')
        {
            queue.push((x - 1, y));
        }
    }

    tiles[sy][sx] = 'F';

    (tiles, seen, width)
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
