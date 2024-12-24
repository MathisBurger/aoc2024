use std::fs::File;
use std::io::Read;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
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
        10 => day10::run(contents),
        11 => day11::run(contents),
        12 => day12::run(contents),
        13 => day13::run(contents),
        14 => day14::run(contents),
        15 => day15::run(contents),
        16 => day16::run(contents),
        17 => day17::run(contents),
        18 => day18::run(contents),
        19 => day19::run(contents),
        20 => day20::run(contents),
        21 => day21::run(contents),
        22 => day22::run(contents),
        23 => day23::run(contents),
        24 => day24::run(contents),
        _ => println!("Day does not exist"),
    };
}
