use aoc::day17::{one, two};

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day17/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", one(&lines));
}
