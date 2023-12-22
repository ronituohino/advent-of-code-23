fn get_char(x: usize, y: usize, map: &Vec<String>) -> char {
    map.get(y).unwrap().chars().nth(x).unwrap()
}

fn handle_symbol(c: char, invalid: &mut bool) {
    if c != '.' && !c.is_numeric() {
        *invalid = false;
    }
}

pub fn one(map: &Vec<String>) -> i32 {
    let mut sum = 0;

    let height = map.len();
    let width = map.get(0).unwrap().len();

    for y in 0..height {
        let mut recorded: Vec<char> = vec![];
        let mut invalid = true;

        for x in 0..width {
            let c = get_char(x, y, map);
            if c.is_numeric() {
                if x > 0 {
                    handle_symbol(get_char(x - 1, y, map), &mut invalid);
                    if y < height - 1 {
                        handle_symbol(get_char(x - 1, y + 1, map), &mut invalid);
                    }

                    if y > 0 {
                        handle_symbol(get_char(x - 1, y - 1, map), &mut invalid);
                    }
                }
                if y < height - 1 {
                    handle_symbol(get_char(x, y + 1, map), &mut invalid);
                }
                if y > 0 {
                    handle_symbol(get_char(x, y - 1, map), &mut invalid);
                }
                if x < width - 1 {
                    handle_symbol(get_char(x + 1, y, map), &mut invalid);
                    if y < height - 1 {
                        handle_symbol(get_char(x + 1, y + 1, map), &mut invalid);
                    }
                    if y > 0 {
                        handle_symbol(get_char(x + 1, y - 1, map), &mut invalid);
                    }
                }
                recorded.push(c);
            }
            if (!c.is_numeric() && recorded.len() > 0) || x == width - 1 {
                if !invalid {
                    let res: String = recorded.iter().collect();
                    let num = res.parse::<i32>().unwrap();
                    sum += num;
                }
                recorded.clear();
                invalid = true;
            }
        }
    }

    sum
}

pub fn two(map: &Vec<String>) -> i32 {
    let mut sum = 0;

    let height = map.len();
    let width = map.get(0).unwrap().len();

    let mut gears = vec![];
    // num as String, ending coords
    let mut numbers = vec![];

    for y in 0..height {
        let mut recorded: Vec<char> = vec![];

        for x in 0..width {
            let c = get_char(x, y, map);

            if c.is_numeric() {
                recorded.push(c);
            } else if recorded.len() > 0 {
                // we entered a new char at it's not a num
                let num: String = recorded.iter().collect();
                numbers.push((x - 1, y, num));
                recorded.clear();
            }
            if recorded.len() > 0 && x == width - 1 {
                let num: String = recorded.iter().collect();
                numbers.push((x, y, num));
                recorded.clear();
            }
            if c == '*' {
                gears.push((x, y));
            }
        }
    }

    for gear in gears {
        let mut nearby_nums = vec![];

        for num in &numbers {
            let gear_has_space_on_left = gear.0 > 0;
            let gear_has_space_on_right = gear.0 < width - 1;
            let gear_has_space_below = gear.1 < height - 1;
            let gear_has_space_on_top = gear.1 > 0;

            let num_end_pos = num.0;
            let num_start_pos = num_end_pos - (num.2.len() - 1);

            // on the same line
            if num.1 == gear.1 {
                // number ended on the left of gear
                if gear_has_space_on_left && num_end_pos == gear.0 - 1 {
                    nearby_nums.push(num);
                }

                // number starts on the right of gear
                if gear_has_space_on_right && num_start_pos == gear.0 + 1 {
                    nearby_nums.push(num);
                }
            } else if gear_has_space_below && num.1 == gear.1 + 1 {
                // num is below gear
                if !(gear_has_space_on_left && num_end_pos < gear.0 - 1)
                    && !(gear_has_space_on_right && num_start_pos > gear.0 + 1)
                {
                    nearby_nums.push(num);
                }
            } else if gear_has_space_on_top && num.1 == gear.1 - 1 {
                // num is on top of gear
                if !(gear_has_space_on_left && num_end_pos < gear.0 - 1)
                    && !(gear_has_space_on_right && num_start_pos > gear.0 + 1)
                {
                    nearby_nums.push(num);
                }
            }
        }

        if nearby_nums.len() == 2 {
            let mut iter = nearby_nums.iter();
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();

            let res = a.2.parse::<i32>().unwrap() * b.2.parse::<i32>().unwrap();
            sum += res;
        }
    }

    sum
}
