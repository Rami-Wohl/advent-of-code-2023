use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::SystemTime;

struct Path {
    left: String,
    right: String,
}

enum Step {
    L,
    R,
}

fn main() {
    let start = SystemTime::now();

    let lines: Vec<String> = read_lines("input.txt");

    let steps = &lines[0]
        .chars()
        .map(|step| match step {
            'L' => Step::L,
            'R' => Step::R,
            _ => panic!("poop"),
        })
        .collect::<Vec<Step>>();

    let mut paths_map: HashMap<String, Path> = HashMap::new();

    (2..=lines.len() - 1).for_each(|n| {
        let (start_path_as_str, dest_paths) = lines[n].split_once("=").unwrap();
        let start_path: String = start_path_as_str.trim().to_string();

        let (left_str, right_str) = dest_paths.split_once(", ").unwrap();

        let left = left_str.trim().replace("(", "").to_string();
        let right = right_str.trim().replace(")", "").to_string();

        println!("row: {}, left: {}, right: {}", n + 1, left, right);

        paths_map.insert(start_path, Path { left, right });
    });

    let mut step_no = 0;
    let mut arrived: bool = false;

    paths_map
        .iter()
        .for_each(|(k, v)| println!("k: {}, vl: {}, vr: {}", k, v.left, v.right));

    let mut current_path = paths_map
        .iter()
        .find(|(k, _p)| k == &&String::from("AAA"))
        .unwrap();

    while !arrived {
        for step in steps {
            step_no += 1;

            let path_options: &Path = current_path.1;

            current_path = match step {
                Step::L => paths_map
                    .iter()
                    .find(|(k, _p)| **k == String::from(&path_options.left))
                    .unwrap(),
                Step::R => paths_map
                    .iter()
                    .find(|(k, _p)| **k == String::from(&path_options.right))
                    .unwrap(),
            };

            if *current_path.0 == String::from("ZZZ") || step_no > 100000 {
                arrived = true;
            }
        }
    }

    println!("took {} steps to get to ZZZ", step_no);
    println!("done in {:?}", start.elapsed().unwrap())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
