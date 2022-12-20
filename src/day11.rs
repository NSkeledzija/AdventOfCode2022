use std::collections::VecDeque;
use std::fs;

use crate::PROJECT_DIRECTORY;
#[derive(Debug, Clone, Copy)]
enum Operation {
    MUL,
    ADD,
    SQUARE,
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    starting_items: VecDeque<u64>,
    operation: Operation,
    arg: u32,
    test: u32,
    if_true: u32,
    if_false: u32,
    inspected: u32,
}

/*
Monkey 0:
  Starting items: 54, 98, 50, 94, 69, 62, 53, 85
  Operation: new = old * 13
  Test: divisible by 3
    If true: throw to monkey 2
    If false: throw to monkey 1
*/

pub fn parse_monkey_id(id: &str) -> u32 {
    let tokens = id.split_ascii_whitespace();
    str::parse::<u32>(tokens.last().unwrap().trim_end_matches(':')).unwrap()
}

pub fn parse_starting_items(starting_items: &str) -> VecDeque<u64> {
    let tokens = starting_items.split(": ").last().unwrap().split(", ");
    let mut out = VecDeque::new();
    for current in tokens {
        out.push_back(str::parse::<u64>(current).unwrap());
    }
    out
}

fn parse_operation(operation: &str) -> (Operation, u32) {
    if operation.contains('*') {
        let second_arg = operation.split("* ").last().unwrap();
        if second_arg.bytes().all(|c| c.is_ascii_digit()) {
            let number = str::parse::<u32>(second_arg).unwrap();
            return (Operation::MUL, number);
        } else {
            return (Operation::SQUARE, 1);
        }
    } else {
        let number = str::parse::<u32>(operation.split("+ ").last().unwrap()).unwrap();
        return (Operation::ADD, number);
    };
}

pub fn parse_test(test: &str) -> u32 {
    str::parse::<u32>(test.split("by ").last().unwrap()).unwrap()
}

pub fn parse_throw(throw: &str) -> u32 {
    str::parse::<u32>(throw.split_ascii_whitespace().last().unwrap()).unwrap()
}

fn parse_monkey(monkey: &str) -> Monkey {
    let mut tokens = monkey.split('\n');

    let id = parse_monkey_id(tokens.next().unwrap());
    let starting_items = parse_starting_items(tokens.next().unwrap());
    let (operation, arg) = parse_operation(tokens.next().unwrap());
    let test = parse_test(tokens.next().unwrap());
    let if_true = parse_throw(tokens.next().unwrap());
    let if_false = parse_throw(tokens.next().unwrap());

    Monkey {
        id,
        starting_items,
        operation,
        arg,
        test,
        if_true,
        if_false,
        inspected: 0,
    }
}

pub fn part1() {
    let input = fs::read_to_string(format!("{}/day11/input.txt", PROJECT_DIRECTORY)).unwrap();
    let monkey_strings = input.split("\n\n");

    let mut monkeys = Vec::new();
    for monkey in monkey_strings {
        monkeys.push(parse_monkey(monkey));
    }

    for round in 0..20 {
        println!("Round: {}", round);
        for monkey_id in 0..monkeys.len().clone() {
            loop {
                let current_item = monkeys[monkey_id].starting_items.pop_front();
                if current_item.is_none() {
                    break;
                }
                monkeys[monkey_id].inspected += 1;
                let mut worry_level = current_item.unwrap();
                let operation = monkeys[monkey_id].operation.clone();
                let arg = &monkeys[monkey_id].arg;
                match operation {
                    Operation::ADD => worry_level += *arg as u64,
                    Operation::MUL => worry_level *= *arg as u64,
                    Operation::SQUARE => worry_level *= worry_level,
                }
                worry_level /= 3;

                let test = &monkeys[monkey_id].test;
                if worry_level % *test as u64 == 0 {
                    let target = (monkeys[monkey_id].if_true) as usize;
                    monkeys[target].starting_items.push_back(worry_level);
                } else {
                    let target = (monkeys[monkey_id].if_false) as usize;
                    monkeys[target].starting_items.push_back(worry_level);
                }
            }
        }
    }

    let mut inspected = Vec::new();
    for monkey in monkeys {
        inspected.push(monkey.inspected);
    }

    inspected.sort();

    inspected.reverse();
    let monkey_business = inspected[0] * inspected[1];

    println!("{}", monkey_business);
}

pub fn part2() {
    let input = fs::read_to_string(format!("{}/day11/input.txt", PROJECT_DIRECTORY)).unwrap();
    let monkey_strings = input.split("\n\n");

    let mut monkeys = Vec::new();
    for monkey in monkey_strings {
        monkeys.push(parse_monkey(monkey));
    }

    let mut smallest_common: u64 = 1;

    for monkey in &monkeys {
        smallest_common *= monkey.test as u64;
    }

    for round in 0..10000 {
        println!("Round: {}", round);
        for monkey_id in 0..monkeys.len().clone() {
            loop {
                let current_item = monkeys[monkey_id].starting_items.pop_front();
                if current_item.is_none() {
                    break;
                }
                monkeys[monkey_id].inspected += 1;
                let mut worry_level = current_item.unwrap();
                let operation = monkeys[monkey_id].operation.clone();
                let arg = &monkeys[monkey_id].arg;
                match operation {
                    Operation::ADD => worry_level += *arg as u64,
                    Operation::MUL => worry_level *= *arg as u64,
                    Operation::SQUARE => worry_level *= worry_level,
                }
                worry_level = worry_level % smallest_common;

                let test = &monkeys[monkey_id].test;
                if worry_level % *test as u64 == 0 {
                    let target = (monkeys[monkey_id].if_true) as usize;
                    monkeys[target].starting_items.push_back(worry_level);
                } else {
                    let target = (monkeys[monkey_id].if_false) as usize;
                    monkeys[target].starting_items.push_back(worry_level);
                }
            }
        }
    }

    let mut inspected = Vec::new();
    for monkey in monkeys {
        inspected.push(monkey.inspected);
    }

    inspected.sort();

    inspected.reverse();
    let monkey_business = inspected[0] as u64 * inspected[1] as u64;

    println!("{}", monkey_business);
}
pub fn solve() {
    part1();
    part2();
}
