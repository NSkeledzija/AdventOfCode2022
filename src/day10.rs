use std::fs;

use crate::PROJECT_DIRECTORY;

pub fn part1() {
    let input = fs::read_to_string(format!("{}/day10/input.txt", PROJECT_DIRECTORY)).unwrap();

    let lines = input.split('\n');
    let mut cycle = 0;
    let mut register: i32 = 1;

    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut current_interest = 0;
    let mut total_strength = 0;
    for line in lines {
        if line.is_empty() {
            break;
        }

        let mut tokens = line.split_ascii_whitespace();

        let command = tokens.next().unwrap();
        let arg = tokens.next();

        if command == "addx" {
            cycle += 2;
            if cycle >= interesting_cycles[current_interest] {
                total_strength += interesting_cycles[current_interest] * register;
                if current_interest == interesting_cycles.len() - 1 {
                    break;
                } else {
                    current_interest += 1;
                }
            }
            register += str::parse::<i32>(arg.unwrap()).unwrap();
        } else if command == "noop" {
            cycle += 1;
        }
    }
    println!("{}", total_strength);
}

pub fn part2() {
    let input = fs::read_to_string(format!("{}/day10/input.txt", PROJECT_DIRECTORY)).unwrap();
    let mut lines = input.split('\n');
    let width = 40;
    let height = 6;

    let mut register: i32 = 1;
    let mut executing_add = false;
    let mut add_value = 0;
    for _ in 0..height {
        for column in 0..width {
            if column >= register - 1 && column <= register + 1 {
                print!("#")
            } else {
                print!(".");
            }
            if !executing_add {
                let line = lines.next().unwrap();
                if line.is_empty() {
                    break;
                }
                let mut tokens = line.split_ascii_whitespace();

                let command = tokens.next().unwrap();
                let arg = tokens.next();

                if command == "addx" {
                    add_value = str::parse::<i32>(arg.unwrap()).unwrap();
                    executing_add = true;
                }
            } else {
                register += add_value;
                executing_add = false;
            }
        }
        println!();
    }
}

pub fn solve() {
    part1();
    part2();
}
