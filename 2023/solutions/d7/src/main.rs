use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let get_card_position = |card: &char| {
        ("AKQJT98765432")
            .chars()
            .rev()
            .position(|c| c == *card)
            .unwrap()
    };

    let get_hand_type = |hand: &str| {
        let occurences = hand
            .chars()
            .fold(HashSet::new(), |mut cards, card| {
                cards.insert(card);

                cards
            })
            .iter()
            .map(|card| *card)
            .map(|card| {
                hand.rmatches(card.to_string().as_str())
                    .collect::<Vec<_>>()
                    .len()
            })
            .collect::<Vec<_>>();
        let max = occurences.iter().max().unwrap();

        get_hand(occurences.len(), *max)
    };

    solve(get_hand_type, get_card_position)
}

fn part_two() -> usize {
    let get_card_position = |card: &char| {
        ("AKQT98765432J")
            .chars()
            .rev()
            .position(|c| c == *card)
            .unwrap()
    };

    let get_hand_type = |hand: &str| {
        let mut jokers = 0usize;
        let occurences = hand
            .chars()
            .fold(HashSet::new(), |mut cards, card| {
                if card == 'J' {
                    jokers += 1;
                } else {
                    cards.insert(card);
                }

                cards
            })
            .iter()
            .map(|card| *card)
            .map(|card| {
                hand.rmatches(card.to_string().as_str())
                    .collect::<Vec<_>>()
                    .len()
            })
            .collect::<Vec<_>>();
        let max = occurences.iter().max().unwrap_or(&0) + jokers;

        get_hand(occurences.len(), max)
    };

    solve(get_hand_type, get_card_position)
}

fn solve<H, C>(get_hand_type: H, get_card_position: C) -> usize
where
    H: Fn(&str) -> Hand,
    C: Fn(&char) -> usize,
{
    let mut totals = read_hands()
        .map(|(hand, bid)| (hand.clone(), get_hand_type(&hand), bid))
        .collect::<Vec<_>>();

    totals.sort_by(|(hand1, _, _), (hand2, _, _)| {
        match are_same_hand_types(hand1, hand2, &get_hand_type) {
            Ordering::Equal => hand1
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

                    result
                })
                .unwrap(),
            compare => compare,
        }
    });

    totals
        .iter()
        .enumerate()
        .fold(0usize, |acc, (index, (_, _, bid))| acc + (index + 1) * bid)
}

fn are_same_hand_types<H>(hand1: &str, hand2: &str, get_hand_type: H) -> Ordering
where
    H: Fn(&str) -> Hand,
{
    let [hand1, hand2] = [hand1, hand2]
        .into_iter()
        .map(get_hand_type)
        .map(|hand| match hand {
            Hand::HighCard => 0,
            Hand::OnePair => 1,
            Hand::TwoPair => 2,
            Hand::ThreeOfAKind => 3,
            Hand::FullHouse => 4,
            Hand::FourOfAKind => 5,
            Hand::FiveOfAKind => 6,
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    hand1.cmp(&hand2)
}

enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand(distinct: usize, max: usize) -> Hand {
    match distinct {
        5 => Hand::HighCard,
        4 => Hand::OnePair,
        3 if max == 3 => Hand::ThreeOfAKind,
        3 => Hand::TwoPair,
        2 if max == 3 => Hand::FullHouse,
        2 => Hand::FourOfAKind,
        _ => Hand::FiveOfAKind,
    }
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
