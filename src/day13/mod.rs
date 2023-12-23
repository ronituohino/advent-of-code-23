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

pub fn two(lines: &Vec<String>) {}
