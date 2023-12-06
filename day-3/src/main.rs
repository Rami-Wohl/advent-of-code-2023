use regex::Regex;
use std::cmp;
use std::collections::HashMap;
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

#[derive(Clone)]
struct StarPosition {
    num: i32,
    pos: String,
}

fn main() {
    let lines: Vec<String> = read_lines("codes.txt");
    let re: Regex = Regex::new(r"(\d{1,3})").unwrap();
    let mut lines_vec: Vec<Line> = Vec::new();
    let mut codes: Vec<Code> = Vec::new();
    let mut no_adjacent_symbols: Vec<i32> = Vec::new();
    let mut star_adjacent_nums: Vec<StarPosition> = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        lines_vec.push(Line {
            line_num,
            line: line.to_string(),
        });

        re.find_iter(line.as_str())
            .for_each(|mat: regex::Match<'_>| {
                codes.push(Code {
                    line_num: line_num,
                    line_idx: mat.start(),
                    len: mat.len(),
                    num: mat.as_str().to_string(),
                });
            });
    }

    codes.iter().for_each(|code: &Code| {
        let mut positions: Vec<Position> = Vec::new();
        let line_length: usize = lines_vec[0].line.len();

        //upper
        if code.line_num > 0 {
            let low_boundary: usize = if code.line_idx == 0 {
                0
            } else {
                code.line_idx - 1
            };
            let high_boundary: usize = cmp::min(code.line_idx + code.len + 1, line_length);

            (low_boundary..high_boundary).for_each(|x| {
                positions.push(Position {
                    x: (x),
                    y: (code.line_num - 1),
                })
            })
        }

        //lower
        if code.line_num + 1 < lines_vec.len() {
            let low_boundary: usize = if code.line_idx == 0 {
                0
            } else {
                code.line_idx - 1
            };
            let high_boundary: usize = cmp::min(code.line_idx + code.len + 1, line_length - 1);

            (low_boundary..high_boundary).for_each(|x: usize| {
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

        if positions.iter().all(|pos: &Position| {
            let line: &Line = lines_vec
                .iter()
                .find(|line: &&Line| line.line_num == pos.y)
                .unwrap();

            line.line.chars().nth(pos.x).unwrap() == '.'
        }) {
            no_adjacent_symbols.push(code.num.parse::<i32>().unwrap())
        }

        for pos in positions {
            let line: &Line = lines_vec
                .iter()
                .find(|line: &&Line| line.line_num == pos.y)
                .unwrap();

            if line.line.chars().nth(pos.x).unwrap() == '*' {
                star_adjacent_nums.push(StarPosition {
                    num: code.num.parse::<i32>().unwrap(),
                    pos: format!("{}-{}", pos.x, pos.y),
                })
            }
        }
    });

    let sum: i32 = no_adjacent_symbols.iter().sum();

    let sum_all: i32 = codes
        .iter()
        .map(|code: &Code| code.num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    let diff: i32 = sum_all - sum;

    println!("sum of all numbers is: {}, diff is {}", sum_all, diff);

    let mut groups: HashMap<String, Vec<i32>> = HashMap::new();

    star_adjacent_nums
        .into_iter()
        .for_each(|star_pos: StarPosition| {
            let group: &mut Vec<i32> = groups.entry(star_pos.pos).or_insert(vec![]);
            group.push(star_pos.num);
        });

    let mut gear_ratios: Vec<i32> = Vec::new();

    for (_id, group) in groups {
        if group.len() == 2 {
            gear_ratios.push(group[0] * group[1])
        }
    }

    let sum_gear_ratios: i32 = gear_ratios.iter().sum();

    println!("sum of all gear ratios is: {}", sum_gear_ratios);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
