use std::time::Instant;

pub fn one(lines: &Vec<String>) -> i64 {
    compute(lines, 1).iter().sum()
}

pub fn two(lines: &Vec<String>) -> i64 {
    let now = Instant::now();
    let res = compute(lines, 1).iter().sum();
    let elapsed_time = now.elapsed();
    println!("Running took {:?} seconds.", elapsed_time.as_secs_f64());

    res
    // I tried inspecting differences between compute(1), compute(2) and compute(3),
    // and sometimes there was a clear multiplier, but sometimes the multiplier changed.
}

fn compute(lines: &Vec<String>, copies: usize) -> Vec<i64> {
    let mut springs: Vec<(Vec<char>, Vec<i64>)> = vec![];
    for line in lines {
        let l: Vec<&str> = line.split(" ").collect();
        if let [s, z] = l[..] {
            let c_rep: Vec<&str> = std::iter::repeat(s).take(copies).collect();
            let c: Vec<char> = c_rep.join("?").chars().collect();

            let n_rep: Vec<&str> = std::iter::repeat(z).take(copies).collect();
            let n: Vec<i64> = n_rep
                .join(",")
                .split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect();

            springs.push((c, n));
        }
    }

    let mut totals: Vec<i64> = vec![];
    for index in 0..springs.len() {
        let mut amount = 0;
        let spring = springs.get(index).unwrap();

        let mut min_space_needed = -1;
        for num in spring.1.iter() {
            min_space_needed += num + 1;
        }

        fill(
            spring.0.clone(),
            &spring.1,
            min_space_needed,
            0,
            &mut amount,
        );
        totals.push(amount);
        println!("Finished {}/999 with {}", index, amount);
    }
    totals
}

fn fill(spring: Vec<char>, map: &Vec<i64>, min_space_needed: i64, index: usize, total: &mut i64) {
    if let Some(c) = spring.get(index) {
        if *c == '?' {
            let mut c1 = spring.clone();
            let v1 = c1.get_mut(index).unwrap();
            *v1 = '#';
            if is_valid(&c1, map, min_space_needed) {
                fill(c1, map, min_space_needed, index + 1, total);
            }

            let mut c2 = spring.clone();
            let v2 = c2.get_mut(index).unwrap();
            *v2 = '.';
            if is_valid(&c2, map, min_space_needed) {
                fill(c2, map, min_space_needed, index + 1, total);
            }
        } else {
            fill(spring, map, min_space_needed, index + 1, total)
        }
    } else if is_valid(&spring, map, min_space_needed) {
        *total += 1;
    }
}

fn is_valid(spring: &Vec<char>, map: &Vec<i64>, min_space_needed: i64) -> bool {
    let mc: Vec<i64> = map.clone();
    let mut m: Vec<&i64> = mc.iter().rev().collect();

    // used in optimization
    let mut space_wasted = 0;

    let mut in_spring = false;
    let mut spring_len = 0;
    for c in spring {
        if *c == '?' {
            // we enter question mark area
            if let Some(ideal_value) = m.pop() {
                if spring_len > *ideal_value {
                    return false;
                }
            }

            if space_wasted > (spring.len() as i64) - min_space_needed {
                return false;
            }
            return true;
        }
        // not in question mark area
        else if *c == '#' {
            in_spring = true;
            spring_len += 1;
        } else if in_spring {
            // '.' but right next to spring
            if let Some(ideal_value) = m.pop() {
                if spring_len == *ideal_value {
                    in_spring = false;
                    spring_len = 0;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            // '.' which is 'useless'
            space_wasted += 1;
        }
    }

    if m.len() == 0 {
        return spring_len == 0;
    } else {
        let v = m.pop().unwrap();
        return m.len() == 0 && *v == spring_len;
    }
}
