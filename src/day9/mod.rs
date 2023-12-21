pub fn one(lines: &Vec<String>) -> i64 {
    let mut result = 0;
    for line in lines {
        let mut vals: Vec<i64> = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
        let mut diffs: Vec<i64> = vec![];

        let mut last_vals = vec![];

        while diffs.len() == 0 || !diffs.iter().all(|v| *v == 0) {
            let last_val = vals.iter().rev().next().unwrap();
            last_vals.push(*last_val);

            diffs.clear();
            for i in 0..vals.len() - 1 {
                let a = vals.get(i).unwrap();
                let b = vals.get(i + 1).unwrap();
                diffs.push(b - a);
            }
            vals = diffs.clone();
        }

        let sum: i64 = last_vals.iter().sum();
        result += sum;
    }
    result
}

pub fn two(lines: &Vec<String>) -> i64 {
    let mut result = 0;
    for line in lines {
        let mut vals: Vec<i64> = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
        let mut diffs: Vec<i64> = vec![];

        let mut first_vals = vec![];

        while diffs.len() == 0 || !diffs.iter().all(|v| *v == 0) {
            let first_val = vals.iter().next().unwrap();
            first_vals.push(*first_val);

            diffs.clear();
            for i in 0..vals.len() - 1 {
                let a = vals.get(i).unwrap();
                let b = vals.get(i + 1).unwrap();
                diffs.push(b - a);
            }
            vals = diffs.clone();
        }

        let mut neg = 0;
        for i in (0..first_vals.len()).rev() {
            let v = first_vals.get(i).unwrap();
            neg = v - neg;
        }
        result += neg;
    }
    result
}
