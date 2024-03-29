use chrono::Datelike;
use chrono::Utc;
use reqwest::{cookie::Jar, Url};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const PROJECT_DIRECTORY: &str = "/home/nskeledz/projects/AdventOfCode2022";

fn input_url(day: u32) -> Url {
    let url_str = format!("https://adventofcode.com/2022/day/{}/input", day);
    return url_str.parse::<Url>().unwrap();
}

fn input_path(day: u32) -> PathBuf {
    let str = format!("{}/day{}/input.txt", PROJECT_DIRECTORY, day);
    PathBuf::from(str)
}

fn cookie_path() -> PathBuf {
    let str = format!("{}/cookie.txt", PROJECT_DIRECTORY);
    PathBuf::from(str)
}

fn dir_path(day: u32) -> PathBuf {
    let str = format!("{}/day{}", PROJECT_DIRECTORY, day);
    PathBuf::from(str)
}

pub fn download_input(day: u32) -> String {
    let url = input_url(day);
    let jar = Jar::default();

    let cookie = fs::read_to_string(cookie_path()).unwrap();

    for line in cookie.split('\n') {
        jar.add_cookie_str(line, &url);
    }

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();

    let request = client.get(url);

    let response_future = request.send();
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    let response = runtime.block_on(response_future);
    let text = runtime.block_on(response.unwrap().text());
    return text.unwrap();
}

fn initialize_day(day: u32) {
    let dir = dir_path(day);
    if dir.is_dir() {
        assert!(fs::remove_dir_all(&dir).is_ok());
    }
    assert!(fs::create_dir(&dir).is_ok());

    let input_file = input_path(day);
    if input_file.is_file() {
        assert!(fs::remove_file(&input_file).is_ok());
    }

    let input = download_input(day);
    assert!(fs::write(&input_file, &input).is_ok());
}

fn day_initialized(day: u32) -> bool {
    if !dir_path(day).is_dir() {
        return false;
    }
    if !input_path(day).is_file() {
        return false;
    }
    return true;
}

fn parse_day_argument() -> Option<u32> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return None;
    }
    let parsed_day = args[1].parse::<u32>();
    match parsed_day {
        Ok(v) => return Some(v),
        Err(e) => {
            println!("Couldn't parse day for argument: {e:?}");
            return None;
        }
    }
}

fn main() {
    let user_provided_day = parse_day_argument();

    let day: u32;
    if user_provided_day.is_none() {
        if Utc::now().year() == 2022 && Utc::now().month() == 12 {
            day = Utc::now().day();
        } else {
            println!("Provide day!!!!");
            return;
        }
    } else {
        day = user_provided_day.unwrap();
    }

    if day > 25 {
        if user_provided_day.is_some() {
            println!("Day {} is not a part of the advent calendar!", day);
        } else {
            println!("No more advent of code tasks :(");
        }
        return;
    }

    if !day_initialized(day) {
        initialize_day(day);
    }

    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        7 => day7::solve(),
        8 => day8::solve(),
        9 => day9::solve(),
        10 => day10::solve(),
        11 => day11::solve(),
        12 => day12::solve(),
        13 => day13::solve(),
        14 => day14::solve(),
        15 => day15::solve(),

        _ => println!("Unimplemented day you fuck!"),
    };
}
