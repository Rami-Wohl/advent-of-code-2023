use std::fs::read_to_string;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    let lines: Vec<String> = read_lines("input.txt");

    println!("done in {:?}", start.elapsed().unwrap())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
