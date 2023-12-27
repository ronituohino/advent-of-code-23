use aoc::day12::{one, two};

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day12/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", two(&lines));
}
