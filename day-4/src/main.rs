use std::fs::read_to_string;

struct Card {
    index: usize,
    winning: Vec<i32>,
    actual: Vec<i32>,
}

fn main() {
    let lines: Vec<String> = read_lines("cards.txt");
    let mut cards: Vec<Card> = vec![];
    let mut scores: Vec<i32> = vec![];

    for (idx, line) in lines.iter().enumerate() {
        let sequences = line.split(" | ").into_iter();

        let mut winning: Vec<i32> = vec![];
        let mut actual: Vec<i32> = vec![];

        for (idx, seq) in sequences.enumerate() {
            if idx == 0 {
                let first_part = seq.trim().split(": ").into_iter().collect::<Vec<&str>>();

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
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
            }
        }
        // println!("{} win: {:?} actual: {:?}", idx, winning, actual);

        cards.push(Card {
            index: (idx + 1),
            winning: winning,
            actual: actual,
        })
    }

    for card in cards {
        let mut counter: i32 = 0;

        for num in card.actual {
            if card.winning.contains(&num) {
                if counter == 0 {
                    counter += 1;
                } else {
                    counter = counter * 2;
                }
            }
        }

        scores.push(counter);
    }

    let sum: i32 = scores.iter().sum();

    println!("total score is: {}", sum)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
