use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("scores.txt");
    //let re_index: Regex = Regex::new(r"(\d{1,3})").unwrap();
    let re_blue: Regex = Regex::new(r"(?<blue>\d{1,3})\ blue").unwrap();
    let re_green: Regex = Regex::new(r"(?<green>\d{1,3})\ green").unwrap();
    let re_red: Regex = Regex::new(r"(?<red>\d{1,3})\ red").unwrap();
    // const MAX_RED: i32 = 12;
    // const MAX_GREEN: i32 = 13;
    // const MAX_BLUE: i32 = 14;

    let mut powers: Vec<i32> = Vec::new();

    for line in lines {
        // let mat = re_index.find(&line).unwrap();
        //let id: i32 = mat.as_str().parse().unwrap();

        let blues: Vec<i32> = re_blue
            .captures_iter(&line)
            .map(|caps: regex::Captures<'_>| {
                caps.name("blue")
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse()
                    .unwrap()
            })
            .collect();

        // let blues_exceeded = blues.iter().any(|&f| f > MAX_BLUE);

        let greens: Vec<i32> = re_green
            .captures_iter(&line)
            .map(|caps: regex::Captures<'_>| {
                caps.name("green")
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse()
                    .unwrap()
            })
            .collect();

        // let greens_exceeded = greens.iter().any(|&f| f > MAX_GREEN);

        let reds: Vec<i32> = re_red
            .captures_iter(&line)
            .map(|caps: regex::Captures<'_>| {
                caps.name("red")
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse()
                    .unwrap()
            })
            .collect();

        // let reds_exceeded = reds.iter().any(|&f| f > MAX_RED);

        // if blues_exceeded || greens_exceeded || reds_exceeded {
        //     continue;
        // }

        let power: i32 =
            blues.iter().max().unwrap() * greens.iter().max().unwrap() * reds.iter().max().unwrap();

        powers.push(power);
    }

    let sum: i32 = powers.iter().sum();

    println!("sum is: {}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
