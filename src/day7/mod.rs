use std::collections::HashMap;

#[derive(Debug)]
struct HandOne {
    pub hand: String,
    pub hand_type: HandType,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandOne {
    fn new(hand: String) -> Self {
        let mut letters: HashMap<char, i64> = HashMap::new();

        for ch in hand.chars() {
            letters
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut hand_type = HandType::HighCard;
        let mut highest = 0;
        let mut lowest = 5;
        for v in letters.values() {
            if *v > highest {
                highest = *v;
            }
            if *v < lowest {
                lowest = *v;
            }
        }
        if highest == 5 {
            hand_type = HandType::FiveOfAKind;
        } else if highest == 4 {
            hand_type = HandType::FourOfAKind;
        } else if highest == 3 {
            if lowest == 2 {
                // FH
                hand_type = HandType::FullHouse;
            } else {
                // ToK
                hand_type = HandType::ThreeOfAKind;
            }
        } else if highest == 2 {
            // TP or OP
            let mut found = false;
            let mut double = false;
            for v in letters.values() {
                if *v == highest {
                    if !found {
                        found = true;
                    } else {
                        double = true;
                        break;
                    }
                }
            }
            if double {
                hand_type = HandType::TwoPair;
            } else {
                hand_type = HandType::OnePair
            }
        }
        // else High Card

        Self { hand, hand_type }
    }
    fn better_than(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type > other.hand_type {
            return std::cmp::Ordering::Greater;
        } else if self.hand_type < other.hand_type {
            return std::cmp::Ordering::Less;
        } else {
            let card_ranks: HashMap<char, i64> = HashMap::from([
                ('A', 12),
                ('K', 11),
                ('Q', 10),
                ('J', 9),
                ('T', 8),
                ('9', 7),
                ('8', 6),
                ('7', 5),
                ('6', 4),
                ('5', 3),
                ('4', 2),
                ('3', 1),
                ('2', 0),
            ]);

            let mut self_iter = self.hand.chars();
            let mut other_iter = other.hand.chars();

            for _ in 0..5 {
                let self_val = card_ranks.get(&self_iter.next().unwrap()).unwrap();
                let other_val = card_ranks.get(&other_iter.next().unwrap()).unwrap();

                if *self_val > *other_val {
                    return std::cmp::Ordering::Greater;
                } else if *self_val < *other_val {
                    return std::cmp::Ordering::Less;
                }
            }
        }
        std::cmp::Ordering::Less
    }
}

pub fn one(lines: &Vec<String>) -> i64 {
    let mut hands: Vec<(HandOne, i64)> = vec![];
    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();
        if let [h, b] = s[..] {
            let hand = HandOne::new(h.to_string());
            let bid = b.parse::<i64>().unwrap();
            hands.push((hand, bid));
        }
    }
    hands.sort_by(|first, second| first.0.better_than(&second.0));

    let mut sum: i64 = 0;
    for i in 0..hands.len() {
        let hand = &hands[i];
        sum += (i as i64 + 1) * hand.1;
    }

    sum
}

// ---

#[derive(Debug)]
struct HandTwo {
    pub hand: String,
    pub hand_type: HandType,
}

impl HandTwo {
    fn new(hand: String) -> Self {
        let mut letters: HashMap<char, i64> = HashMap::new();
        for ch in hand.chars() {
            letters
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        // remove jokers from letters
        // note how many there are though
        let joker_key = &'J';
        let mut joker_amount: i64 = 0;
        let jokers = letters.get(joker_key);
        if jokers.is_some() {
            joker_amount = *jokers.unwrap();
            letters.remove(joker_key);
        }

        let mut hand_type = HandType::HighCard;
        let mut highest = 0;
        let mut lowest = 5;
        for v in letters.values() {
            if *v > highest {
                highest = *v;
            }
            if *v < lowest {
                lowest = *v;
            }
        }
        if highest + joker_amount == 5 {
            // 5 same cards - all jokers range
            hand_type = HandType::FiveOfAKind;
            // ok
        } else if highest + joker_amount == 4 {
            // 4 same cards - two different cards + 3 jokers range
            hand_type = HandType::FourOfAKind;
            // ok
        } else if highest + joker_amount == 3 {
            if lowest == 2 {
                // two pairs + 1 joker
                hand_type = HandType::FullHouse;
            } else {
                // three different cards + 2 jokers
                hand_type = HandType::ThreeOfAKind;
            }
            // ok
        } else if highest + joker_amount == 2 {
            // TP or OP
            if joker_amount == 0 {
                let mut found = false;
                let mut double = false;
                for v in letters.values() {
                    if *v == highest {
                        if !found {
                            found = true;
                        } else {
                            double = true;
                            break;
                        }
                    }
                }
                if double {
                    hand_type = HandType::TwoPair;
                } else {
                    hand_type = HandType::OnePair
                }
            } else {
                // 1 joker, all others are 1
                hand_type = HandType::OnePair;
            }
        }
        // else High Card

        Self { hand, hand_type }
    }
    fn better_than(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type > other.hand_type {
            return std::cmp::Ordering::Greater;
        } else if self.hand_type < other.hand_type {
            return std::cmp::Ordering::Less;
        } else {
            let card_ranks: HashMap<char, i64> = HashMap::from([
                ('A', 12),
                ('K', 11),
                ('Q', 10),
                ('T', 9),
                ('9', 8),
                ('8', 7),
                ('7', 6),
                ('6', 5),
                ('5', 4),
                ('4', 3),
                ('3', 2),
                ('2', 1),
                ('J', 0),
            ]);

            let mut self_iter = self.hand.chars();
            let mut other_iter = other.hand.chars();

            for _ in 0..5 {
                let self_val = card_ranks.get(&self_iter.next().unwrap()).unwrap();
                let other_val = card_ranks.get(&other_iter.next().unwrap()).unwrap();

                if *self_val > *other_val {
                    return std::cmp::Ordering::Greater;
                } else if *self_val < *other_val {
                    return std::cmp::Ordering::Less;
                }
            }
        }
        std::cmp::Ordering::Less
    }
}

pub fn two(lines: &Vec<String>) -> i64 {
    let mut hands: Vec<(HandTwo, i64)> = vec![];
    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();
        if let [h, b] = s[..] {
            let hand = HandTwo::new(h.to_string());
            let bid = b.parse::<i64>().unwrap();
            hands.push((hand, bid));
        }
    }
    hands.sort_by(|first, second| first.0.better_than(&second.0));

    let mut sum: i64 = 0;
    for i in 0..hands.len() {
        let hand = &hands[i];
        sum += (i as i64 + 1) * hand.1;
    }

    sum
}
