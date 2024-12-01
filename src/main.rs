use std::io::{self, Write};

use days::run_day;

mod days;

fn main() {
    print!("Enter your day: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let day = input.trim().parse::<u8>().unwrap();
    run_day(day);
}
