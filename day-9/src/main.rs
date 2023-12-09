use std::fs::read_to_string;
use std::time::SystemTime;

fn diff_until_zero(sequence: Vec<i32>, extrapolations: &mut Vec<i32>) -> Vec<i32> {
    //w[1] - w[0] here for pt1
    let diffs: Vec<i32> = sequence.windows(2).map(|w| w[0] - w[1]).collect();

    println!("line: {:?}", sequence);

    //take last elem here instead of first for pt1
    let first_elem = diffs[0];

    let sum_of_diffs: i32 = diffs.iter().sum();

    if sum_of_diffs != 0 {
        diff_until_zero(diffs, extrapolations);
    }

    extrapolations.push(first_elem);

    extrapolations.to_vec()
}

fn find_next_number(line: &String) -> i32 {
    let line_vec: Vec<i32> = line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let extrapolations: &mut Vec<i32> = &mut Vec::new();
    //take last elem here for pt. 1
    extrapolations.push(line_vec[0]);

    let diffs_vec = diff_until_zero(line_vec, extrapolations);

    println!("diffs: {:?}", diffs_vec);

    let sum_for_line: i32 = diffs_vec.iter().sum();

    sum_for_line
}

fn main() {
    let start = SystemTime::now();

    let lines: Vec<String> = read_lines("input.txt");

    let sum_pt1: i32 = lines.iter().map(find_next_number).into_iter().sum();

    println!("sum is {:?}", sum_pt1);
    println!("done in {:?}", start.elapsed().unwrap())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
