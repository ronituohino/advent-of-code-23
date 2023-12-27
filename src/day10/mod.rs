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

pub fn two(lines: &Vec<String>) -> i32 {
    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    let mut map: Map<char> = Map::new(width, height);
    let mut visited: Map<i32> = Map::new(width, height);
    let mut inner_map: Map<i32> = Map::new(width - 1, height - 1);

    let mut start_coords = (0, 0);
    for y in 0..height {
        let line = lines.get(y).unwrap();
        let chars: Vec<char> = line.chars().collect();
        map.insert_row(chars, None);
        visited.insert_row(vec![0; width], None);

        if y < height - 1 {
            inner_map.insert_row(vec![0; width - 1], None);
        }

        for x in 0..width {
            let c = &line[x..x + 1];
            if c == "S" {
                start_coords = (x, y);
            }
        }
    }

    // traverse through pipes and update pipe map
    traverse_two(
        &map,
        &mut visited,
        start_coords.0 as i32,
        start_coords.1 as i32,
    );

    // scan through pipe map, and update inner_map which tells if the spaces between pipe
    // segments are inside the loop or not
    for _ in 0..300 {
        for y in 0..map.height {
            for x in 0..map.width {
                scan(&map, &visited, &mut inner_map, x as i32, y as i32);
            }
            for x in (0..map.width).rev() {
                scan(&map, &visited, &mut inner_map, x as i32, y as i32);
            }
        }
        for x in 0..map.width {
            for y in 0..map.height {
                scan(&map, &visited, &mut inner_map, x as i32, y as i32);
            }
            for y in (0..map.height).rev() {
                scan(&map, &visited, &mut inner_map, x as i32, y as i32);
            }
        }
    }

    // count mass that is inside pipes
    let mut spots: Vec<(i32, i32)> = vec![];
    let mut mass = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let vis = visited.get(x as i32, y as i32).unwrap();
            if *vis == 0 {
                let mut found = false;
                if let Ok(se) = inner_map.get(x as i32, y as i32) {
                    if !found && *se == 2 {
                        mass += 1;
                        found = true;
                    }
                }
                if let Ok(ne) = inner_map.get(x as i32, (y - 1) as i32) {
                    if !found && *ne == 2 {
                        mass += 1;
                        found = true;
                    }
                }
                if let Ok(sw) = inner_map.get((x - 1) as i32, y as i32) {
                    if !found && *sw == 2 {
                        mass += 1;
                        found = true;
                    }
                }
                if let Ok(nw) = inner_map.get((x - 1) as i32, (y - 1) as i32) {
                    if !found && *nw == 2 {
                        mass += 1;
                        found = true;
                    }
                }
                if found {
                    spots.push((x as i32, y as i32));
                }
            }
        }
    }

    /*
    inner_map.pretty_print();
    println!("---");
    visited.pretty_print();
    println!("---");
    println!("{:?}", spots);
    */
    mass
}

