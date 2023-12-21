use std::collections::HashMap;

pub fn one(lines: &Vec<String>) -> usize {
    let mut lines_iter = lines.iter();
    let sequence = lines_iter.next().unwrap();
    lines_iter.next();

    let mut maps: HashMap<String, (String, String)> = HashMap::new();
    for mapping in lines_iter {
        let p: Vec<&str> = mapping.split("=").collect();
        if let [s, d] = p[..] {
            let source = s.trim();

            let d1 = d.replace("(", "");
            let d2 = d1.replace(")", "");
            let dest: Vec<&str> = d2.split(",").map(|x| x.trim()).collect();
            let dest1 = *dest.get(0).unwrap();
            let dest2 = *dest.get(1).unwrap();

            maps.insert(source.to_string(), (dest1.to_string(), dest2.to_string()));
        }
    }

    let mut node = "AAA";
    let mut steps = 0;
    while node != "ZZZ" {
        let index = steps % sequence.len();
        let r_or_l = sequence.chars().nth(index).unwrap();

        let directions = maps.get(node).unwrap();
        if r_or_l == 'L' {
            node = &directions.0;
        } else {
            node = &directions.1;
        }
        steps += 1;
    }

    steps
}

/// Does not give answer, but can find the Z cycles for each variable.
pub fn two(lines: &Vec<String>) {
    let mut lines_iter = lines.iter();
    let sequence = lines_iter.next().unwrap();
    lines_iter.next();

    let mut maps: HashMap<String, (String, String)> = HashMap::new();
    for mapping in lines_iter {
        let p: Vec<&str> = mapping.split("=").collect();
        if let [s, d] = p[..] {
            let source = s.trim();

            let d1 = d.replace("(", "");
            let d2 = d1.replace(")", "");
            let dest: Vec<&str> = d2.split(",").map(|x| x.trim()).collect();
            let dest1 = *dest.get(0).unwrap();
            let dest2 = *dest.get(1).unwrap();

            maps.insert(source.to_string(), (dest1.to_string(), dest2.to_string()));
        }
    }

    let mut nodes: Vec<String> = maps
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.to_owned())
        .collect();
    let mut steps = 0;

    println!("Starting iter with nodes: {:?} from step {}", nodes, steps);
    while !nodes.iter().all(|n| n.ends_with("Z")) {
        let index = steps % sequence.len();
        let r_or_l = sequence.chars().nth(index).unwrap();
        let mut new_nodes: Vec<String> = vec![];
        let directions = nodes.iter().map(|n| maps.get(n).unwrap());
        for dir in directions {
            if r_or_l == 'L' {
                new_nodes.push(dir.0.clone());
            } else {
                new_nodes.push(dir.1.clone());
            }
        }

        if nodes.iter().any(|n| n.ends_with("Z")) {
            println!("Hit a Z: {:?} with {} steps", nodes, steps);
            // Hey!
            /*
            First hits Z every  13301 steps,
            Second hits Z every 22357 steps,
            Third hits Z every  14999 steps,
            Fourth hits Z every 20659 steps,
            Fifth hits Z every  17263 steps,
            Sixth hits Z every  16697 steps,
             */
            // LCM is the answer.
        }

        nodes = new_nodes;
        steps += 1;
    }
}
