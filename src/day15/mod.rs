pub fn one(lines: &Vec<String>) -> u32 {
    let sequence: Vec<&str> = lines.get(0).unwrap().split(",").collect();
    let mut total = 0;
    for part in sequence {
        total += FocusHashMap::hash(part);
    }
    total
}

#[derive(Debug)]
struct FocusHashMap<'a> {
    boxes: Vec<Vec<(&'a str, u32)>>,
}

impl<'a> FocusHashMap<'a> {
    const LEN: u32 = 256;

    pub fn new() -> Self {
        let inner: Vec<(&str, u32)> = vec![];
        Self {
            boxes: vec![inner; Self::LEN as usize],
        }
    }

    pub fn set(&mut self, key: &'a str, val: u32) {
        let h = Self::hash(key);
        let values = self.boxes.get_mut(h as usize).unwrap();
        for v in values.iter_mut() {
            if v.0 == key {
                v.1 = val;
                return;
            }
        }
        values.push((key, val));
    }

    pub fn remove(&mut self, key: &'a str) {
        let h = Self::hash(key);
        let values = self.boxes.get_mut(h as usize).unwrap();
        let mut idx = 0;
        let mut found = false;
        for v in values.iter() {
            if v.0 == key {
                found = true;
                break;
            }
            idx += 1;
        }
        if found {
            values.remove(idx);
        }
    }

    pub fn focusing_power(&self) -> u32 {
        let mut total = 0;
        for b in 0..self.boxes.len() {
            let foc_box = self.boxes.get(b).unwrap();
            for v in 0..foc_box.len() {
                let val = foc_box.get(v).unwrap();
                total += (b + 1) * (v + 1) * val.1 as usize;
            }
        }
        total as u32
    }

    fn hash(text: &str) -> u32 {
        let mut current_value = 0;
        for c in text.chars() {
            let ascii_val = c as u32;
            current_value += ascii_val;
            current_value *= 17;
            current_value %= Self::LEN as u32;
        }
        current_value
    }
}

pub fn two(lines: &Vec<String>) -> u32 {
    let mut custom = FocusHashMap::new();
    let sequence: Vec<&str> = lines.get(0).unwrap().split(",").collect();
    for part in sequence {
        if part.contains("=") {
            if let [key, value] = part.split("=").collect::<Vec<&str>>()[..] {
                let u_val = value.parse().unwrap();
                custom.set(key, u_val);
            }
        } else {
            custom.remove(&part[..part.len() - 1]);
        }
    }
    custom.focusing_power()
}
