use std::collections::HashMap;

pub fn one(games: &Vec<String>) -> i32 {
    let mut limits = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);

    let mut possible_games: Vec<i32> = vec![];

    let mut index = 1;
    for game in games {
        let mut is_game_possible = true;

        let a: Vec<&str> = game.split(":").collect();
        if let [.., r] = a[..] {
            let rounds: Vec<&str> = r.split(";").map(|g| g.trim()).collect();
            for round in rounds {
                let mut is_round_possible = true;

                let cubes: Vec<&str> = round.split(",").map(|cube| cube.trim()).collect();
                for cube in cubes {
                    let b: Vec<&str> = cube.split(" ").collect();
                    if let [amount_str, color] = b[..] {
                        let amount = amount_str.parse::<usize>().unwrap();
                        let limit = limits.get(color).unwrap();
                        if amount > *limit {
                            is_round_possible = false;
                            break;
                        }
                    }
                }

                if !is_round_possible {
                    is_game_possible = false;
                    break;
                }
            }
        }
        if is_game_possible {
            possible_games.push(index);
        }
        index += 1;
    }

    possible_games.iter().sum()
}

pub fn two(games: &Vec<String>) -> i32 {
    let mut least_powers: Vec<i32> = vec![];
    for game in games {
        let mut least_amounts: HashMap<&str, i32> = HashMap::new();

        let a: Vec<&str> = game.split(":").collect();
        if let [.., r] = a[..] {
            let rounds: Vec<&str> = r.split(";").map(|g| g.trim()).collect();
            for round in rounds {
                let cubes: Vec<&str> = round.split(",").map(|cube| cube.trim()).collect();
                for cube in cubes {
                    let b: Vec<&str> = cube.split(" ").collect();
                    if let [amount_str, color] = b[..] {
                        let amount = amount_str.parse::<i32>().unwrap();
                        if let Some(least) = least_amounts.get(color) {
                            if amount > *least {
                                least_amounts.insert(color, amount);
                            }
                        } else {
                            least_amounts.insert(color, amount);
                        }
                    }
                }
            }
        }

        let r = least_amounts.get("red").unwrap();
        let g = least_amounts.get("green").unwrap();
        let b = least_amounts.get("blue").unwrap();
        least_powers.push(r * g * b);
    }

    least_powers.iter().sum()
}
