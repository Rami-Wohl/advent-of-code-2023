use num;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::SystemTime;
#[derive(Clone)]
struct Path {
    start: String,
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

    let mut all_paths: Vec<Path> = vec![];

    (2..=lines.len() - 1).for_each(|n| {
        let (start_path_as_str, dest_paths) = lines[n].split_once("=").unwrap();
        let start_path: String = start_path_as_str.trim().to_string();

        let (left_str, right_str) = dest_paths.split_once(", ").unwrap();

        let left = left_str.trim().replace("(", "").to_string();
        let right = right_str.trim().replace(")", "").to_string();

        all_paths.push(Path {
            start: start_path,
            left,
            right,
        });
    });

    let starting_paths: Vec<&Path> = all_paths
        .iter()
        .filter(|k| k.start.chars().nth(2).unwrap() == 'A')
        .collect();

    starting_paths
        .iter()
        .for_each(|p| println!("s: {}, vl: {}, vr: {}", p.start, p.left, p.right));

    let mut step_numbers_to_z: HashMap<&str, Vec<u128>> = HashMap::new();

    'outer: for path in starting_paths {
        let starting_path = path.start.as_str();

        let steps_for_starting_path: &mut Vec<u128> =
            step_numbers_to_z.entry(starting_path).or_insert(vec![]);
        let mut step_no: u128 = 0;
        let mut arrived = false;

        let mut curr = path;

        while !arrived {
            for step in steps {
                step_no += 1;

                let next_step = match step {
                    Step::L => all_paths
                        .iter()
                        .find(|p| p.start == String::from(&curr.left))
                        .unwrap(),
                    Step::R => all_paths
                        .iter()
                        .find(|p| p.start == String::from(&curr.right))
                        .unwrap(),
                };

                curr = next_step;

                if next_step.start.chars().nth(2).unwrap() == 'Z' {
                    steps_for_starting_path.push(step_no);
                    arrived = true;
                    continue 'outer;
                }
            }
        }
    }

    step_numbers_to_z
        .iter()
        .for_each(|(k, v)| println!("for start {} step numbers to z are {:?}", k, v));

    let steps: Vec<u128> = step_numbers_to_z
        .iter()
        .map(|(k, v)| v.iter().next().unwrap())
        .cloned()
        .collect();

    let lcm = steps
        .iter()
        .fold(steps[0], |acc, n| num::integer::lcm(acc, *n));

    println!("took {:?} steps to get to all Z's", lcm);
    println!("done in {:?}", start.elapsed().unwrap())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
