pub fn one(lines: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    for line in lines {
        let mut first = '0';
        let mut last = '0';

        for c in line.chars() {
            if c.is_numeric() {
                first = c;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                last = c;
                break;
            }
        }

        if let Ok(num) = format!("{}{}", first, last).parse::<u64>() {
            sum += num;
        }
    }

    sum
}

pub fn two(lines: &mut Vec<String>) -> u64 {
    for line in lines.into_iter() {
        parse_text_num(line);
    }

    one(lines)
}

pub fn parse_text_num(line: &mut String) {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    // first
    let mut lowest_index = line.len();
    let mut lowest_num = 0;
    for num in nums.iter().enumerate() {
        if let Some(idx) = line.find(num.1) {
            if idx < lowest_index {
                lowest_index = idx;
                lowest_num = num.0 + 1;
            }
        }
    }

    // last
    let mut highest_index = 0;
    let mut highest_num = 0;
    for num in nums.iter().enumerate() {
        if let Some(idx) = line.rfind(num.1) {
            if idx > highest_index {
                highest_index = idx + 1;
                highest_num = num.0 + 1;
            }
        }
    }

    if lowest_num > 0 {
        line.insert_str(lowest_index, &lowest_num.to_string());
    }
    if highest_num > 0 {
        line.insert_str(highest_index, &highest_num.to_string());
    }
}
