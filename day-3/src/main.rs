use regex::Regex;
use std::cmp;
use std::fs::read_to_string;

struct Line {
    line: String,
    line_num: usize,
}

struct Code {
    line_num: usize,
    line_idx: usize,
    len: usize,
    num: String,
}

struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let lines: Vec<String> = read_lines("codes.txt");
    let re: Regex = Regex::new(r"(\d{1,3})").unwrap();
    let mut lines_vec: Vec<Line> = Vec::new();
    let mut codes: Vec<Code> = Vec::new();
    let mut no_adjacent_symbols: Vec<i32> = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        lines_vec.push(Line {
            line_num,
            line: line.to_string(),
        });

        re.find_iter(line.as_str()).for_each(|mat| {
            codes.push(Code {
                line_num: line_num,
                line_idx: mat.start(),
                len: mat.len(),
                num: mat.as_str().to_string(),
            });
        });
    }

    codes.iter().for_each(|code| {
        let mut positions: Vec<Position> = Vec::new();
        let line_length = lines_vec[0].line.len();

        // println!(
        //     "num: {}, idx: {}, line: {}, len: {}",
        //     code.num, code.line_idx, code.line_num, code.len
        // );

        //upper
        if code.line_num > 0 {
            let low_boundary = if code.line_idx == 0 {
                0
            } else {
                code.line_idx - 1
            };
            let high_boundary = cmp::min(code.line_idx + code.len + 1, line_length);

            (low_boundary..high_boundary).for_each(|x| {
                positions.push(Position {
                    x: (x),
                    y: (code.line_num - 1),
                })
            })
        }

        //lower
        if code.line_num + 1 < lines_vec.len() {
            let low_boundary = if code.line_idx == 0 {
                0
            } else {
                code.line_idx - 1
            };
            let high_boundary = cmp::min(code.line_idx + code.len + 1, line_length - 1);

            (low_boundary..high_boundary).for_each(|x| {
                positions.push(Position {
                    x: (x),
                    y: (code.line_num + 1),
                })
            })
        }

        //left
        if code.line_idx > 0 {
            positions.push(Position {
                x: (code.line_idx - 1),
                y: (code.line_num),
            })
        }

        //right

        if code.line_idx + code.len < line_length - 1 {
            positions.push(Position {
                x: (code.line_idx + code.len),
                y: (code.line_num),
            })
        }
        // if code.num == "827" {
        //     println!(
        //         "lineidx + {}, linelen {}",
        //         code.line_idx + code.len,
        //         line_length
        //     );
        //     lines_vec
        //         .iter()
        //         .for_each(|lne| println!("numnum {}", lne.line_num));

        //     positions
        //         .iter()
        //         .for_each(|pos| println!("num: {} x: {} y: {}", code.num, pos.x, pos.y));
        // }

        if positions.iter().all(|pos| {
            let line = lines_vec
                .iter()
                .find(|line| line.line_num == pos.y)
                .unwrap();

            // println!(
            //     "num: {}, symbol: {}",
            //     code.num,
            //     line.line.chars().nth(pos.x).unwrap()
            //);

            line.line.chars().nth(pos.x).unwrap() == '.'
        }) {
            no_adjacent_symbols.push(code.num.parse::<i32>().unwrap())
        }
    });

    no_adjacent_symbols
        .iter()
        .for_each(|num| println!("num: {}", num));

    let sum: i32 = no_adjacent_symbols.iter().sum();

    let sum_all: i32 = codes
        .iter()
        .map(|code| code.num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    let diff = sum_all - sum;

    println!("sum of all numbers is: {}, diff is {}", sum_all, diff);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
