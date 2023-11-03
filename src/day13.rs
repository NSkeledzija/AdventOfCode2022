use std::fs;

use core::str::Chars;
use std::cmp::Ordering;
use std::iter::Peekable;

use crate::PROJECT_DIRECTORY;

/*
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9] */

#[derive(Debug, Clone)]
enum Value {
    Int(u32),
    List(Vec<Value>),
}

fn parse_list(list: &mut Peekable<Chars>) -> Vec<Value> {
    let mut out = Vec::new();
    let mut current = list.next().unwrap();
    if current != '[' {
        panic!("List should start with [");
    }

    let mut current_value: String = String::from("");
    loop {
        let peek_val = list.peek().unwrap().clone();
        if peek_val == '[' {
            out.push(Value::List(parse_list(list)));
            continue;
        }
        current = list.next().unwrap();
        if current == ']' {
            if !current_value.is_empty() {
                out.push(Value::Int(str::parse::<u32>(&current_value).unwrap()));
            }
            return out;
        } else if current == ',' {
            if !current_value.is_empty() {
                out.push(Value::Int(str::parse::<u32>(&current_value).unwrap()));
                current_value = String::from("");
            }
        } else if current.is_alphanumeric() {
            current_value.push(current);
        } else {
            panic!("Invalid char {}", current);
        }
    }
}

fn compare_two_lists(left: &Vec<Value>, right: &Vec<Value>) -> Ordering {
    for i in 0..std::cmp::max(left.len(), right.len()) {
        if i >= left.len() {
            return Ordering::Less;
        }
        if i >= right.len() {
            return Ordering::Greater;
        }

        let left_val = left[i].clone();
        let right_val = right[i].clone();

        match (left_val, right_val) {
            (Value::Int(lv), Value::Int(rv)) => {
                if lv < rv {
                    return Ordering::Less;
                } else if lv > rv {
                    return Ordering::Greater;
                } else {
                    continue;
                }
            }
            (Value::List(lv), Value::List(rv)) => {
                let retval = compare_two_lists(&lv, &rv);
                if retval != Ordering::Equal {
                    return retval;
                } else {
                    continue;
                }
            }
            (Value::List(lv), Value::Int(rv)) => {
                let retval = compare_two_lists(&lv, &vec![Value::Int(rv)]);
                if retval != Ordering::Equal {
                    return retval;
                } else {
                    continue;
                }
            }
            (Value::Int(lv), Value::List(rv)) => {
                let retval = compare_two_lists(&vec![Value::Int(lv)], &rv);
                if retval != Ordering::Equal {
                    return retval;
                } else {
                    continue;
                }
            }
        }
    }
    return Ordering::Equal;
}

pub fn part1() {
    let input = fs::read_to_string(format!("{}/day13/input.txt", PROJECT_DIRECTORY)).unwrap();

    let comparisons = input.split("\n\n");

    let mut sum = 0;
    for (i, comparison) in comparisons.enumerate() {
        let mut lists = comparison.split('\n');
        let list1 = lists.next();
        let list2 = lists.next();

        let parsed1 = parse_list(&mut list1.unwrap().chars().peekable());
        let parsed2 = parse_list(&mut list2.unwrap().chars().peekable());

        if compare_two_lists(&parsed1, &parsed2) == Ordering::Less {
            sum += i + 1;
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    let input = fs::read_to_string(format!("{}/day13/input.txt", PROJECT_DIRECTORY)).unwrap();

    let comparisons = input.split("\n\n");

    let mut all_lists: Vec<Vec<Value>> = Vec::new();
    for (_, comparison) in comparisons.enumerate() {
        let mut lists = comparison.split('\n');
        let list1 = lists.next();
        let list2 = lists.next();

        let parsed1 = parse_list(&mut list1.unwrap().chars().peekable());
        let parsed2 = parse_list(&mut list2.unwrap().chars().peekable());
        all_lists.push(parsed1);
        all_lists.push(parsed2);
    }

    let divider1 = "[[2]]";
    let divider2 = "[[6]]";

    all_lists.push(parse_list(&mut divider1.chars().peekable()));
    all_lists.push(parse_list(&mut divider2.chars().peekable()));
    all_lists.sort_by(compare_two_lists);

    let mut signal: u32 = 1;
    for (i, list) in all_lists.iter().enumerate() {
        if list.len() > 0 {
            let val = &list[0];
            match val {
                Value::List(v) => {
                    if v.len() == 1 {
                        let inner_val = &v[0];
                        match inner_val {
                            Value::Int(iv) => {
                                if *iv == 6 || *iv == 2 {
                                    signal *= (i + 1) as u32;
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }
    println!("{}", signal);
}

pub fn solve() {
    part1();
    part2();
}
