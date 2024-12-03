use std::fs::File;
use std::io::Read;

mod day1;
mod day2;
mod day3;

pub fn run_day(day: u8) {
    let file_path = "input.txt";
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    match day {
        1 => day1::run(contents),
        2 => day2::run(contents),
        3 => day3::run(contents),
        _ => println!("Day does not exist"),
    };
}
