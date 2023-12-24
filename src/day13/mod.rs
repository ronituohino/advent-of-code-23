use crate::Map;
use std::cmp::min;

pub fn one(lines: &Vec<String>) -> i32 {
    let mut maps: Vec<Map<char>> = vec![];

    let mut cur_map: Map<char> = Map::new(0, 0);
    let mut first_line_in_map = true;
    for line in lines {
        if line == "" {
            maps.push(cur_map);
            cur_map = Map::new(0, 0);
            first_line_in_map = true;
        } else {
            let c_line: Vec<char> = line.chars().collect();
            if first_line_in_map {
                cur_map.width = c_line.len();
                first_line_in_map = false;
            }
            cur_map.insert_row(c_line, None);
            cur_map.height += 1;
        }
    }
    maps.push(cur_map);

    let mut sum = 0;
    for map in maps {
        let (horizontal_mirror_space_indices, vertical_mirror_space_indices) =
            get_reflection_indices(&map);

        // finally count points
        let mut total = 0;
        let mut idx = 0;
        for b in horizontal_mirror_space_indices {
            if b {
                total += idx + 1;
            }
            idx += 1;
        }
        idx = 0;
        for b in vertical_mirror_space_indices {
            if b {
                total += (idx + 1) * 100;
            }
            idx += 1;
        }
        sum += total;
    }

    sum
}

fn get_reflection_indices(map: &Map<char>) -> (Vec<bool>, Vec<bool>) {
    let mut horizontal_mirror_space_indices = vec![true; map.width - 1];
    let mut vertical_mirror_space_indices = vec![true; map.height - 1];

    // horizontal mirroring
    for y in 0..map.height {
        // space between chars
        for s in 0..map.width - 1 {
            // letters on each side
            let mut left_indices = (0..s + 1).rev();
            let mut right_indices = (s + 1)..map.width;
            let mut mismatch = false;

            for _ in 0..min(left_indices.len(), right_indices.len()) {
                let l = map
                    .get(left_indices.next().unwrap() as i32, y as i32)
                    .unwrap();
                let r = map
                    .get(right_indices.next().unwrap() as i32, y as i32)
                    .unwrap();

                if *l != *r {
                    mismatch = true;
                    break;
                }
            }

            if mismatch {
                let val = horizontal_mirror_space_indices.get_mut(s).unwrap();
                *val = false;
            }
        }
    }

    // vertical mirroring
    for x in 0..map.width {
        // space between chars
        for s in 0..map.height - 1 {
            // letters on each side
            let mut top_indices = (0..s + 1).rev();
            let mut bottom_indices = (s + 1)..map.height;
            let mut mismatch = false;

            for _ in 0..min(top_indices.len(), bottom_indices.len()) {
                let t = map
                    .get(x as i32, top_indices.next().unwrap() as i32)
                    .unwrap();
                let b = map
                    .get(x as i32, bottom_indices.next().unwrap() as i32)
                    .unwrap();

                if *t != *b {
                    mismatch = true;
                    break;
                }
            }

            if mismatch {
                let val = vertical_mirror_space_indices.get_mut(s).unwrap();
                *val = false;
            }
        }
    }

    (
        horizontal_mirror_space_indices,
        vertical_mirror_space_indices,
    )
}

fn swap(map: &mut Map<char>, x: i32, y: i32) {
    let smudge = map.get(x, y).unwrap();
    if *smudge == '.' {
        let _ = map.set(x, y, '#');
    } else {
        let _ = map.set(x, y, '.');
    }
}

pub fn two(lines: &Vec<String>) -> i32 {
    let mut maps: Vec<Map<char>> = vec![];

    let mut cur_map: Map<char> = Map::new(0, 0);
    let mut first_line_in_map = true;
    for line in lines {
        if line == "" {
            maps.push(cur_map);
            cur_map = Map::new(0, 0);
            first_line_in_map = true;
        } else {
            let c_line: Vec<char> = line.chars().collect();
            if first_line_in_map {
                cur_map.width = c_line.len();
                first_line_in_map = false;
            }
            cur_map.insert_row(c_line, None);
            cur_map.height += 1;
        }
    }
    maps.push(cur_map);

    let mut sum = 0;
    for mut map in maps {
        let (base_horizontal_mirror_space_indices, base_vertical_mirror_space_indices) =
            get_reflection_indices(&map);

        let mut found_smudge = false;
        for s_y in 0..map.height {
            for s_x in 0..map.width {
                swap(&mut map, s_x as i32, s_y as i32);

                let (horizontal_mirror_space_indices, vertical_mirror_space_indices) =
                    get_reflection_indices(&map);

                // finally count points
                let mut total = 0;
                for i in 0..horizontal_mirror_space_indices.len() {
                    let hor = horizontal_mirror_space_indices.get(i).unwrap();
                    let base = base_horizontal_mirror_space_indices.get(i).unwrap();
                    if *hor {
                        if !*base {
                            found_smudge = true;
                            total += i + 1;
                        }
                    }
                }

                for i in 0..vertical_mirror_space_indices.len() {
                    let ver = vertical_mirror_space_indices.get(i).unwrap();
                    let base = base_vertical_mirror_space_indices.get(i).unwrap();
                    if *ver {
                        if !*base {
                            found_smudge = true;
                            total += (i + 1) * 100;
                        }
                    }
                }

                if found_smudge {
                    sum += total as i32;
                    break;
                } else {
                    swap(&mut map, s_x as i32, s_y as i32);
                }
            }
            if found_smudge {
                break;
            }
        }
    }

    sum
}
