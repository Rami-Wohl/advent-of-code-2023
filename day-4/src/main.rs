use std::cmp;
use std::fs::read_to_string;

struct Card {
    index: usize,
    winning: Vec<i32>,
    actual: Vec<i32>,
}

fn main() {
    let lines: Vec<String> = read_lines("cards.txt");
    let mut cards: Vec<Card> = vec![];
    let mut scores: Vec<usize> = vec![];

    for (idx, line) in lines.iter().enumerate() {
        let sequences: std::str::Split<'_, &str> = line.split(" | ").into_iter();

        let mut winning: Vec<i32> = vec![];
        let mut actual: Vec<i32> = vec![];

        for (idx, seq) in sequences.enumerate() {
            if idx == 0 {
                let first_part: Vec<&str> =
                    seq.trim().split(": ").into_iter().collect::<Vec<&str>>();

                winning = first_part[1]
                    .trim()
                    .split_ascii_whitespace()
                    .into_iter()
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
            } else {
                actual = seq
                    .split_ascii_whitespace()
                    .into_iter()
                    .map(|n: &str| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
            }
        }

        cards.push(Card {
            index: idx,
            winning: winning,
            actual: actual,
        })
    }

    let mut card_counts: Vec<usize> = vec![1; cards.len()];

    cards.iter().for_each(|card: &Card| {
        let mut counter: usize = 0;

        for num in &card.actual {
            if card.winning.contains(&num) {
                counter += 1;
            }
        }

        let high_boundary: usize = cmp::min(lines.len(), card.index + counter + 1);

        (card.index + 1..high_boundary)
            .into_iter()
            .for_each(|n: usize| {
                card_counts[n] += card_counts[card.index];
            });

        scores.push(counter);
    });

    let sum: usize = card_counts.iter().sum();

    println!("total amount of cards is: {}", sum)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
