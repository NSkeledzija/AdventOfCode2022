use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

use crate::PROJECT_DIRECTORY;

fn char_priority(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        return 1 + (c as u32 - 'a' as u32);
    } else {
        return 27 + (c as u32 - 'A' as u32);
    }
}

fn part1() {
    let backpacks = fs::read_to_string(format!("{}/day3/input.txt", PROJECT_DIRECTORY)).unwrap();
    let mut sum: u64 = 0;
    for backpack in backpacks.split('\n') {
        let len = backpack.len();
        if backpack.is_empty() {
            break;
        }
        let slot_len = len / 2;

        let mut slot1_set = HashSet::<char>::new();
        let mut slot2_set = HashSet::<char>::new();

        for (i, c) in backpack.chars().enumerate() {
            if i < slot_len {
                slot1_set.insert(c);
            } else {
                slot2_set.insert(c);
            }
        }

        let mut intersection = slot1_set.intersection(&slot2_set);

        sum += u64::from(char_priority(intersection.next().unwrap().clone()));
    }
    dbg!(sum);
}

fn part2() {
    let backpacks = fs::read_to_string(format!("{}/day3/input.txt", PROJECT_DIRECTORY)).unwrap();
    let mut sum: u64 = 0;
    for (first, second, third) in backpacks.split('\n').tuples() {
        let mut first_set = HashSet::<char>::new();
        let mut second_set = HashSet::<char>::new();
        let mut third_set = HashSet::<char>::new();

        for c in first.chars() {
            first_set.insert(c);
        }
        for c in second.chars() {
            second_set.insert(c);
        }
        for c in third.chars() {
            third_set.insert(c);
        }

        let intersect: HashSet<char> = first_set.intersection(&second_set).cloned().collect();
        let mut total_intersect = intersect.intersection(&third_set);
        sum += u64::from(char_priority(total_intersect.next().unwrap().clone()));
    }
    dbg!(sum);
}

pub fn solve() {
    part1();
    part2();
}
