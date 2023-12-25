use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    let (instructions, network) = parse_input();

    let mut steps = 0usize;
    let mut next = "AAA".to_owned();

    for char in instructions.chars().cycle() {
        if next == "ZZZ" {
            break;
        }

        steps += 1;

        next = get_next(&next, char, &network);
    }

    steps
}

fn get_next(current: &str, instruction: char, network: &HashMap<String, [String; 2]>) -> String {
    match instruction {
        'L' => network.get(current).unwrap()[0].to_owned(),
        'R' => network.get(current).unwrap()[1].to_owned(),
        _ => panic!("Unsupported instruction"),
    }
}

fn parse_input() -> (String, HashMap<String, [String; 2]>) {
    let mut lines = read_lines();

    let instructions = lines.next().unwrap().unwrap();
    let network = lines.fold(HashMap::new(), |mut network, line| {
        let re = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();

        re.captures(&line.unwrap()).map(|cap| {
            network
                .entry(cap.get(1).unwrap().as_str().to_owned())
                .or_insert([
                    cap.get(2).unwrap().as_str().to_owned(),
                    cap.get(3).unwrap().as_str().to_owned(),
                ]);
        });

        network
    });

    (instructions, network)
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
