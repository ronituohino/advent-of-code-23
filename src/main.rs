use aoc::day2::two;

use aoc::read_lines;
const INPUT_PATH: &str = "./src/day2/input.txt";

fn main() {
    let lines = read_lines(INPUT_PATH);
    println!("{:?}", two(&lines));
}
