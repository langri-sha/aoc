use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    let mut totals = read_hands()
        .map(|(hand, bid)| (hand.clone(), get_hand_value(&hand), bid))
        .collect::<Vec<_>>();

    totals.sort_by(|(hand1, value1, _), (hand2, value2, _)| {
        let compare = are_same_hand_type(hand1, hand2);

        if compare == Ordering::Equal {
            println!("EQUAL {} {}", hand1, hand2);
            hand1
                .chars()
                .zip(hand2.chars())
                .find_map(|(card1, card2)| {
                    let result = Some((card1, card2))
                        .map(|(card1, card2)| {
                            (get_card_position(&card1), get_card_position(&card2))
                        })
                        .map(|(card1, card2)| card1.cmp(&card2))
                        .map(|compare| match compare {
                            Ordering::Equal => None,
                            _ => Some(compare),
                        })
                        .unwrap();

                    println!("{:?} {} {}", result, card1, card2);

                    result
                })
                .unwrap()
        } else {
            compare
        }
    });

    totals
        .iter()
        .enumerate()
        .fold(0usize, |acc, (index, (_, _, bid))| {
            println!("{} {}", index + 1, bid);
            acc + (index + 1) * bid
        })
}

fn are_same_hand_type(hand1: &str, hand2: &str) -> Ordering {
    println!("SIZE_IN  {} {}", hand1, hand2);

    let [hand1, hand2] = [hand1, hand2]
        .into_iter()
        .map(get_hand_value)
        .map(|value| {
            if value == 0 {
                1
            } else {
                (value as f64).log10().floor() as usize + 1
            }
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    println!("SIZE_OUT {} {}", hand1, hand2);

    hand1.cmp(&hand2)
}

fn get_hand_value(hand: &str) -> usize {
    hand.chars()
        .fold(HashSet::new(), |mut cards, card| {
            cards.insert(card);

            cards
        })
        .iter()
        .fold(0usize, |sum, card| {
            let occurrences = hand
                .rmatches(card.to_string().as_str())
                .collect::<Vec<_>>()
                .len();

            sum + 10usize.pow(occurrences as u32) + get_card_position(card)
        })
}

fn get_card_position(card: &char) -> usize {
    get_cards().iter().position(|c| c == card).unwrap()
}

fn get_cards() -> [char; 13] {
    ("AKQJT98765432")
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn read_hands() -> impl Iterator<Item = (String, usize)> {
    read_lines().map(|line| line.unwrap()).map(|line| {
        let [hand, bid] = line
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        (hand, bid.parse::<usize>().unwrap())
    })
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let file = File::open(Path::join(out_dir, "share/input.txt")).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}
