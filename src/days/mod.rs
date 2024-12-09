use std::fs::File;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run_day(day: u8) {
    let file_path = "input.txt";
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    match day {
        1 => day1::run(contents),
        2 => day2::run(contents),
        3 => day3::run(contents),
        4 => day4::run(contents),
        5 => day5::run(contents),
        6 => day6::run(contents),
        7 => day7::run(contents),
        8 => day8::run(contents),
        9 => day9::run(contents),
        _ => println!("Day does not exist"),
    };
}
