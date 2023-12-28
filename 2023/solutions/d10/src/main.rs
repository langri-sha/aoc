use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    let tiles: Vec<Vec<char>> = read_lines().map(|line| line.unwrap().chars().collect()).collect();
    let width = tiles[0].len();
    let height = tiles.len();

    let y = tiles.iter().position(|row| row.contains(&'S')).unwrap();
    let x = tiles[y].iter().position(|tile| tile == &'S').unwrap();

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

        if y + 1 < height && matches!(current, 'S' | '|' | '7' | 'F') && matches!(tiles[y + 1][x], 'S' | '|' | 'L' | 'J') {
            queue.push((x, y + 1));
        }

        if y > 0 && matches!(current, 'S' | '|' | 'L' | 'J') && matches!(tiles[y - 1][x], 'S' | '|' | '7' | 'F') {
            queue.push((x, y - 1));
        }

        if x + 1 < width && matches!(current, 'S' | '-' | 'L' | 'F') && matches!(tiles[y][x + 1], 'S' | '-' | 'J' | '7') {
            queue.push((x + 1, y));
        }

        if x > 0 && matches!(current, 'S' | '-' | 'J' | '7')  && matches!(tiles[y][x - 1], 'S' | '-' | 'L' | 'F') {
            queue.push((x - 1, y));
        }
    }

    print_map(&tiles, &seen);

    return (seen.len()) / 2;
}

fn print_map(tiles: &Vec<Vec<char>>, seen: &HashSet<(usize, usize)>) {
    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if seen.contains(&(x, y)) {
                print!("*");
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
