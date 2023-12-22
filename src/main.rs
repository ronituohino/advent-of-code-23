use aoc::day11::two;

use aoc::{read_lines, Map};
const INPUT_PATH: &str = "./src/day11/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", two(&lines));
}
