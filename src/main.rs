use chrono::Datelike;
use reqwest::{cookie::Jar, Url};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod day1;
mod day2;
mod day3;
mod day4;

const PROJECT_DIRECTORY: &str = "/home/niksaskeledzija/projects/adventofcode";

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

fn get_current_day() -> u32 {
    let current_day = chrono::Utc::now().day();
    println!("Using current day as fallback: {current_day}");
    return current_day;
}

fn main() {
    let user_provided_day = parse_day_argument();

    let day = match user_provided_day {
        Some(value) => value,
        None => get_current_day(),
    };

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

        _ => println!("Unimplemented day you fuck!"),
    };
}
