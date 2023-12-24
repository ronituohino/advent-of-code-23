use crate::Map;

pub fn one(lines: &Vec<String>) -> i32 {
    let mut map = init(lines);
    // roll round rocks north
    roll_north(&mut map);
    // calculate weight
    calc_total(&map)
}

/// Doesn't print out correct result straight up.
/// Prints out a sequence from which you have to find the 'cycle'
/// which repeats until 1_000_000_000.
/// Find the index at which the cycle starts and the length of the cycle
/// to get the correct result, like in cycle.txt.
pub fn two(lines: &Vec<String>) {
    let mut map = init(lines);
    // roll round rocks north
    for i in 0..1_000 {
        roll_north(&mut map);
        roll_west(&mut map);
        roll_south(&mut map);
        roll_east(&mut map);
        // calculate weight
        let w = calc_total(&map);
        println!("{}", w);
        /*
        if i > 100 && w == 100821 {
            println!("Found: {} at index {}", w, i);
            break;
        }
        */
    }
}

fn init(lines: &Vec<String>) -> Map<char> {
    let height = lines.len();
    let width = lines.get(0).unwrap().len();
    let mut map: Map<char> = Map::new(width, height);
    for line in lines {
        map.insert_row(line.chars().collect(), None);
    }
    map
}

fn roll_north(map: &mut Map<char>) {
    for y in 1..map.height {
        for x in 0..map.width {
            let c = map.get(x as i32, y as i32).unwrap();
            if *c == 'O' {
                let mut most_north_point: i32 = -1;
                for i in (0..y).rev() {
                    let b = map.get(x as i32, i as i32).unwrap();
                    if *b == '.' {
                        most_north_point = i as i32;
                    } else {
                        break;
                    }
                }

                if most_north_point != -1 {
                    // remove old spot
                    let _ = map.set(x as i32, y as i32, '.');
                    // set rock to norther most point
                    let _ = map.set(x as i32, most_north_point as i32, 'O');
                }
            }
        }
    }
}

fn roll_south(map: &mut Map<char>) {
    for y in (0..map.height - 1).rev() {
        for x in 0..map.width {
            let c = map.get(x as i32, y as i32).unwrap();
            if *c == 'O' {
                let mut most_south_point: i32 = -1;
                for i in (y + 1)..map.height {
                    let b = map.get(x as i32, i as i32).unwrap();
                    if *b == '.' {
                        most_south_point = i as i32;
                    } else {
                        break;
                    }
                }

                if most_south_point != -1 {
                    // remove old spot
                    let _ = map.set(x as i32, y as i32, '.');
                    // set rock to norther most point
                    let _ = map.set(x as i32, most_south_point as i32, 'O');
                }
            }
        }
    }
}

fn roll_west(map: &mut Map<char>) {
    for x in 1..map.width {
        for y in 0..map.height {
            let c = map.get(x as i32, y as i32).unwrap();
            if *c == 'O' {
                let mut most_west_point: i32 = -1;
                for i in (0..x).rev() {
                    let b = map.get(i as i32, y as i32).unwrap();
                    if *b == '.' {
                        most_west_point = i as i32;
                    } else {
                        break;
                    }
                }

                if most_west_point != -1 {
                    // remove old spot
                    let _ = map.set(x as i32, y as i32, '.');
                    // set rock to norther most point
                    let _ = map.set(most_west_point as i32, y as i32, 'O');
                }
            }
        }
    }
}

fn roll_east(map: &mut Map<char>) {
    for x in (0..map.width - 1).rev() {
        for y in 0..map.height {
            let c = map.get(x as i32, y as i32).unwrap();
            if *c == 'O' {
                let mut most_east_point: i32 = -1;
                for i in (x + 1)..map.width {
                    let b = map.get(i as i32, y as i32).unwrap();
                    if *b == '.' {
                        most_east_point = i as i32;
                    } else {
                        break;
                    }
                }

                if most_east_point != -1 {
                    // remove old spot
                    let _ = map.set(x as i32, y as i32, '.');
                    // set rock to norther most point
                    let _ = map.set(most_east_point as i32, y as i32, 'O');
                }
            }
        }
    }
}

fn calc_total(map: &Map<char>) -> i32 {
    let mut total = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let c = map.get(x as i32, y as i32).unwrap();
            if *c == 'O' {
                total += map.height as i32 - y as i32;
            }
        }
    }
    total
}
