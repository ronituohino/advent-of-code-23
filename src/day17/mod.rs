use crate::Map;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{Debug, Display};

#[derive(Clone)]
struct MapKey(u32, Direction, u8);
impl Debug for MapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn one(lines: &Vec<String>) -> u32 {
    let width = lines.get(0).unwrap().len();
    let height = lines.len();

    let mut map: Map<u32> = Map::new(width, height);
    let mut lowest_heat_map: Map<Vec<MapKey>> = Map::new(width, height);
    let default_map_key = vec![];

    for line in lines {
        map.insert_row(
            line.chars().map(|c| c.to_digit(10).unwrap()).collect(),
            None,
        );
        lowest_heat_map.insert_row(vec![default_map_key.clone(); width], None);
    }

    search_path(&map, 0, 0, Direction::North, 0, 0, &mut lowest_heat_map);
    lowest_heat_map
        .get((map.width - 1) as i32, (map.height - 1) as i32)
        .unwrap()
        .iter()
        .min_by_key(|f| f.0)
        .unwrap()
        .0
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

const MAX_MOVES_IN_SAME_DIR: u8 = 3;
fn search_path(
    map: &Map<u32>,
    x: i32,
    y: i32,
    dir: Direction,
    times_moved_in_same_dir: u8,
    total_heat_loss: u32,
    lowest_heat_map: &mut Map<Vec<MapKey>>,
) {
    let visits = lowest_heat_map.get(x, y).unwrap();
    if visits.len() > 0 && (visits.iter().any(|v| v.0 < total_heat_loss)) {
        return;
    }

    let mut new = visits.clone();
    new.push(MapKey(total_heat_loss, dir, times_moved_in_same_dir));
    let _ = lowest_heat_map.set(x, y, new);

    // north
    if let Ok(n) = map.get(x, y - 1) {
        let times = if dir == Direction::North {
            times_moved_in_same_dir + 1
        } else {
            1
        };

        if times <= MAX_MOVES_IN_SAME_DIR && dir != Direction::South {
            search_path(
                map,
                x,
                y - 1,
                Direction::North,
                times,
                total_heat_loss + n,
                lowest_heat_map,
            );
        }
    }
    // east
    if let Ok(n) = map.get(x + 1, y) {
        let times = if dir == Direction::East {
            times_moved_in_same_dir + 1
        } else {
            1
        };

        if times <= MAX_MOVES_IN_SAME_DIR && dir != Direction::West {
            search_path(
                map,
                x + 1,
                y,
                Direction::East,
                times,
                total_heat_loss + n,
                lowest_heat_map,
            );
        }
    }
    // south
    if let Ok(n) = map.get(x, y + 1) {
        let times = if dir == Direction::South {
            times_moved_in_same_dir + 1
        } else {
            1
        };

        if times <= MAX_MOVES_IN_SAME_DIR && dir != Direction::North {
            search_path(
                map,
                x,
                y + 1,
                Direction::South,
                times,
                total_heat_loss + n,
                lowest_heat_map,
            );
        }
    }
    // west
    if let Ok(n) = map.get(x - 1, y) {
        let times = if dir == Direction::West {
            times_moved_in_same_dir + 1
        } else {
            1
        };

        if times <= MAX_MOVES_IN_SAME_DIR && dir != Direction::East {
            search_path(
                map,
                x - 1,
                y,
                Direction::West,
                times,
                total_heat_loss + n,
                lowest_heat_map,
            );
        }
    }
}

pub fn two(lines: &Vec<String>) {}
