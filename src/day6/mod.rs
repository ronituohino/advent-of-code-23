pub fn one(lines: &Vec<String>) -> i64 {
    let mut iter = lines.iter();
    let t: Vec<&str> = iter.next().unwrap().split(":").collect();
    let times: Vec<i64> = t[1]
        .split(" ")
        .map(|x| x.trim())
        .filter(|x| *x != "")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let d: Vec<&str> = iter.next().unwrap().split(":").collect();
    let distances: Vec<i64> = d[1]
        .split(" ")
        .map(|x| x.trim())
        .filter(|x| *x != "")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut winning = vec![];
    for i in 0..times.len() {
        let time = times[i];
        let dist = distances[i];

        let mut min = 0;
        for min_hold in 1..time {
            let remaining_time = time - min_hold;
            if remaining_time * min_hold > dist {
                min = min_hold;
                break;
            }
        }

        let mut max = 0;
        for max_hold in (1..time).rev() {
            let remaining_time = time - max_hold;
            if remaining_time * max_hold > dist {
                max = max_hold;
                break;
            }
        }

        let winning_ways = max - min + 1;
        winning.push(winning_ways);
    }

    winning.iter().product()
}

pub fn two(lines: &Vec<String>) -> i64 {
    let mut iter = lines.iter();
    let t: Vec<&str> = iter.next().unwrap().split(":").collect();
    let time: i64 = t[1].replace(" ", "").parse::<i64>().unwrap();
    let d: Vec<&str> = iter.next().unwrap().split(":").collect();
    let dist: i64 = d[1].replace(" ", "").parse::<i64>().unwrap();

    let mut min = 0;
    for min_hold in 1..time {
        let remaining_time = time - min_hold;
        if remaining_time * min_hold > dist {
            min = min_hold;
            break;
        }
    }

    let mut max = 0;
    for max_hold in (1..time).rev() {
        let remaining_time = time - max_hold;
        if remaining_time * max_hold > dist {
            max = max_hold;
            break;
        }
    }

    let winning_ways = max - min + 1;
    winning_ways
}
