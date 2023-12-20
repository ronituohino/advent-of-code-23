use aoc::day4::two;

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day4/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", two(&lines));
}
