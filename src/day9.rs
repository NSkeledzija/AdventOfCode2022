use core::panic;
use std::{collections::HashSet, fs};

use crate::PROJECT_DIRECTORY;

fn dir_to_step(dir: &str) -> (i32, i32) {
    if dir == "D" {
        return (-1, 0);
    } else if dir == "U" {
        return (1, 0);
    } else if dir == "R" {
        return (0, 1);
    } else if dir == "L" {
        return (0, -1);
    } else {
        panic!("Invalid direction!");
    }
}

fn print(h: (i32, i32), t: (i32, i32)) {
    let nr_rows = 5;
    let nr_columns = 6;

    for i in (0..nr_rows).rev() {
        for j in 0..nr_columns {
            if h.0 == i && h.1 == j {
                print!("H");
            } else if t.0 == i && t.1 == j {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
    println!("");
}

pub fn part1() {
    println!("Run solver!");
    let input = fs::read_to_string(format!("{}/day9/input.txt", PROJECT_DIRECTORY)).unwrap();

    let lines = input.split("\n");

    let mut h: (i32, i32) = (0, 0);
    let mut t: (i32, i32) = (0, 0);
    print(h, t);
    let mut visited = HashSet::new();
    visited.insert(t);
    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut m = line.split(" ");
        let dir = m.next().unwrap();
        let length: u32 = str::parse(m.next().unwrap()).unwrap();

        let step = dir_to_step(dir);

        println!("{} for {}", dir.clone(), length.clone());

        for _ in 0..length {
            h.0 += step.0;
            h.1 += step.1;

            if h.0 == t.0 {
                if t.1 < h.1 - 1 {
                    t.1 += 1;
                } else if t.1 > h.1 + 1 {
                    t.1 -= 1;
                }
            } else if h.1 == t.1 {
                if t.0 < h.0 - 1 {
                    t.0 += 1;
                } else if t.0 > h.0 + 1 {
                    t.0 -= 1;
                }
            } else {
                if (h.0 - t.0).abs() > 1 || (h.1 - t.1).abs() > 1 {
                    if h.0 > t.0 {
                        t.0 += 1;
                    } else {
                        t.0 -= 1;
                    }
                    if h.1 > t.1 {
                        t.1 += 1;
                    } else {
                        t.1 -= 1;
                    }
                }
            }
            visited.insert(t);
            print(h, t);
        }
    }
    println!("{}", visited.len());
}

pub fn part2() {
    println!("Run part2!");
    let input = fs::read_to_string(format!("{}/day9/input.txt", PROJECT_DIRECTORY)).unwrap();

    let lines = input.split("\n");

    let mut r: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut visited = HashSet::new();
    visited.insert(r[r.len() - 1]);
    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut m = line.split(" ");
        let dir = m.next().unwrap();
        let length: u32 = str::parse(m.next().unwrap()).unwrap();

        let step = dir_to_step(dir);

        println!("{} for {}", dir.clone(), length.clone());

        for _ in 0..length {
            r[0].0 += step.0;
            r[0].1 += step.1;

            for k in 0..r.len() - 1 {
                if r[k].0 == r[k + 1].0 {
                    if r[k + 1].1 < r[k].1 - 1 {
                        r[k + 1].1 += 1;
                    } else if r[k + 1].1 > r[k].1 + 1 {
                        r[k + 1].1 -= 1;
                    }
                } else if r[k].1 == r[k + 1].1 {
                    if r[k + 1].0 < r[k].0 - 1 {
                        r[k + 1].0 += 1;
                    } else if r[k + 1].0 > r[k].0 + 1 {
                        r[k + 1].0 -= 1;
                    }
                } else {
                    if (r[k].0 - r[k + 1].0).abs() > 1 || (r[k].1 - r[k + 1].1).abs() > 1 {
                        if r[k].0 > r[k + 1].0 {
                            r[k + 1].0 += 1;
                        } else {
                            r[k + 1].0 -= 1;
                        }
                        if r[k].1 > r[k + 1].1 {
                            r[k + 1].1 += 1;
                        } else {
                            r[k + 1].1 -= 1;
                        }
                    }
                }

                visited.insert(r[r.len() - 1]);
            }
        }
    }
    println!("{}", visited.len());
}

pub fn solve() {
    part1();
    part2();
}
