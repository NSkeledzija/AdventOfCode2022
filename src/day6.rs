use std::fs;

use crate::PROJECT_DIRECTORY;

fn first_repeating_index(data: &[u8], marker_len: usize) -> usize {
    for j in 0..marker_len {
        for k in j + 1usize..marker_len {
            if data[j] == data[k] {
                return j + 1;
            }
        }
    }
    return marker_len;
}

fn solve_part(marker_len: usize) {
    println!("Solve part with marker_len: {}", marker_len);
    let data = fs::read(format!("{}/day6/input.txt", PROJECT_DIRECTORY)).unwrap();
    let mut i = 0usize;
    while i < data.len() - (marker_len - 1) {
        let current = &data[i..i + marker_len];
        let first_repeating = first_repeating_index(current, marker_len);

        if first_repeating < marker_len {
            i += first_repeating;
        } else {
            println!("First marker after: {}", i + marker_len);
            return;
        }
    }

    println!("No marker found!");
}

fn part1() {
    solve_part(4);
}

fn part2() {
    solve_part(14);
}

pub fn solve() {
    part1();
    part2();
}
