use phf::phf_map;
use std::fs;

use crate::PROJECT_DIRECTORY;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(PartialEq, Clone, Copy)]
enum Result {
    WIN,
    DRAW,
    LOSE,
}

struct Round {
    our: Move,
    their: Move,
}

fn score_round(round: Round) -> u32 {
    const SCORE_ROCK: u32 = 1;
    const SCORE_PAPER: u32 = 2;
    const SCORE_SCISSORS: u32 = 3;
    const SCORE_DRAW: u32 = 3;
    const SCORE_WIN: u32 = 6;

    if round.our == Move::ROCK {
        if round.their == Move::ROCK {
            return SCORE_ROCK + SCORE_DRAW;
        } else if round.their == Move::PAPER {
            return SCORE_ROCK;
        } else {
            return SCORE_ROCK + SCORE_WIN;
        }
    } else if round.our == Move::PAPER {
        if round.their == Move::ROCK {
            return SCORE_PAPER + SCORE_WIN;
        } else if round.their == Move::PAPER {
            return SCORE_PAPER + SCORE_DRAW;
        } else {
            return SCORE_PAPER;
        }
    } else {
        if round.their == Move::ROCK {
            return SCORE_SCISSORS;
        } else if round.their == Move::PAPER {
            return SCORE_SCISSORS + SCORE_WIN;
        } else {
            return SCORE_SCISSORS + SCORE_DRAW;
        }
    }
}

fn parse_round(our_move: &str, their_move: &str) -> Round {
    static OUR: phf::Map<&str, Move> = phf_map! {
        "X" => Move::ROCK,
        "Y" => Move::PAPER,
        "Z" => Move::SCISSORS,
    };

    static THEIR: phf::Map<&str, Move> = phf_map! {
        "A" => Move::ROCK,
        "B" => Move::PAPER,
        "C" => Move::SCISSORS,
    };

    return Round {
        our: OUR[&our_move],
        their: THEIR[&their_move],
    };
}

fn parse_round_with_result(their_move: &str, result: &str) -> Round {
    static RESULT: phf::Map<&str, Result> = phf_map! {
        "X" => Result::LOSE,
        "Y" => Result::DRAW,
        "Z" => Result::WIN,
    };

    static THEIR: phf::Map<&str, Move> = phf_map! {
        "A" => Move::ROCK,
        "B" => Move::PAPER,
        "C" => Move::SCISSORS,
    };
    let t = THEIR[&their_move];
    let r = RESULT[&result];
    let our: Move;
    if t == Move::ROCK {
        if r == Result::LOSE {
            our = Move::SCISSORS;
        } else if r == Result::DRAW {
            our = Move::ROCK;
        } else {
            our = Move::PAPER;
        }
    } else if t == Move::PAPER {
        if r == Result::LOSE {
            our = Move::ROCK;
        } else if r == Result::DRAW {
            our = Move::PAPER;
        } else {
            our = Move::SCISSORS;
        }
    } else {
        if r == Result::LOSE {
            our = Move::PAPER;
        } else if r == Result::DRAW {
            our = Move::SCISSORS;
        } else {
            our = Move::ROCK;
        }
    }

    return Round {
        our: our,
        their: THEIR[&their_move],
    };
}

pub fn part1() {
    let rounds = fs::read_to_string(format!("{}/day2/input.txt", PROJECT_DIRECTORY)).unwrap();
    let mut sum: u64 = 0;
    for round in rounds.split('\n') {
        if round.is_empty() {
            break;
        }
        let mut moves = round.split(' ');
        let their_move = moves.next().unwrap();
        let our_move = moves.next().unwrap();

        sum += u64::from(score_round(parse_round(our_move, their_move)));
    }
    println!("{}", sum);
}

pub fn part2() {
    let rounds = fs::read_to_string(format!("{}/day2/input.txt", PROJECT_DIRECTORY)).unwrap();
    let mut sum: u64 = 0;
    for round in rounds.split('\n') {
        if round.is_empty() {
            break;
        }
        let mut moves = round.split(' ');
        let their_move = moves.next().unwrap();
        let result = moves.next().unwrap();

        sum += u64::from(score_round(parse_round_with_result(their_move, result)));
    }
    println!("{}", sum);
}

pub fn solve() {
    part1();
    part2();
}
