pub fn one(games: &Vec<String>) -> i32 {
    let mut sum = 0;

    for game in games {
        let mut points = 0;

        let a: Vec<&str> = game.split(":").map(|x| x.trim()).collect();
        if let [.., nums] = a[..] {
            let b: Vec<&str> = nums.split("|").map(|n| n.trim()).collect();
            if let [w, c] = b[..] {
                let mut winning: Vec<usize> = w
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                winning.sort();
                let mut chosen: Vec<usize> = c
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                chosen.sort();

                let mut i1 = 0;
                let mut i2 = 0;
                while i1 < chosen.len() && i2 < winning.len() {
                    let choose = chosen[i1];
                    let win = winning[i2];

                    if choose == win {
                        points += 1;
                        i1 += 1;
                        i2 += 1;
                    } else if choose < win {
                        i1 += 1;
                    } else {
                        i2 += 1;
                    }
                }
            }
        }

        if points > 0 {
            let result: i32 = 2;
            sum += result.pow(points - 1);
        }
    }

    sum
}

pub fn two(games: &Vec<String>) -> i32 {
    let mut instances: Vec<i32> = vec![];
    for _ in 0..games.len() {
        instances.push(1);
    }

    let mut idx = 0;
    for game in games {
        let mut wins = 0;

        let a: Vec<&str> = game.split(":").map(|x| x.trim()).collect();
        if let [.., nums] = a[..] {
            let b: Vec<&str> = nums.split("|").map(|n| n.trim()).collect();
            if let [w, c] = b[..] {
                let mut winning: Vec<usize> = w
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                winning.sort();
                let mut chosen: Vec<usize> = c
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                chosen.sort();

                let mut i1 = 0;
                let mut i2 = 0;
                while i1 < chosen.len() && i2 < winning.len() {
                    let choose = chosen[i1];
                    let win = winning[i2];

                    if choose == win {
                        wins += 1;
                        i1 += 1;
                        i2 += 1;
                    } else if choose < win {
                        i1 += 1;
                    } else {
                        i2 += 1;
                    }
                }
            }
        }

        let instances_of_this_card = instances[idx];
        let mut index: usize = 1;
        for _ in 0..wins {
            let new_index = idx + index;
            if new_index < instances.len() {
                instances[new_index] += instances_of_this_card;
                index += 1;
            } else {
                break;
            }
        }
        idx += 1;
    }

    instances.iter().sum()
}
