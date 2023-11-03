use regex::Regex;
use std::collections::HashSet;
use std::fs;

use crate::PROJECT_DIRECTORY;

struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
}

fn parse_sensor(sensor: &str) -> Sensor {
    let mut tokens = sensor.split(": ");
    let sensor_info = tokens.next().unwrap();
    let beacon_info = tokens.next().unwrap();

    let match_x = Regex::new(r".*x=(-?\d+),").unwrap();
    let match_y = Regex::new(r".*y=(-?\d+)$").unwrap();
    let sensor_x = str::parse::<i32>(&match_x.captures(sensor_info).unwrap()[1]).unwrap();
    let sensor_y = str::parse::<i32>(&match_y.captures(sensor_info).unwrap()[1]).unwrap();
    let beacon_x = str::parse::<i32>(&match_x.captures(beacon_info).unwrap()[1]).unwrap();
    let beacon_y = str::parse::<i32>(&match_y.captures(beacon_info).unwrap()[1]).unwrap();
    return Sensor {
        position: (sensor_x, sensor_y),
        beacon: (beacon_x, beacon_y),
    };
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    let mut out = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            break;
        }
        out.push(parse_sensor(line));
    }
    out
}

fn covers(row: i32, sensor: &Sensor) -> Option<(i32, i32)> {
    let sensor_to_beacon_x = (sensor.position.0 - sensor.beacon.0).abs();
    let sensor_to_beacon_y = (sensor.position.1 - sensor.beacon.1).abs();

    let total_distance = sensor_to_beacon_x + sensor_to_beacon_y;
    let sensor_to_row = (sensor.position.1 - row).abs();

    if sensor_to_row > total_distance {
        return None;
    }

    let lr = total_distance - sensor_to_row;
    return Some((sensor.position.0 - lr, sensor.position.0 + lr));
}

fn part1() {
    let input = fs::read_to_string(format!("{}/day15/input.txt", PROJECT_DIRECTORY)).unwrap();
    let sensors = parse_sensors(&input);

    let mut covered_points: HashSet<i32> = HashSet::new();
    for sensor in sensors {
        let range = covers(2000000, &sensor);

        if range.is_some() {
            for i in range.unwrap().0..range.unwrap().1 {
                covered_points.insert(i);
            }
        }
    }

    println!("Covered: {}", covered_points.len());
}

fn part2() {
    let input = fs::read_to_string(format!("{}/day15/input.txt", PROJECT_DIRECTORY)).unwrap();
    let sensors = parse_sensors(&input);

    let max = 4000000;
    let min = 0;
    for y in 0..max + 1 {
        let mut ranges = Vec::new();
        for sensor in &sensors {
            let range = covers(y, &sensor);
            if range.is_some() {
                ranges.push(range.unwrap());
            }
        }
        ranges.sort_by_key(|k| k.0);

        let mut it = ranges.iter();
        let first = it.next().unwrap();
        let start = first.0;
        if start > min {
            println!("Point: {},{}", start - 1, y);
            println!(
                "Multiplication: {}",
                (start - 1) as i64 * 4000000 + y as i64
            );
            return;
        }

        let mut end = first.1;
        loop {
            let next = it.next();

            if next.is_none() {
                break;
            }

            let next_val = next.unwrap();

            if next_val.0 > end + 1 {
                println!("Point: {},{}", end + 1, y);
                println!("Multiplication: {}", (end + 1) as i64 * 4000000 + y as i64);
                return;
            }

            end = std::cmp::max(end, next_val.1);
        }

        if end < max {
            println!("Point: {},{}", end + 1, y);
            println!("Multiplication: {}", (end + 1) as i64 * 4000000 + y as i64);
            return;
        }
    }
}

pub fn solve() {
    part1();
    part2();
}
