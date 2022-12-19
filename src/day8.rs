use std::fs;

use crate::PROJECT_DIRECTORY;

pub fn part1() {
    let input = fs::read_to_string(format!("{}/day8/input.txt", PROJECT_DIRECTORY)).unwrap();

    let rows: Vec<&str> = input.split("\n").collect();

    let nr_rows = rows.len();
    let nr_columns = rows[0].len();

    let mut visibility: Vec<Vec<bool>> = Vec::new();
    let mut heights: Vec<Vec<i8>> = Vec::new();

    for row in 0..nr_rows {
        heights.push(Vec::new());
        visibility.push(Vec::new());
        for c in rows[row].chars() {
            heights[row].push(c.to_digit(10).unwrap() as i8);
            visibility[row].push(false);
        }
    }

    for row in 0..nr_rows {
        let mut max = -1;
        for column in 0..nr_columns {
            if heights[row][column] > max {
                visibility[row][column] = true;
                max = heights[row][column];
            }
        }
    }

    for row in 0..nr_rows {
        let mut max = -1;
        for column in (0..nr_columns).rev() {
            if heights[row][column] > max {
                visibility[row][column] = true;
                max = heights[row][column];
            }
        }
    }

    for column in 0..nr_columns {
        let mut max = -1;
        for row in 0..nr_rows {
            if heights[row][column] > max {
                visibility[row][column] = true;
                max = heights[row][column];
            }
        }
    }

    for column in 0..nr_columns {
        let mut max = -1;
        for row in (0..nr_rows).rev() {
            if heights[row][column] > max {
                visibility[row][column] = true;
                max = heights[row][column];
            }
        }
    }

    let mut count = 0;
    for row in 0..nr_rows {
        for column in 0..nr_columns {
            if visibility[row][column] {
                count += 1;
            }
        }
    }

    println!("Total visible: {}", count);
}

pub fn viewing_distance_in_direction(
    heights: &Vec<Vec<i8>>,
    starting_row: usize,
    starting_column: usize,
    step: (i8, i8),
) -> u8 {
    let mut i = starting_row as i8 + step.0;
    let mut j = starting_column as i8 + step.1;
    let mut viewing_distance = 0;
    let height = heights[starting_row][starting_column];
    while i >= 0 && i < heights.len() as i8 && j >= 0 && j < heights[0].len() as i8 {
        viewing_distance += 1;
        if heights[i as usize][j as usize] >= height {
            break;
        }
        i += step.0;
        j += step.1;
    }
    return viewing_distance;
}

pub fn scenic_score(heights: &Vec<Vec<i8>>, row: usize, column: usize) -> u64 {
    let mut scenic_score: u64 = 1;
    scenic_score *= viewing_distance_in_direction(heights, row, column, (0, 1)) as u64;
    scenic_score *= viewing_distance_in_direction(heights, row, column, (0, -1)) as u64;
    scenic_score *= viewing_distance_in_direction(heights, row, column, (1, 0)) as u64;
    scenic_score *= viewing_distance_in_direction(heights, row, column, (-1, 0)) as u64;

    return scenic_score;
}

pub fn part2() {
    let input = fs::read_to_string(format!("{}/day8/input.txt", PROJECT_DIRECTORY)).unwrap();

    let rows: Vec<&str> = input.split("\n").collect();

    let nr_rows = rows.len();
    let nr_columns = rows[0].len();

    let mut visibility: Vec<Vec<bool>> = Vec::new();
    let mut heights: Vec<Vec<i8>> = Vec::new();

    for row in 0..nr_rows {
        heights.push(Vec::new());
        visibility.push(Vec::new());
        for c in rows[row].chars() {
            heights[row].push(c.to_digit(10).unwrap() as i8);
            visibility[row].push(false);
        }
    }

    let mut max_scenic_score = 0;
    for row in 0..nr_rows {
        for column in 0..nr_columns {
            max_scenic_score = std::cmp::max(scenic_score(&heights, row, column), max_scenic_score);
        }
    }

    println!("Max scenic score: {}", max_scenic_score);
}

pub fn solve() {
    part1();
    part2();
}
