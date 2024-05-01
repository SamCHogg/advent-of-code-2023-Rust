use std::fs;
use std::io::{BufRead, BufReader, Error};
use std::string::String;

fn process_line(line: String) -> u32 {
    let mut first = 0;
    for c in line.chars() {
        if c.is_numeric() {
            first = c.to_digit(10).unwrap();
            break;
        }
    }

    let mut last = 0;
    for c in line.chars().rev() {
        if c.is_numeric() {
            last = c.to_digit(10).unwrap();
            break;
        }
    }

    (first * 10) + last
}

fn main() -> Result<(), Error> {
    let mut total = 0;
    let buff_reader = BufReader::new(fs::File::open("./input.txt")?);
    for line in buff_reader.lines() {
        total += process_line(line?);
    }
    println!("{}", total);
    Ok(())
}
