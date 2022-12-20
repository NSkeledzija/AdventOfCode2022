use std::{collections::VecDeque, fs};

use crate::PROJECT_DIRECTORY;

pub fn part1() {
    let input = fs::read_to_string(format!("{}/day12/input.txt", PROJECT_DIRECTORY)).unwrap();

    let lines = input.split('\n');

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut distances: Vec<Vec<Option<usize>>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        let mut row_distances = Vec::new();
        for c in line.chars() {
            row.push(c);
            row_distances.push(None);
        }
        grid.push(row);
        distances.push(row_distances);
    }

    let mut end_i = 0;
    let mut end_j = 0;
    let mut start_i = 0;
    let mut start_j = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'E' {
                end_i = i;
                end_j = j;
            } else if *c == 'S' {
                start_i = i;
                start_j = j;
            }
        }
    }

    let mut to_visit = VecDeque::new();

    to_visit.push_back((start_i, start_j));

    distances[start_i][start_j] = Some(0);
    grid[start_i][start_j] = 'a';
    grid[end_i][end_j] = 'z';
    loop {
        let (i, j) = to_visit.pop_front().unwrap();
        if (i == end_i) && (j == end_j) {
            println!("Distance: {}", distances[i][j].unwrap());
            break;
        }

        if i > 0 {
            // Check left direction
            let start_height = grid[i][j] as u8;
            let end_height = grid[i - 1][j] as u8;
            if start_height > end_height || end_height - start_height <= 1 {
                if distances[i - 1][j].is_none() {
                    distances[i - 1][j] = Some(distances[i][j].unwrap() + 1);
                    to_visit.push_back((i - 1, j));
                }
            }
        }
        if i < grid.len() - 1 {
            // Check right direction
            let start_height = grid[i][j] as u8;
            let end_height = grid[i + 1][j] as u8;
            if start_height > end_height || end_height - start_height <= 1 {
                if distances[i + 1][j].is_none() {
                    distances[i + 1][j] = Some(distances[i][j].unwrap() + 1);
                    to_visit.push_back((i + 1, j));
                }
            }
        }
        if j > 0 {
            // Check up direction
            let start_height = grid[i][j] as u8;
            let end_height = grid[i][j - 1] as u8;
            if start_height > end_height || end_height - start_height <= 1 {
                if distances[i][j - 1].is_none() {
                    distances[i][j - 1] = Some(distances[i][j].unwrap() + 1);
                    to_visit.push_back((i, j - 1));
                }
            }
        }
        if j < grid[0].len() - 1 {
            // Check down direction
            let start_height = grid[i][j] as u8;
            let end_height = grid[i][j + 1] as u8;
            if start_height > end_height || end_height - start_height <= 1 {
                if distances[i][j + 1].is_none() {
                    distances[i][j + 1] = Some(distances[i][j].unwrap() + 1);
                    to_visit.push_back((i, j + 1));
                }
            }
        }
    }
}

pub fn part2() {
    let input = fs::read_to_string(format!("{}/day12/input.txt", PROJECT_DIRECTORY)).unwrap();

    let lines = input.split('\n');

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut distances: Vec<Vec<Option<usize>>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        let mut row_distances = Vec::new();
        for c in line.chars() {
            row.push(c);
            row_distances.push(None);
        }
        grid.push(row);
        distances.push(row_distances);
    }

    let mut start_i = 0;
    let mut start_j = 0;

    for (i, row) in grid.iter_mut().enumerate() {
        for (j, c) in row.iter_mut().enumerate() {
            if *c == 'E' {
                start_i = i;
                start_j = j;
                *c = 'z';
            } else if *c == 'S' {
                *c = 'a';
            }
        }
    }

    let mut to_visit = VecDeque::new();

    to_visit.push_back((start_i, start_j));

    distances[start_i][start_j] = Some(0);
    loop {
        let (i, j) = to_visit.pop_front().unwrap();
        if grid[i][j] == 'a' {
            println!("Distance: {}", distances[i][j].unwrap());
            break;
        }

        if i > 0 {
            // Check left direction
            let start_height = grid[i][j] as u8;
            let end_height = grid[i - 1][j] as u8;
            if end_height > start_height || start_height - end_height <= 1 {
                if distances[i - 1][j].is_none() {
                    distances[i - 1][j] = Some(distances[i][j].unwrap() + 1);
                    to_visit.push_back((i - 1, j));
                }
            }
        }
        if i < grid.len() - 1 {
            // Check right direction
            let start_height = grid[i][j] as u8;
            let end_height = grid[i + 1][j] as u8;
            if end_height > start_height || start_height - end_height <= 1 {
                if distances[i + 1][j].is_none() {
                    distances[i + 1][j] = Some(distances[i][j].unwrap() + 1);
                    to_visit.push_back((i + 1, j));
                }
            }
        }
        if j > 0 {
            // Check up direction
            let start_height = grid[i][j] as u8;
            let end_height = grid[i][j - 1] as u8;
            if end_height > start_height || start_height - end_height <= 1 {
                if distances[i][j - 1].is_none() {
                    distances[i][j - 1] = Some(distances[i][j].unwrap() + 1);
                    to_visit.push_back((i, j - 1));
                }
            }
        }
        if j < grid[0].len() - 1 {
            // Check down direction
            let start_height = grid[i][j] as u8;
            let end_height = grid[i][j + 1] as u8;
            if end_height > start_height || start_height - end_height <= 1 {
                if distances[i][j + 1].is_none() {
                    distances[i][j + 1] = Some(distances[i][j].unwrap() + 1);
                    to_visit.push_back((i, j + 1));
                }
            }
        }
    }
}

pub fn solve() {
    part1();
    part2();
}
