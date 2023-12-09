use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::SystemTime;
struct Mapping {
    destination: i64,
    range: i64,
    diff: i64,
}

fn main() {
    let start: SystemTime = SystemTime::now();
    let mut lowest: i64 = 0;
    let lines: Vec<String> = read_lines("input.txt");
    let line_1: &String = &lines[0];
    let (_, seeds_string) = line_1.split_once(":").unwrap();
    let seed_numbers: Vec<i64> = seeds_string
        .split_ascii_whitespace()
        .map(|str: &str| str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let re: Regex = Regex::new(r"(?<map>\w+-\w+-\w+)\ map:").unwrap();

    let mut mappings: HashMap<String, Vec<Mapping>> = HashMap::new();
    let mut current: String = String::new();

    for (_idx, line) in lines.iter().enumerate() {
        let caps: Option<regex::Captures<'_>> = re.captures(&line);

        if caps.is_some() {
            let mapping_title: String = caps
                .unwrap()
                .name("map")
                .unwrap()
                .as_str()
                .to_string()
                .parse()
                .unwrap();

            current = mapping_title;
            continue;
        }

        if line.trim() == "" {
            continue;
        }

        if current != String::new() {
            let mapping: Vec<i64> = line
                .split_ascii_whitespace()
                .into_iter()
                .map(|n: &str| n.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let destination: i64 = mapping[0];
            let source: i64 = mapping[1];
            let range: i64 = mapping[2];
            let diff: i64 = destination - source;

            let key: String = String::from(&current);

            let mapping_group: &mut Vec<Mapping> = mappings.entry(key).or_insert(vec![]);

            mapping_group.push(Mapping {
                //source: (source),
                range: (range),
                destination: (destination),
                diff: (diff),
            });
        }
    }

    let (_, seed_to_soil_map) = mappings
        .iter()
        .find(|(k, _m)| **k == String::from("seed-to-soil"))
        .unwrap();

    let (_, soil_to_fertilizer_map) = mappings
        .iter()
        .find(|(k, _m)| **k == String::from("soil-to-fertilizer"))
        .unwrap();

    let (_, fertilizer_to_water_map) = mappings
        .iter()
        .find(|(k, _m)| **k == String::from("fertilizer-to-water"))
        .unwrap();

    let (_, water_to_light_map) = mappings
        .iter()
        .find(|(k, _m)| **k == String::from("water-to-light"))
        .unwrap();

    let (_, light_to_temperature_map) = mappings
        .iter()
        .find(|(k, _m)| **k == String::from("light-to-temperature"))
        .unwrap();

    let (_, temperature_to_humidity_map) = mappings
        .iter()
        .find(|(k, _m)| **k == String::from("temperature-to-humidity"))
        .unwrap();

    let (_, humidity_to_location_map) = mappings
        .iter()
        .find(|(k, _m)| **k == String::from("humidity-to-location"))
        .unwrap();

    // //way too high

    let range: std::ops::Range<i64> = 0..100_000_000;

    'outer: for n in range {
        let mut current_number: i64 = n;
        let mut target: i64 = n;

        humidity_to_location_map
            .iter()
            .for_each(|mapping: &Mapping| {
                let low: i64 = mapping.destination;
                let high: i64 = mapping.destination + mapping.range;
                if (low..=high).contains(&current_number) {
                    target = current_number - mapping.diff;
                }
            });

        current_number = target;

        temperature_to_humidity_map
            .iter()
            .for_each(|mapping: &Mapping| {
                let low: i64 = mapping.destination;
                let high: i64 = mapping.destination + mapping.range;
                if (low..=high).contains(&current_number) {
                    target = current_number - mapping.diff;
                }
            });

        current_number = target;

        light_to_temperature_map
            .iter()
            .for_each(|mapping: &Mapping| {
                let low: i64 = mapping.destination;
                let high: i64 = mapping.destination + mapping.range;
                if (low..=high).contains(&current_number) {
                    target = current_number - mapping.diff;
                }
            });

        current_number = target;

        water_to_light_map.iter().for_each(|mapping: &Mapping| {
            let low: i64 = mapping.destination;
            let high: i64 = mapping.destination + mapping.range;
            if (low..=high).contains(&current_number) {
                target = current_number - mapping.diff;
            }
        });

        current_number = target;

        fertilizer_to_water_map
            .iter()
            .for_each(|mapping: &Mapping| {
                let low: i64 = mapping.destination;
                let high: i64 = mapping.destination + mapping.range;
                if (low..=high).contains(&current_number) {
                    target = current_number - mapping.diff;
                }
            });

        current_number = target;

        soil_to_fertilizer_map.iter().for_each(|mapping: &Mapping| {
            let low: i64 = mapping.destination;
            let high: i64 = mapping.destination + mapping.range;
            if (low..=high).contains(&current_number) {
                target = current_number - mapping.diff;
            }
        });

        current_number = target;

        seed_to_soil_map.iter().for_each(|mapping: &Mapping| {
            let low: i64 = mapping.destination;
            let high: i64 = mapping.destination + mapping.range;
            if (low..=high).contains(&current_number) {
                target = current_number - mapping.diff;
            }
        });

        if (1..=10).any(|idx: usize| {
            let id: i64 = seed_numbers[idx * 2 - 2];
            let range: i64 = seed_numbers[idx * 2 - 1];

            (id..id + range).contains(&target)
        }) {
            lowest = n;
            break 'outer;
        }
    }

    println!("{:?}", lowest);
    println!("done in {:?}", start.elapsed().unwrap());
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
