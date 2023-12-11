use std::fs::read_to_string;
use std::time::SystemTime;
use std::usize;

#[derive(Clone, PartialEq)]
enum Move {
    UP,
    DOWN,
    RIGHT,
    LEFT,
    NONE,
}

#[derive(Clone)]
struct Point {
    last_move: Move,
    x: usize,
    y: usize,
}

fn get_next_steps(lines: &Vec<String>, current: &Point) -> Vec<Point> {
    let current_point = lines[current.y].chars().nth(current.x).unwrap();

    let mut next_steps: Vec<Point> = vec![];

    match (&current.last_move, current_point) {
        (Move::NONE, 'S') => {
            if current.y > 0 {
                next_steps.push(Point {
                    last_move: Move::UP,
                    x: current.x,
                    y: current.y - 1,
                });
            }

            if current.x < lines.first().unwrap().len() + 1 {
                next_steps.push(Point {
                    last_move: Move::RIGHT,
                    x: current.x + 1,
                    y: current.y,
                })
            };
            if current.y < lines.len() + 1 {
                next_steps.push(Point {
                    last_move: Move::DOWN,
                    x: current.x,
                    y: current.y + 1,
                })
            }

            if current.x > 0 {
                next_steps.push(Point {
                    last_move: Move::LEFT,
                    x: current.x - 1,
                    y: current.y,
                })
            }
        }
        (Move::UP, '|') => {
            if current.y > 0 {
                next_steps.push(Point {
                    last_move: Move::UP,
                    x: current.x,
                    y: current.y - 1,
                })
            }
        }
        (Move::UP, 'F') => {
            if current.x < lines.first().unwrap().len() + 1 {
                next_steps.push(Point {
                    last_move: Move::RIGHT,
                    x: current.x + 1,
                    y: current.y,
                })
            }
        }
        (Move::UP, '7') => {
            if current.x > 0 {
                next_steps.push(Point {
                    last_move: Move::LEFT,
                    x: current.x - 1,
                    y: current.y,
                })
            }
        }
        (Move::DOWN, '|') => {
            if current.y < lines.len() + 1 {
                next_steps.push(Point {
                    last_move: Move::DOWN,
                    x: current.x,
                    y: current.y + 1,
                })
            }
        }
        (Move::DOWN, 'J') => {
            if current.x > 0 {
                next_steps.push(Point {
                    last_move: Move::LEFT,
                    x: current.x - 1,
                    y: current.y,
                })
            }
        }
        (Move::DOWN, 'L') => {
            if current.x < lines.first().unwrap().len() + 1 {
                next_steps.push(Point {
                    last_move: Move::RIGHT,
                    x: current.x + 1,
                    y: current.y,
                })
            }
        }
        (Move::RIGHT, '-') => {
            if current.x < lines.first().unwrap().len() + 1 {
                next_steps.push(Point {
                    last_move: Move::RIGHT,
                    x: current.x + 1,
                    y: current.y,
                })
            }
        }
        (Move::RIGHT, 'J') => {
            if current.y > 0 {
                next_steps.push(Point {
                    last_move: Move::UP,
                    x: current.x,
                    y: current.y - 1,
                })
            }
        }
        (Move::RIGHT, '7') => {
            if current.y < lines.len() + 1 {
                next_steps.push(Point {
                    last_move: Move::DOWN,
                    x: current.x,
                    y: current.y + 1,
                })
            }
        }
        (Move::LEFT, '-') => {
            if current.x > 0 {
                next_steps.push(Point {
                    last_move: Move::LEFT,
                    x: current.x - 1,
                    y: current.y,
                })
            }
        }
        (Move::LEFT, 'F') => {
            if current.y < lines.len() + 1 {
                next_steps.push(Point {
                    last_move: Move::DOWN,
                    x: current.x,
                    y: current.y + 1,
                })
            }
        }
        (Move::LEFT, 'L') => {
            if current.y > 0 {
                next_steps.push(Point {
                    last_move: Move::UP,
                    x: current.x,
                    y: current.y - 1,
                })
            }
        }
        _ => {}
    }

    next_steps
}

fn is_path_valid(lines: &Vec<String>, current: &Point) -> bool {
    let current_point = lines[current.y].chars().nth(current.x).unwrap();

    match current.last_move {
        Move::UP => vec!['|', 'F', '7', 'S'].to_vec().contains(&current_point),
        Move::DOWN => vec!['|', 'J', 'L', 'S'].to_vec().contains(&current_point),
        Move::LEFT => vec!['-', 'F', 'L', 'S'].to_vec().contains(&current_point),
        Move::RIGHT => vec!['-', 'J', '7', 'S'].to_vec().contains(&current_point),
        Move::NONE => true,
    }
}

fn step(lines: &Vec<String>, current: &Point, path: &mut Vec<Point>) -> bool {
    let valid_path = is_path_valid(&lines, &current);

    let reached_end: bool =
        path.len() > 1 && lines[current.y].chars().nth(current.x).unwrap() == 'S';

    if !valid_path {
        return false;
    }

    if lines[current.y].chars().nth(current.x).unwrap() == 'G' {
        return false;
    }

    if reached_end {
        path.push(current.clone());
        return true;
    }

    path.push(current.clone());

    let next_steps: Vec<Point> = get_next_steps(&lines, &current);

    for next in next_steps {
        if step(lines, &next, path) {
            return true;
        }
    }

    path.pop();

    false
}

fn main() {
    let start_time: SystemTime = SystemTime::now();

    let lines: Vec<String> = read_lines("test-input-2.txt");

    let mut start: Point = Point {
        last_move: Move::NONE,
        x: 0,
        y: 0,
    };

    for (i, line) in lines.iter().enumerate() {
        if line.chars().position(|c: char| c == 'S').is_some() {
            start.x = line.chars().position(|c: char| c == 'S').unwrap();
            start.y = i;
            break;
        }
    }

    // println!("start x: {}, start y: {}", start.x, start.y);

    let path: &mut Vec<Point> = &mut Vec::new();

    step(&lines, &start, path);

    let points: Vec<String> = vec![];

    println!("path is {} steps long", path.len() - 1);
    println!("max distance is {}", ((path.len() - 1) / 2));

    struct PathPoint {
        x: usize,
        y: usize,
    }

    let points_split_vec: Vec<PathPoint> = points
        .iter()
        .map(|p| {
            let x = p.split_once("-").unwrap().0.parse::<usize>().unwrap();
            let y = p.split_once("-").unwrap().1.parse::<usize>().unwrap();

            PathPoint { x: x, y: y }
        })
        .collect();

    for (i, line) in lines.iter().enumerate() {
        let line_path = line
            .chars()
            .into_iter()
            .enumerate()
            .map(|(j, _c)| {
                if points_split_vec.iter().any(|p| p.x == j && p.y == i) {
                    'O'
                } else if start.x == j && start.y == i {
                    'S'
                } else {
                    match path.iter().any(|p| p.x == j && p.y == i) {
                        true => '*',
                        false => '.',
                    }
                }
            })
            .collect::<String>();
        println!("{}", line_path)
    }

    println!(
        "out of {} points, {} belong to path, {} are inside of loop and {} are outside of loop",
        lines.first().unwrap().len() * lines.len(),
        path.len() - 1,
        points.len(),
        lines.first().unwrap().len() * lines.len() - (path.len() - 1 + points.len())
    );

    println!("done in {:?}", start_time.elapsed().unwrap())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
