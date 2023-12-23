pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use std::fs::read_to_string;
pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

#[derive(Debug)]
pub struct Map<T> {
    pub width: usize,
    pub height: usize,
    pub rows: Vec<Vec<T>>,
}

impl<T: Clone> Map<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            rows: vec![],
        }
    }

    pub fn insert_row(&mut self, values: Vec<T>, pos: Option<usize>) {
        match pos {
            Some(p) => self.rows.insert(p, values),
            None => self.rows.push(values),
        }
    }

    /// Slow, prefer insert_horizontal instead
    pub fn insert_column(&mut self, values: Vec<T>, pos: Option<usize>) {
        for i in 0..self.height {
            let row = self.rows.get_mut(i).unwrap();
            let val = values.get(i).unwrap();
            match pos {
                Some(p) => row.insert(p, (*val).clone()),
                None => row.push((*val).clone()),
            }
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Result<&T, ()> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Ok(self.rows.get(y as usize).unwrap().get(x as usize).unwrap())
        } else {
            Err(())
        }
    }

    pub fn set(&mut self, x: i32, y: i32, val: T) -> Result<(), ()> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let el = self
                .rows
                .get_mut(y as usize)
                .unwrap()
                .get_mut(x as usize)
                .unwrap();

            let _ = std::mem::replace(el, val);
            Ok(())
        } else {
            Err(())
        }
    }
}
