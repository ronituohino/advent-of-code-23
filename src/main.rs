use aoc::day1::{one, parse_text_num, two};

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day1/input.txt";

fn main() {
    let mut lines = read_lines(INPUT_PATH);
    println!("{}", one(&mut lines));
}
