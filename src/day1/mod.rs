use crate::read_lines;

pub fn solve(filename: &str) {
    let lines = read_lines(filename);
    for line in lines {
        println!("{}", line);
    }
}