fn scan(map: &Map<char>, visited: &Map<i32>, inner_map: &mut Map<i32>, x: i32, y: i32) {
    let is_pipe_loop = *(visited.get(x as i32, y as i32).unwrap()) == 1;
    if is_pipe_loop {
        let c = map.get(x as i32, y as i32).unwrap();
        if *c == '-' {
            check(
                inner_map,
                &vec![((x - 1) as i32, (y - 1) as i32), (x as i32, (y - 1) as i32)],
                &vec![((x - 1) as i32, y as i32), (x as i32, y as i32)],
            );
        } else if *c == '|' {
            check(
                inner_map,
                &vec![((x - 1) as i32, (y - 1) as i32), ((x - 1) as i32, y as i32)],
                &vec![(x as i32, y as i32), (x as i32, (y - 1) as i32)],
            );
        } else if *c == 'F' {
            check(
                inner_map,
                &vec![(x as i32, y as i32)],
                &vec![
                    ((x - 1) as i32, (y - 1) as i32),
                    (x as i32, (y - 1) as i32),
                    ((x - 1) as i32, y as i32),
                ],
            );
        } else if *c == 'L' {
            check(
                inner_map,
                &vec![(x as i32, (y - 1) as i32)],
                &vec![
                    ((x - 1) as i32, (y - 1) as i32),
                    ((x - 1) as i32, y as i32),
                    (x as i32, y as i32),
                ],
            );
        } else if *c == 'J' {
            check(
                inner_map,
                &vec![((x - 1) as i32, (y - 1) as i32)],
                &vec![
                    ((x - 1) as i32, y as i32),
                    (x as i32, y as i32),
                    (x as i32, (y - 1) as i32),
                ],
            );
        } else if *c == '7' {
            check(
                inner_map,
                &vec![
                    ((x - 1) as i32, (y - 1) as i32),
                    (x as i32, y as i32),
                    (x as i32, (y - 1) as i32),
                ],
                &vec![((x - 1) as i32, y as i32)],
            );
        } else if *c == 'S' {
        }
    } else {
        if let Ok(inner) = inner_map.get((x - 1) as i32, (y - 1) as i32) {
            if *inner != 0 {
                set_all_around_to_val(inner_map, x, y, *inner);
            }
        } else if let Ok(inner) = inner_map.get(x as i32, (y - 1) as i32) {
            if *inner != 0 {
                set_all_around_to_val(inner_map, x, y, *inner);
            }
        } else if let Ok(inner) = inner_map.get(x as i32, y as i32) {
            if *inner != 0 {
                set_all_around_to_val(inner_map, x, y, *inner);
            }
        } else if let Ok(inner) = inner_map.get((x - 1) as i32, y as i32) {
            if *inner != 0 {
                set_all_around_to_val(inner_map, x, y, *inner);
            }
        }
    }
}

fn set_all_around_to_val(inner_map: &mut Map<i32>, x: i32, y: i32, val: i32) {
    let _ = inner_map.set(x as i32, y as i32, val);
    let _ = inner_map.set((x - 1) as i32, y as i32, val);
    let _ = inner_map.set(x as i32, (y - 1) as i32, val);
    let _ = inner_map.set((x - 1) as i32, (y - 1) as i32, val);
}

fn check(inner_map: &mut Map<i32>, a: &Vec<(i32, i32)>, b: &Vec<(i32, i32)>) {
    let mut some_in_a_outside = false;
    let mut some_in_b_outside = false;

    for v in a {
        if is_outside(inner_map, v.0, v.1) {
            some_in_a_outside = true;
            break;
        }
    }
    if !some_in_a_outside {
        for v in b {
            if is_outside(inner_map, v.0, v.1) {
                some_in_b_outside = true;
                break;
            }
        }
    }

    if some_in_a_outside {
        for v in a {
            let _ = inner_map.set(v.0 as i32, v.1 as i32, 1);
        }
        for v in b {
            let _ = inner_map.set(v.0 as i32, v.1 as i32, 2);
        }
    } else if some_in_b_outside {
        for v in a {
            let _ = inner_map.set(v.0 as i32, v.1 as i32, 2);
        }
        for v in b {
            let _ = inner_map.set(v.0 as i32, v.1 as i32, 1);
        }
    }
}

fn is_outside(inner_map: &Map<i32>, x: i32, y: i32) -> bool {
    if let Ok(inner) = inner_map.get(x, y) {
        return *inner == 1;
    } else {
        true
    }
}

fn traverse_two(map: &Map<char>, visited: &mut Map<i32>, x: i32, y: i32) {
    if let Ok(num) = visited.get(x, y) {
        // only traverse around if this hasn't been processed
        if *num == 0 {
            let _ = visited.set(x, y, 1);
            if let Ok(this) = map.get(x, y) {
                if connects_right(*this) {
                    if let Ok(other) = map.get(x + 1, y) {
                        if connects_left(*other) {
                            traverse_two(map, visited, x + 1, y);
                        }
                    }
                }

                if connects_left(*this) {
                    if let Ok(other) = map.get(x - 1, y) {
                        if connects_right(*other) {
                            traverse_two(map, visited, x - 1, y);
                        }
                    }
                }

                if connects_up(*this) {
                    if let Ok(other) = map.get(x, y - 1) {
                        if connects_down(*other) {
                            traverse_two(map, visited, x, y - 1);
                        }
                    }
                }

                if connects_down(*this) {
                    if let Ok(other) = map.get(x, y + 1) {
                        if connects_up(*other) {
                            traverse_two(map, visited, x, y + 1);
                        }
                    }
                }
            }
        }
    }
}
