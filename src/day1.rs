use std::cmp;
use std::fs;

use std::collections::BinaryHeap;

use crate::PROJECT_DIRECTORY;

pub fn part1() {
    let input = fs::read_to_string(format!("{}/day1/input.txt", PROJECT_DIRECTORY)).unwrap();
    let elves = input.split("\n\n");

    let mut max: u64 = 0;
    for elf in elves {
        let calories = elf.split("\n");

        let total = calories
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .sum();
        max = cmp::max(total, max);
    }
    println!("Max calories: {}", max);
}

pub fn part2() {
    let input = fs::read_to_string(format!("{}/day1/input.txt", PROJECT_DIRECTORY)).unwrap();
    let elves = input.split("\n\n");

    let mut heap = BinaryHeap::<u64>::new();

    for elf in elves {
        let calories = elf.split("\n");

        let total = calories
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .sum();
        heap.push(total);
    }

    let mut max_total: u64 = 0;
    for _ in 0..3 {
        max_total += heap.pop().unwrap();
    }
    println!("Total amongst top 3 elves: {}", max_total);
}

pub fn solve() {
    part1();
    part2();
}
