use std::fs;

use crate::PROJECT_DIRECTORY;

pub fn parse_rock(line: &str) -> Vec<(usize, usize)> {
    let points = line.split(" -> ");

    let mut rock: Vec<(usize, usize)> = Vec::new();
    for point in points {
        let mut vals = point.split(',');
        let x: usize = str::parse::<usize>(vals.next().unwrap()).unwrap();
        let y: usize = str::parse::<usize>(vals.next().unwrap()).unwrap();

        rock.push((x, y));
    }
    return rock;
}

pub fn parse_rocks(input: &str) -> Vec<Vec<(usize, usize)>> {
    let mut rocks: Vec<Vec<(usize, usize)>> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            break;
        }
        rocks.push(parse_rock(line));
    }
    return rocks;
}

pub fn build_cave(input: &str) -> (usize, Vec<Vec<char>>) {
    let rocks = parse_rocks(input);

    let mut max_x: usize = 0;
    let mut min_x: usize = usize::MAX;
    let mut max_y: usize = 0;
    let min_y: usize = 0;
    for rock in &rocks {
        for (x, y) in rock {
            max_x = std::cmp::max(*x, max_x);
            min_x = std::cmp::min(*x, min_x);
            max_y = std::cmp::max(*y, max_y);
        }
    }

    let row = vec!['.'; max_x + 1 - min_x];
    let mut grid = vec![row; max_y + 1 - min_y];

    for rock in &rocks {
        let mut prev: Option<(usize, usize)> = None;
        for (real_x, real_y) in rock {
            let x = *real_x - min_x;
            let y = *real_y - min_y;
            if prev.is_some() {
                if x == prev.unwrap().0 {
                    for r in
                        std::cmp::min(y, prev.unwrap().1)..std::cmp::max(y, prev.unwrap().1) + 1
                    {
                        grid[r][x] = '#';
                    }
                }

                if y == prev.unwrap().1 {
                    for r in
                        std::cmp::min(x, prev.unwrap().0)..std::cmp::max(x, prev.unwrap().0) + 1
                    {
                        grid[y][r] = '#';
                    }
                }
            }
            prev = Some((x, y));
        }
    }

    return (min_x, grid);
}

pub fn build_big_cave(input: &str) -> (usize, Vec<Vec<char>>) {
    let rocks = parse_rocks(input);

    let mut max_y: usize = 0;
    let min_y: usize = 0;
    for rock in &rocks {
        for (x, y) in rock {
            max_y = std::cmp::max(*y, max_y);
        }
    }

    let max_x = 500 + max_y + 2;
    let min_x = 500 - max_y - 2;

    let row = vec!['.'; max_x + 1 - min_x];
    let mut grid = vec![row; max_y + 1 - min_y];

    for rock in &rocks {
        let mut prev: Option<(usize, usize)> = None;
        for (real_x, real_y) in rock {
            let x = *real_x - min_x;
            let y = *real_y - min_y;
            if prev.is_some() {
                if x == prev.unwrap().0 {
                    for r in
                        std::cmp::min(y, prev.unwrap().1)..std::cmp::max(y, prev.unwrap().1) + 1
                    {
                        grid[r][x] = '#';
                    }
                }

                if y == prev.unwrap().1 {
                    for r in
                        std::cmp::min(x, prev.unwrap().0)..std::cmp::max(x, prev.unwrap().0) + 1
                    {
                        grid[y][r] = '#';
                    }
                }
            }
            prev = Some((x, y));
        }
    }

    return (min_x, grid);
}

