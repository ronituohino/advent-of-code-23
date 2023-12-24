use std::collections::HashSet;

use crate::Map;

fn init(lines: &Vec<String>) -> Map<char> {
    let height = lines.len();
    let width = lines.get(0).unwrap().len();
    let mut map: Map<char> = Map::new(width, height);
    for line in lines {
        map.insert_row(line.chars().collect(), None);
    }
    map
}

pub fn one(lines: &Vec<String>) -> usize {
    let map = init(lines);
    let mut visited_tiles: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_states: HashSet<(i32, i32, Direction)> = HashSet::new();
    traverse(
        &map,
        &mut visited_tiles,
        &mut visited_states,
        0,
        0,
        Direction::East,
    );
    visited_tiles.len()
}

pub fn two(lines: &Vec<String>) -> usize {
    let map = init(lines);

    let mut max_energized = 0;
    for i in 0..2 {
        let dir = if i == 0 {
            Direction::South
        } else {
            Direction::North
        };
        let y = if i == 0 { 0 } else { map.height };
        for x in 0..map.width {
            let mut visited_tiles: HashSet<(i32, i32)> = HashSet::new();
            let mut visited_states: HashSet<(i32, i32, Direction)> = HashSet::new();
            traverse(
                &map,
                &mut visited_tiles,
                &mut visited_states,
                x as i32,
                y as i32,
                dir,
            );
            let total = visited_tiles.len();
            if total > max_energized {
                max_energized = total;
            }
        }
    }
    for i in 0..2 {
        let dir = if i == 0 {
            Direction::East
        } else {
            Direction::West
        };
        let x = if i == 0 { 0 } else { map.width };
        for y in 0..map.height {
            let mut visited_tiles: HashSet<(i32, i32)> = HashSet::new();
            let mut visited_states: HashSet<(i32, i32, Direction)> = HashSet::new();
            traverse(
                &map,
                &mut visited_tiles,
                &mut visited_states,
                x as i32,
                y as i32,
                dir,
            );
            let total = visited_tiles.len();
            if total > max_energized {
                max_energized = total;
            }
        }
    }

    max_energized
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn get_new_coords(x: i32, y: i32, dir: Direction) -> (i32, i32) {
    match dir {
        Direction::North => (x, y - 1),
        Direction::East => (x + 1, y),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
    }
}

fn traverse(
    map: &Map<char>,
    visited_tiles: &mut HashSet<(i32, i32)>,
    visited_states: &mut HashSet<(i32, i32, Direction)>,
    x: i32,
    y: i32,
    dir: Direction,
) {
    if let Ok(c) = map.get(x, y) {
        visited_tiles.insert((x, y));
        let new_state = visited_states.insert((x, y, dir));
        if !new_state {
            return;
        }
        if *c == '.' {
            let new_coords = get_new_coords(x, y, dir);
            traverse(
                map,
                visited_tiles,
                visited_states,
                new_coords.0,
                new_coords.1,
                dir,
            );
        } else if *c == '-' {
            if dir == Direction::East || dir == Direction::West {
                let new_coords = get_new_coords(x, y, dir);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords.0,
                    new_coords.1,
                    dir,
                );
            } else {
                let new_coords_west = get_new_coords(x, y, Direction::West);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_west.0,
                    new_coords_west.1,
                    Direction::West,
                );
                let new_coords_east = get_new_coords(x, y, Direction::East);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_east.0,
                    new_coords_east.1,
                    Direction::East,
                );
            }
        } else if *c == '|' {
            if dir == Direction::North || dir == Direction::South {
                let new_coords = get_new_coords(x, y, dir);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords.0,
                    new_coords.1,
                    dir,
                );
            } else {
                let new_coords_north = get_new_coords(x, y, Direction::North);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_north.0,
                    new_coords_north.1,
                    Direction::North,
                );
                let new_coords_south = get_new_coords(x, y, Direction::South);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_south.0,
                    new_coords_south.1,
                    Direction::South,
                );
            }
        } else if *c == '/' {
            if dir == Direction::East {
                let new_coords_north = get_new_coords(x, y, Direction::North);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_north.0,
                    new_coords_north.1,
                    Direction::North,
                );
            } else if dir == Direction::South {
                let new_coords_west = get_new_coords(x, y, Direction::West);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_west.0,
                    new_coords_west.1,
                    Direction::West,
                );
            } else if dir == Direction::North {
                let new_coords_east = get_new_coords(x, y, Direction::East);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_east.0,
                    new_coords_east.1,
                    Direction::East,
                );
            } else {
                // dir == Direction::West
                let new_coords_south = get_new_coords(x, y, Direction::South);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_south.0,
                    new_coords_south.1,
                    Direction::South,
                );
            }
        } else {
            // '\'
            if dir == Direction::East {
                let new_coords_south = get_new_coords(x, y, Direction::South);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_south.0,
                    new_coords_south.1,
                    Direction::South,
                );
            } else if dir == Direction::South {
                let new_coords_east = get_new_coords(x, y, Direction::East);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_east.0,
                    new_coords_east.1,
                    Direction::East,
                );
            } else if dir == Direction::North {
                let new_coords_west = get_new_coords(x, y, Direction::West);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_west.0,
                    new_coords_west.1,
                    Direction::West,
                );
            } else {
                // West
                let new_coords_north = get_new_coords(x, y, Direction::North);
                traverse(
                    map,
                    visited_tiles,
                    visited_states,
                    new_coords_north.0,
                    new_coords_north.1,
                    Direction::North,
                );
            }
        }
    }
}
