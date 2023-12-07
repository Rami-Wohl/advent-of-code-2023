use std::collections::{BTreeMap, HashMap};
use std::fs::read_to_string;
use std::time::SystemTime;

struct Hand {
    cards: Vec<i32>,
    bid: i32,
}

fn get_type(cards: &str) -> i32 {
    let cards_vec: Vec<char> = cards.chars().collect();

    let mut cards_grouped: HashMap<char, Vec<char>> = HashMap::new();

    let mut no_of_jokers: usize = 0;

    for card in cards_vec {
        //comment this out for pt. 1
        if card == 'J' {
            no_of_jokers += 1;
            continue;
        }
        let card_group: &mut Vec<char> = cards_grouped.entry(card).or_insert(vec![]);
        card_group.push(card)
    }

    let card_type: i32 = match cards_grouped.len() {
        5 => 1,
        4 => 2,
        3 => {
            if cards_grouped
                .iter()
                .any(|(_, cards)| cards.len() + no_of_jokers == 3)
            {
                4
            } else {
                3
            }
        }
        2 => {
            if cards_grouped
                .iter()
                .any(|(_, cards)| cards.len() + no_of_jokers == 4)
            {
                6
            } else {
                5
            }
        }
        1 => 7,
        0 => 7,
        _ => 8,
    };

    card_type
}

fn main() {
    let start = SystemTime::now();

    let mut hands_per_type: BTreeMap<i32, Vec<Hand>> = BTreeMap::new();

    let lines: Vec<String> = read_lines("input.txt");

    let mut winnings: Vec<i32> = vec![];

    for line in lines {
        let (cards, bid) = line.split_once(" ").unwrap();

        let hand_type = get_type(cards);

        let hands_group: &mut Vec<Hand> = hands_per_type.entry(hand_type).or_insert(vec![]);

        hands_group.push(Hand {
            cards: cards
                .chars()
                .into_iter()
                .map(|card| {
                    card.to_string()
                        // ja dit is AIDS I know, ff bek houe pls
                        .replace("T", "10")
                        //"J" to "11" for pt. 1
                        .replace("J", "11")
                        .replace("Q", "12")
                        .replace("K", "13")
                        .replace("A", "14")
                        .trim()
                        .parse::<i32>()
                        .unwrap()
                })
                .collect(),
            bid: bid.parse::<i32>().unwrap(),
        })
    }

    let mut index = 0;

    for (_hand_type, mut type_group) in hands_per_type {
        type_group.sort_by(|a, b| {
            let mut index = 0;
            'inner: for (i, card) in a.cards.iter().enumerate() {
                if card != &b.cards[i] {
                    break 'inner;
                }

                index += 1;
            }

            a.cards[index].cmp(&b.cards[index])
        });

        for hand in type_group {
            index += 1;
            winnings.push(index * hand.bid);
        }
    }

    let sum: i32 = winnings.iter().sum();

    println!("sum is {}", sum);
    println!("done in {:?}", start.elapsed().unwrap())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