pub fn part1() {
    let input = fs::read_to_string(format!("{}/day14/input.txt", PROJECT_DIRECTORY)).unwrap();
    let (x_offset, mut cave) = build_cave(&input);
    let mut sand_path: Vec<(usize, usize)> = Vec::new();
    let mut prev_path: Option<Vec<(usize, usize)>> = None;

    let real_source_x = 500;

    let source_x = real_source_x - x_offset;
    let source_y = 0;

    let mut sand_x = source_x;
    let mut sand_y = source_y;
    let mut resting_sand = 0;
    loop {
        // on bottom of the rocky part of the cave, always falls to infinity
        if sand_y == (cave.len() - 1) {
            if prev_path.as_ref().is_some() {
                if prev_path.as_ref().unwrap().eq(&sand_path) {
                    break;
                }
            }
            prev_path = Some(sand_path.clone());
            sand_path.clear();

            sand_x = source_x;
            sand_y = source_y;
        }
        // air under the current position, fall down
        else if cave[sand_y + 1][sand_x] == '.' {
            sand_y += 1;
            sand_path.push((sand_x, sand_y));
        }
        // can't fall down, on left edge, falls to infinity
        else if sand_x == 0 {
            if prev_path.as_ref().is_some() {
                if prev_path.as_ref().unwrap().eq(&sand_path) {
                    break;
                }
            }
            prev_path = Some(sand_path.clone());
            sand_path.clear();

            sand_x = source_x;
            sand_y = source_y;
        }
        // can't fall down, not on left edge, air down left, fall diagonally left
        else if cave[sand_y + 1][sand_x - 1] == '.' {
            sand_y += 1;
            sand_x -= 1;
            sand_path.push((sand_x, sand_y));
        }
        // can't fall down, or left, on right edge, falls to infinity
        else if sand_x == (cave[0].len() - 1) {
            if prev_path.as_ref().is_some() {
                if prev_path.as_ref().unwrap().eq(&sand_path) {
                    break;
                }
            }
            prev_path = Some(sand_path.clone());
            sand_path.clear();

            sand_x = source_x;
            sand_y = source_y;
        }
        // can't fall down, or left, not on right edge, air down right, falls diagonally right.
        else if cave[sand_y + 1][sand_x + 1] == '.' {
            sand_y += 1;
            sand_x += 1;
            sand_path.push((sand_x, sand_y));
        }
        // can't move, end the trajectory.
        else {
            resting_sand += 1;
            cave[sand_y][sand_x] = 'o';

            if prev_path.as_ref().is_some() {
                if prev_path.as_ref().unwrap().eq(&sand_path) {
                    break;
                }
            }
            prev_path = Some(sand_path.clone());
            sand_path.clear();

            sand_x = source_x;
            sand_y = source_y;
        }
    }

    println!("Resting sand: {}", resting_sand);
}

pub fn part2() {
    let input = fs::read_to_string(format!("{}/day14/input.txt", PROJECT_DIRECTORY)).unwrap();
    let (x_offset, mut cave) = build_big_cave(&input);

    let mut lowest_empty = Vec::new();
    for _ in &cave[0] {
        lowest_empty.push('.');
    }
    let mut floor = Vec::new();
    for _ in &cave[0] {
        floor.push('#');
    }

    cave.push(lowest_empty);
    cave.push(floor);

    let real_source_x = 500;

    let source_x = real_source_x - x_offset;
    let source_y = 0;

    let mut sand_x = source_x;
    let mut sand_y = source_y;
    let mut resting_sand = 0;
    loop {
        // on bottom of the rocky part of the cave, shouldn't happen with big cave
        if sand_y == (cave.len() - 1) {
            panic!("Wrong!");
        }
        // air under the current position, fall down
        else if cave[sand_y + 1][sand_x] == '.' {
            sand_y += 1;
        }
        // can't fall down, shouldn't happen with big cave
        else if sand_x == 0 {
            panic!("Wrong!");
        }
        // can't fall down, not on left edge, air down left, fall diagonally left
        else if cave[sand_y + 1][sand_x - 1] == '.' {
            sand_y += 1;
            sand_x -= 1;
        }
        // can't fall down, or left, on right edge, shouldn't happen with big cave
        else if sand_x == (cave[0].len() - 1) {
            panic!("Wrong")
        }
        // can't fall down, or left, not on right edge, air down right, falls diagonally right.
        else if cave[sand_y + 1][sand_x + 1] == '.' {
            sand_y += 1;
            sand_x += 1;
        }
        // can't move, end the trajectory.
        else {
            resting_sand += 1;
            cave[sand_y][sand_x] = 'o';

            if sand_y == source_y && sand_x == source_x {
                break;
            }

            sand_x = source_x;
            sand_y = source_y;
        }
    }

    println!("Resting sand: {}", resting_sand);
}

pub fn solve() {
    part1();
    part2();
}
