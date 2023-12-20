use aoc::day7::one;

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day7/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", one(&lines));
}
