use aoc::day15::{one, two};

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day15/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", two(&lines));
}
