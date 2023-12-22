use crate::Map;

pub fn one(lines: &Vec<String>) -> i32 {
    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    let mut map: Map<char> = Map::new(width, height);
    let mut dist_map: Map<i32> = Map::new(width, height);
    let mut start_coords = (0, 0);

    for y in 0..height {
        let line = lines.get(y).unwrap();
        let chars: Vec<char> = line.chars().collect();
        map.insert_row(chars, None);
        dist_map.insert_row(vec![-1; width], None);

        for x in 0..width {
            let c = &line[x..x + 1];
            if c == "S" {
                start_coords = (x, y);
            }
        }
    }

    // traverse through pipes and update distance map
    traverse_one(
        &map,
        &mut dist_map,
        start_coords.0 as i32,
        start_coords.1 as i32,
        0,
    );

    // find largest distance
    let mut max = 0;
    for y in 0..height {
        for x in 0..width {
            if let Ok(d) = dist_map.get(x as i32, y as i32) {
                if *d > max {
                    max = *d;
                }
            }
        }
    }

    println!("{:?}", dist_map);
    max
}

fn connects_down(c: char) -> bool {
    c == 'S' || c == '|' || c == '7' || c == 'F'
}
fn connects_up(c: char) -> bool {
    c == 'S' || c == '|' || c == 'J' || c == 'L'
}
fn connects_right(c: char) -> bool {
    c == 'S' || c == '-' || c == 'L' || c == 'F'
}
fn connects_left(c: char) -> bool {
    c == 'S' || c == '-' || c == '7' || c == 'J'
}

fn traverse_one(map: &Map<char>, dist_map: &mut Map<i32>, x: i32, y: i32, dist: i32) {
    if let Ok(num) = dist_map.get(x, y) {
        // only traverse around if this hasn't been processed, or this dist is smaller
        if *num == -1 || dist < *num {
            let _ = dist_map.set(x, y, dist);
            if let Ok(this) = map.get(x, y) {
                if connects_right(*this) {
                    if let Ok(other) = map.get(x + 1, y) {
                        if connects_left(*other) {
                            traverse_one(map, dist_map, x + 1, y, dist + 1);
                        }
                    }
                }

                if connects_left(*this) {
                    if let Ok(other) = map.get(x - 1, y) {
                        if connects_right(*other) {
                            traverse_one(map, dist_map, x - 1, y, dist + 1);
                        }
                    }
                }

                if connects_up(*this) {
                    if let Ok(other) = map.get(x, y - 1) {
                        if connects_down(*other) {
                            traverse_one(map, dist_map, x, y - 1, dist + 1);
                        }
                    }
                }

                if connects_down(*this) {
                    if let Ok(other) = map.get(x, y + 1) {
                        if connects_up(*other) {
                            traverse_one(map, dist_map, x, y + 1, dist + 1);
                        }
                    }
                }
            }
        }
    }
}

/// Incomplete, maybe we have to detect already visited pipes when traversing new ones and
/// based on some rules, and mark areas?
pub fn two(lines: &Vec<String>) -> i32 {
    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    let mut map: Map<char> = Map::new(width, height);
    let mut visit_map: Map<i32> = Map::new(width, height);
    let mut start_coords = (0, 0);

    for y in 0..height {
        let line = lines.get(y).unwrap();
        let chars: Vec<char> = line.chars().collect();
        map.insert_row(chars, None);
        visit_map.insert_row(vec![0; width], None);

        for x in 0..width {
            let c = &line[x..x + 1];
            if c == "S" {
                start_coords = (x, y);
            }
        }
    }

    // traverse through pipes and update distance map
    traverse_two(
        &map,
        &mut visit_map,
        start_coords.0 as i32,
        start_coords.1 as i32,
    );

    // find largest distance
    let mut max = 0;
    for y in 0..height {
        for x in 0..width {
            if let Ok(d) = visit_map.get(x as i32, y as i32) {
                if *d > max {
                    max = *d;
                }
            }
        }
    }

    println!("{:?}", visit_map);
    max
}

fn traverse_two(map: &Map<char>, visit_map: &mut Map<i32>, x: i32, y: i32) {
    if let Ok(num) = visit_map.get(x, y) {
        // only traverse around if this hasn't been processed
        if *num == 0 {
            let _ = visit_map.set(x, y, 1);
            if let Ok(this) = map.get(x, y) {
                if connects_right(*this) {
                    if let Ok(other) = map.get(x + 1, y) {
                        if connects_left(*other) {
                            traverse_two(map, visit_map, x + 1, y);
                        }
                    }
                }

                if connects_left(*this) {
                    if let Ok(other) = map.get(x - 1, y) {
                        if connects_right(*other) {
                            traverse_two(map, visit_map, x - 1, y);
                        }
                    }
                }

                if connects_up(*this) {
                    if let Ok(other) = map.get(x, y - 1) {
                        if connects_down(*other) {
                            traverse_two(map, visit_map, x, y - 1);
                        }
                    }
                }

                if connects_down(*this) {
                    if let Ok(other) = map.get(x, y + 1) {
                        if connects_up(*other) {
                            traverse_two(map, visit_map, x, y + 1);
                        }
                    }
                }
            }
        }
    }
}
