use regex::{Captures, Regex};
use std::fs::read_to_string;
fn main() {
    let lines: Vec<String> = read_lines("codes.txt");
    let re_num: Regex = Regex::new(r"(\d{1})").unwrap();
    let re_str: Regex = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_str_rev: Regex = Regex::new("(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        let line_parsed = re_str.replace(&line, |cap: &Captures| {
            match &cap[0] {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => panic!("We should never get here"),
            }
            .to_string()
        });

        let mut numbers_string: String = "".to_owned();
        if let Some(mat) = re_num.find(&line_parsed) {
            let first_digit: &str = mat.as_str();
            numbers_string.push_str(&first_digit);
        }

        let reverse_line: String = line.chars().rev().collect::<String>();

        let line_parsed_2 = re_str_rev.replace(&reverse_line, |cap: &Captures| {
            match &cap[0] {
                "eno" => "1",
                "owt" => "2",
                "eerht" => "3",
                "ruof" => "4",
                "evif" => "5",
                "xis" => "6",
                "neves" => "7",
                "thgie" => "8",
                "enin" => "9",
                _ => panic!("We should never get here"),
            }
            .to_string()
        });

        if let Some(mat) = re_num.find(&line_parsed_2) {
            let last_digit: &str = mat.as_str();
            numbers_string.push_str(&last_digit);
        }

        let number: i32 = numbers_string.parse().unwrap();

        numbers.push(number);
    }

    let sum: i32 = numbers.iter().sum();

    println!("sum is {}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
