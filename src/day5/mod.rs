pub fn one(input: &Vec<String>) {
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
    println!("{:?} <<>>> {:?}", seeds, maps);
}
