use std::fs::read_to_string;
use std::time::SystemTime;

fn main() {
    let start: SystemTime = SystemTime::now();

    let lines: Vec<String> = read_lines("input.txt");

    let times: Vec<i32> = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|str: &str| str.parse::<i32>().unwrap())
        .collect();

    let distances: Vec<i32> = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|str: &str| str.parse::<i32>().unwrap())
        .collect();

    println!("times: {:?}", times);
    println!("distances: {:?}", distances);

    let mut ways_per_race: Vec<i32> = vec![];

    (0..times.len()).for_each(|race_number: usize| {
        let mut lower: i32 = 0;
        let mut higher: i32 = 0;
        let time: i32 = times[race_number];
        let distance: i32 = distances[race_number];

        let range: std::ops::Range<i32> = 0..time;

        for n in range {
            if ((n * time) - (n * n)) > distance && lower == 0 {
                lower = n;
            }
            if ((n * time) - (n * n)) <= distance && lower != 0 {
                higher = n;
                break;
            }
        }

        ways_per_race.push(higher - lower);
    });

    let product: i32 = ways_per_race.iter().product();

    println!("product pt 1 is {}", product);

    let time_pt2: i64 = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .into_iter()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let distance_pt2: i64 = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .into_iter()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let mut lower: i64 = 0;
    let mut higher: i64 = 0;

    let range: std::ops::Range<i64> = 0..time_pt2;

    for n in range {
        if lower == 0 && ((n * time_pt2) - (n * n)) > distance_pt2 {
            lower = n;
        }
        if lower != 0 && ((n * time_pt2) - (n * n)) <= distance_pt2 {
            higher = n;
            break;
        }
    }

    println!("no_of_ways pt 2 is {}", higher - lower);
    println!("done in {:?}", start.elapsed().unwrap())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
