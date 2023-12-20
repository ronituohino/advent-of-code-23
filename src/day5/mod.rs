pub fn one(input: &Vec<String>) -> i64 {
    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<Vec<(i64, i64, i64)>> = vec![];

    // read input
    let mut mode = "seeds";
    let mut map: Vec<(i64, i64, i64)> = vec![];
    for line in input {
        if mode == "seeds" {
            // seeds
            let l: Vec<&str> = line.split(":").map(|x| x.trim()).collect();
            if let [.., s] = l[..] {
                seeds = s
                    .split(" ")
                    .map(|x| x.trim().parse::<i64>().unwrap())
                    .collect();
            }
            mode = "idle";
            continue;
        } else if line.contains("map") {
            map.clear();
            mode = "mapping";
        } else if mode == "mapping" {
            if line == "" {
                maps.push(map.clone());
                mode = "idle"
            } else {
                let l: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
                if let [dest, source, range] = l[..] {
                    map.push((dest, source, range));
                }
            }
        }
    }
    // push final map, because input doesn't have empty line at end
    maps.push(map.clone());

    // process seeds through all maps
    let mut final_locations: Vec<i64> = vec![];
    for seed in seeds {
        let mut cur_val = seed;
        for map in &maps {
            for range in map {
                if cur_val >= range.1 && cur_val < range.1 + range.2 {
                    let offset = cur_val - range.1;
                    cur_val = range.0 + offset;
                    break;
                }
            }
        }
        final_locations.push(cur_val);
    }

    *final_locations.iter().min().unwrap()
}

pub fn two(input: &Vec<String>) -> i64 {
    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<Vec<(i64, i64, i64)>> = vec![];

    // read input
    let mut mode = "seeds";
    let mut map: Vec<(i64, i64, i64)> = vec![];
    for line in input {
        if mode == "seeds" {
            // seeds
            let l: Vec<&str> = line.split(":").map(|x| x.trim()).collect();
            if let [.., s] = l[..] {
                seeds = s
                    .split(" ")
                    .map(|x| x.trim().parse::<i64>().unwrap())
                    .collect();
            }
            mode = "idle";
            continue;
        } else if line.contains("map") {
            map.clear();
            mode = "mapping";
        } else if mode == "mapping" {
            if line == "" {
                maps.push(map.clone());
                mode = "idle"
            } else {
                let l: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
                if let [dest, source, range] = l[..] {
                    map.push((dest, source, range));
                }
            }
        }
    }
    // push final map, because input doesn't have empty line at end
    maps.push(map.clone());

    // get seed vals
    let mut seed_pairs: Vec<(i64, i64)> = vec![];
    let mut seed_iter = seeds.iter();
    loop {
        if let Some(a) = seed_iter.next() {
            if let Some(b) = seed_iter.next() {
                seed_pairs.push((*a, *b));
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // process seeds through all maps
    let mut final_loc = i64::MAX;
    for pair in seed_pairs {
        for seed in pair.0..(pair.0 + pair.1) {
            let mut cur_val = seed;
            for map in &maps {
                for range in map {
                    if cur_val >= range.1 && cur_val < range.1 + range.2 {
                        let offset = cur_val - range.1;
                        cur_val = range.0 + offset;
                        break;
                    }
                }
            }
            if cur_val < final_loc {
                final_loc = cur_val;
            }
        }
    }

    final_loc
}
