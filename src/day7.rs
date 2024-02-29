use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn card_str(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        num => num.to_digit(10).unwrap(),
    }
}

fn hand_str(hand: &str) -> u32 {
    let mut hs: HashSet<char> = HashSet::new();
    for c in hand.chars() {
        hs.insert(c);
    }

    if hs.len() == 1 {
        return 7;
    } else if hs.len() == 2 {
        // check for fullhouse vs fours
        let card = hs.iter().next().unwrap();
        let mut instances = 0;
        for curr in hand.chars() {
            if curr == *card {
                instances += 1;
            }
        }
        if instances == 4 || instances == 1 {
            return 6;
        } else {
            return 5;
        }
    } else if hs.len() == 3 {
        // check for 2 pairs vs triples
        let mut it = hs.iter();
        let card1 = it.next().unwrap();
        let card2 = it.next().unwrap();
        let mut instances1 = 0;
        let mut instances2 = 0;
        for curr in hand.chars() {
            if curr == *card1 {
                instances1 += 1;
            }
            if curr == *card2 {
                instances2 += 1;
            }
        }
        if instances1 == 2 || instances2 == 2 {
            return 3;
        } else {
            return 4;
        }
    }

    6 - hs.len() as u32
}

fn rank_hands(h1: &str, h2: &str) -> Ordering {
    let r1 = hand_str(h1);
    let r2 = hand_str(h2);
    if r1 == r2 {
        let mut chars1 = h1.chars();
        let mut chars2 = h2.chars();
        for _ in 0..5 {
            let v1 = card_str(chars1.next().unwrap());
            let v2 = card_str(chars2.next().unwrap());
            if v1 == v2 {
                continue;
            } else {
                return v1.cmp(&v2);
            }
        }
        Ordering::Equal
    } else {
        r1.cmp(&r2)
    }
}

fn part1(reader: BufReader<File>) -> u32 {
    let mut hands: Vec<(String, u32)> = reader
        .lines()
        .map_while(Result::ok)
        .map(|s| {
            let mut splits = s.split_whitespace();
            (
                String::from(splits.next().unwrap()),
                splits.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    println!("{:?}", hands);
    hands.sort_by(|v1, v2| rank_hands(&v1.0, &v2.0));
    println!("{:?}", hands);
    let mut sum = 0;
    for (i, h) in hands.iter().enumerate() {
        sum += (i as u32 + 1) * h.1;
    }
    sum
}

fn card_str2(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        num => num.to_digit(10).unwrap(),
    }
}
fn hand_str2(hand: &str) -> u32 {
    let mut hs: HashSet<char> = HashSet::new();
    let mut jokers = 0;
    for c in hand.chars() {
        if c == 'J' {
            jokers += 1;
        }
        hs.insert(c);
    }

    if hs.len() == 1 {
        return 7;
    } else if hs.len() == 2 {
        // check for fullhouse vs fours
        let card = hs.iter().next().unwrap();
        let mut instances = 0;
        for curr in hand.chars() {
            if curr == *card {
                instances += 1;
            }
        }
        if instances == 4 || instances == 1 {
            if jokers > 0 {
                return 7;
            } else {
                return 6;
            }
        } else {
            if jokers > 0 {
                return 7;
            }
            return 5;
        }
    } else if hs.len() == 3 {
        // check for 2 pairs vs triples
        let mut it = hs.iter();
        let card1 = it.next().unwrap();
        let card2 = it.next().unwrap();
        let mut instances1 = 0;
        let mut instances2 = 0;
        for curr in hand.chars() {
            if curr == *card1 {
                instances1 += 1;
            }
            if curr == *card2 {
                instances2 += 1;
            }
        }
        if instances1 == 2 || instances2 == 2 {
            if jokers == 1 {
                return 5;
            } else if jokers > 1 {
                return 6;
            }
            return 3;
        } else {
            if jokers != 0 {
                return 6;
            }
            return 4;
        }
    } else if hs.len() == 4 {
        if jokers != 0 {
            return 4;
        } else {
            return 2;
        }
    } else {
        if jokers != 0 {
            return 2;
        } else {
            return 1;
        }
    }
}

fn rank_hands2(h1: &str, h2: &str) -> Ordering {
    let r1 = hand_str2(h1);
    let r2 = hand_str2(h2);
    if r1 == r2 {
        let mut chars1 = h1.chars();
        let mut chars2 = h2.chars();
        for _ in 0..5 {
            let v1 = card_str2(chars1.next().unwrap());
            let v2 = card_str2(chars2.next().unwrap());
            if v1 == v2 {
                continue;
            } else {
                return v1.cmp(&v2);
            }
        }
        Ordering::Equal
    } else {
        r1.cmp(&r2)
    }
}

fn part2(reader: BufReader<File>) -> u32 {
    let mut hands: Vec<(String, u32)> = reader
        .lines()
        .map_while(Result::ok)
        .map(|s| {
            let mut splits = s.split_whitespace();
            (
                String::from(splits.next().unwrap()),
                splits.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    hands.sort_by(|v1, v2| rank_hands2(&v1.0, &v2.0));
    let mut sum = 0;
    for (i, h) in hands.iter().enumerate() {
        sum += (i as u32 + 1) * h.1;
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day7.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
