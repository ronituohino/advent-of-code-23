pub mod day1;
pub mod day2;
pub mod day3;

use std::fs::read_to_string;
pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
