use aoc::day10::{one, two};

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day10/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", two(&lines));
}
