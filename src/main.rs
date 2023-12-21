use aoc::day8::two;

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day8/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", two(&lines));
}
