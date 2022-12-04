use std::cmp::max;
use std::cmp::min;
use std::fs;
use std::ops::Range;

use crate::PROJECT_DIRECTORY;

fn range_overlap(r1: &Range<u32>, r2: &Range<u32>) -> Option<Range<u32>> {
    if max(r1.start, r2.start) <= min(r1.end, r2.end) {
        return Some(Range {
            start: max(r1.start, r2.start),
            end: min(r1.end, r2.end),
        });
    } else {
        None
    }
}

fn range_contains_other(r1: &Range<u32>, r2: &Range<u32>) -> bool {
    if r1.start <= r2.start && r1.end >= r2.end {
        return true;
    } else if r2.start <= r1.start && r2.end >= r1.end {
        return true;
    } else {
        return false;
    }
}

fn parse_range(range: &str) -> Range<u32> {
    // 7-50
    let mut vals = range.split('-');

    let start = str::parse::<u32>(vals.next().unwrap()).unwrap();
    let end = str::parse::<u32>(vals.next().unwrap()).unwrap();
    Range { start, end }
}

fn parse_ranges(ranges: &str) -> (Range<u32>, Range<u32>) {
    // 7-50,8-33
    let mut range_strings = ranges.split(',');
    let first = range_strings.next().unwrap();
    let second = range_strings.next().unwrap();

    return (parse_range(first), parse_range(second));
}
fn part1() {
    let data = fs::read_to_string(format!("{}/day4/input.txt", PROJECT_DIRECTORY)).unwrap();
    let elves = data.split('\n');
    let mut count = 0;
    for elf in elves {
        if elf.is_empty() {
            break;
        }
        let (first, second) = parse_ranges(elf);

        if range_contains_other(&first, &second) {
            count += 1;
        }
    }
    dbg!(count);
}

fn part2() {
    let data = fs::read_to_string(format!("{}/day4/input.txt", PROJECT_DIRECTORY)).unwrap();
    let elves = data.split('\n');
    let mut count = 0;
    for elf in elves {
        if elf.is_empty() {
            break;
        }
        let (first, second) = parse_ranges(elf);

        if range_overlap(&first, &second).is_some() {
            count += 1;
        }
    }
    dbg!(count);
}

pub fn solve() {
    part1();
    part2();
}
