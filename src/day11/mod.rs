use crate::Map;

pub fn one(lines: &Vec<String>) -> usize {
    let height = lines.len();
    let width = lines.get(0).unwrap().len();
    let mut map: Map<char> = Map::new(width, height);

    // create map
    let mut expanded_row_indices = vec![];
    for y in 0..height {
        let line = lines.get(y).unwrap();
        let chars: Vec<char> = line.chars().collect();
        map.insert_row(chars, None);

        // vertical expansion
        if !line.contains("#") {
            expanded_row_indices.push(y);
        }
    }

    // horizontal expansion
    let mut expanded_column_indices = vec![];
    for x in 0..width {
        let mut found = false;
        for y in 0..height {
            if *map.get(x as i32, y as i32).unwrap() == '#' {
                found = true;
                break;
            }
        }
        if !found {
            expanded_column_indices.push(x);
        }
    }

    // apply expansion to map
    let multiplier = 1;

    let new_width = width + (expanded_column_indices.len() * multiplier);
    let empty_row: Vec<char> = (0..new_width).map(|_| '.').collect();
    let new_height = height + (expanded_row_indices.len() * multiplier);
    let empty_column: Vec<char> = (0..new_height).map(|_| '.').collect();

    let mut c = 0;
    for index in expanded_column_indices {
        for _ in 0..multiplier {
            map.insert_column(empty_column.clone(), Some(index + c));
            map.width += 1;
            c += 1;
        }
    }

    let mut r = 0;
    for index in expanded_row_indices {
        for _ in 0..multiplier {
            map.insert_row(empty_row.clone(), Some(index + r));
            map.height += 1;
            r += 1;
        }
    }

    // map out stars
    let mut stars: Vec<(usize, usize)> = vec![];
    for y in 0..new_height {
        for x in 0..new_width {
            if *map.get(x as i32, y as i32).unwrap() == '#' {
                stars.push((x, y));
            }
        }
    }

    // calculate final distance sum
    let mut dist_sum = 0;
    for i in 0..stars.len() {
        for j in i..stars.len() {
            let s1 = *stars.get(i).unwrap();
            let s2 = *stars.get(j).unwrap();

            let dist = s1.0.abs_diff(s2.0) + s1.1.abs_diff(s2.1);
            dist_sum += dist;
        }
    }

    dist_sum
}

pub fn two(lines: &Vec<String>) -> usize {
    let height = lines.len();
    let width = lines.get(0).unwrap().len();
    let mut map: Map<char> = Map::new(width, height);

    // create map
    let mut expanded_row_indices = vec![];
    for y in 0..height {
        let line = lines.get(y).unwrap();
        let chars: Vec<char> = line.chars().collect();
        map.insert_row(chars, None);

        // vertical expansion
        if !line.contains("#") {
            expanded_row_indices.push(y);
        }
    }

    // horizontal expansion
    let mut expanded_column_indices = vec![];
    for x in 0..width {
        let mut found = false;
        for y in 0..height {
            if *map.get(x as i32, y as i32).unwrap() == '#' {
                found = true;
                break;
            }
        }
        if !found {
            expanded_column_indices.push(x);
        }
    }

    // map out stars
    let mut stars: Vec<(usize, usize)> = vec![];
    for y in 0..height {
        for x in 0..width {
            if *map.get(x as i32, y as i32).unwrap() == '#' {
                stars.push((x, y));
            }
        }
    }

    // calculate final distance sum
    // note that multiplier needs to negate one,
    // because the map alredy has 1 space between stars
    let multiplier = 1_000_000 - 1;
    let mut dist_sum = 0;
    for i in 0..stars.len() - 1 {
        for j in i + 1..stars.len() {
            let s1 = *stars.get(i).unwrap();
            let s2 = *stars.get(j).unwrap();

            let mut dist = s1.0.abs_diff(s2.0) + s1.1.abs_diff(s2.1);
            // take expansion into account
            dist += multiplier
                * expanded_column_indices
                    .iter()
                    .filter(|x| {
                        if s1.0 < s2.0 {
                            **x > s1.0 && **x < s2.0
                        } else {
                            **x < s1.0 && **x > s2.0
                        }
                    })
                    .count();
            dist += multiplier
                * expanded_row_indices
                    .iter()
                    .filter(|y| {
                        if s1.1 < s2.1 {
                            **y > s1.1 && **y < s2.1
                        } else {
                            **y < s1.1 && **y > s2.1
                        }
                    })
                    .count();

            dist_sum += dist;
        }
    }

    dist_sum
}
