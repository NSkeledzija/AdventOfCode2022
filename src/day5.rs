use std::collections::VecDeque;
use std::fs;

use crate::PROJECT_DIRECTORY;

fn parse_stacks(stacks: &str) -> VecDeque<VecDeque<char>> {
    //             [G] [W]         [Q]
    // [Z]         [Q] [M]     [J] [F]
    // [V]         [V] [S] [F] [N] [R]
    // [T]         [F] [C] [H] [F] [W] [P]
    // [B] [L]     [L] [J] [C] [V] [D] [V]
    // [J] [V] [F] [N] [T] [T] [C] [Z] [W]
    // [G] [R] [Q] [H] [Q] [W] [Z] [G] [B]
    // [R] [J] [S] [Z] [R] [S] [D] [L] [J]
    //  1   2   3   4   5   6   7   8   9

    let mut lines = stacks.split('\n');
    let last_line = lines.next_back().unwrap();
    let indexes = last_line.split_whitespace();
    let number_of_stacks: usize = str::parse(&indexes.last().unwrap()).unwrap();

    let mut stacks_out: VecDeque<VecDeque<char>> = VecDeque::new();

    for _ in 0..number_of_stacks {
        stacks_out.push_back(VecDeque::new());
    }

    for line in lines {
        parse_stack_line(&mut stacks_out, line);
    }
    return stacks_out;
}

fn parse_stack_line(out: &mut VecDeque<VecDeque<char>>, stack_line: &str) {
    for i in 0..out.len() {
        let c = stack_line.chars().nth(i * 4 + 1).unwrap();
        if c != ' ' {
            if c < 'A' || c > 'Z' {
                dbg!(c);
                panic!("c out of range!");
            }
            out[i].push_front(c);
        }
    }
}

fn parse_moves(move_lines: &str) -> Vec<(usize, usize, usize)> {
    let tokens = move_lines.split('\n');
    let mut out: Vec<(usize, usize, usize)> = Vec::new();
    for line in tokens {
        if line.is_empty() {
            break;
        }

        out.push(parse_move(line));
    }

    return out;
}

fn parse_move(move_line: &str) -> (usize, usize, usize) {
    //move 6 from 5 to 7
    let mut tokens = move_line.split_whitespace();
    let count: usize = str::parse(tokens.nth(1).unwrap()).unwrap();
    let source: usize = str::parse(tokens.nth(1).unwrap()).unwrap();
    let dest: usize = str::parse(tokens.nth(1).unwrap()).unwrap();

    return (count, source, dest);
}

fn rearrange(
    stacks: &VecDeque<VecDeque<char>>,
    moves: &Vec<(usize, usize, usize)>,
) -> VecDeque<VecDeque<char>> {
    let mut out = stacks.to_owned();
    for (count, src, dest) in moves {
        for _ in 0usize..*count {
            let source_stack = out.get_mut(*src - 1).unwrap();
            let moved_crate = source_stack.pop_back().unwrap();
            let dest_stack = out.get_mut(*dest - 1).unwrap();

            dest_stack.push_back(moved_crate);
        }
    }
    return out;
}

fn rearrange_9001(
    stacks: &VecDeque<VecDeque<char>>,
    moves: &Vec<(usize, usize, usize)>,
) -> VecDeque<VecDeque<char>> {
    let mut out = stacks.to_owned();
    for (count, src, dest) in moves {
        let mut moved_crates: VecDeque<char> = VecDeque::new();
        for _ in 0usize..*count {
            let source_stack = out.get_mut(*src - 1).unwrap();
            moved_crates.push_back(source_stack.pop_back().unwrap());
        }

        for _ in 0usize..*count {
            let dest_stack = out.get_mut(*dest - 1).unwrap();
            dest_stack.push_back(moved_crates.pop_back().unwrap());
        }
    }
    return out;
}

fn part1() {
    let data = fs::read_to_string(format!("{}/day5/input.txt", PROJECT_DIRECTORY)).unwrap();
    let mut parts = data.split("\n\n");

    let stacks_input = parts.next().unwrap();
    let moves_input = parts.next().unwrap();

    let stacks = parse_stacks(stacks_input);
    let moves = parse_moves(moves_input);

    let out = rearrange(&stacks, &moves);
    for mut o in out {
        print!("{}", o.pop_back().unwrap().clone());
    }
    println!("");
}

fn part2() {
    let data = fs::read_to_string(format!("{}/day5/input.txt", PROJECT_DIRECTORY)).unwrap();
    let mut parts = data.split("\n\n");

    let stacks_input = parts.next().unwrap();
    let moves_input = parts.next().unwrap();

    let stacks = parse_stacks(stacks_input);
    let moves = parse_moves(moves_input);

    let out = rearrange_9001(&stacks, &moves);
    for mut o in out {
        print!("{}", o.pop_back().unwrap().clone());
    }
    println!("");
}

pub fn solve() {
    // example();
    part1();
    part2();
}
