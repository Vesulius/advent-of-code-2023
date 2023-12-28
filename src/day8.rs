use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part2(reader: BufReader<File>) -> u64 {
    let mut it = reader.lines();
    let directions = it.next().unwrap().unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut start_points: Vec<String> = Vec::new();

    it.skip(1).map_while(Result::ok).for_each(|line| {
        let key = String::from(&line[0..3]);
        if key.chars().nth(2).unwrap() == 'A' {
            start_points.push(key.clone());
        }
        map.insert(
            key,
            (String::from(&line[7..10]), String::from(&line[12..15])),
        );
    });


    let mut steps: i64;
    let mut curr;

    let mut loops: Vec<u64> = Vec::new();
    for s in start_points {
        steps = 0;
        curr = s;
        while steps != -1 {
            for c in directions.chars() {
                if c == 'R' {
                    curr = map.get(&curr).unwrap().1.clone();
                } else {
                    curr = map.get(&curr).unwrap().0.clone();
                }
                steps += 1;
                if curr.chars().nth(2).unwrap() == 'Z' {
                    loops.push(steps as u64);
                    steps = -1;
                    break;
                }
            }
        }
    }
    loops.iter().fold(1, |acc, &x| lcm(acc, x))
}

fn part1(reader: BufReader<File>) -> u32 {
    let mut it = reader.lines();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let directions = it.next().unwrap().unwrap();
    it.skip(1).map_while(Result::ok).for_each(|line| {
        map.insert(
            String::from(&line[0..3]),
            (String::from(&line[7..10]), String::from(&line[12..15])),
        );
    });

    let mut steps = 0;
    let mut curr = "AAA";
    loop {
        for c in directions.chars() {
            if c == 'R' {
                curr = &map.get(curr).unwrap().1;
            } else {
                curr = &map.get(curr).unwrap().0;
            }
            steps += 1;
            if curr == "ZZZ" {
                return steps;
            }
        }
    }
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day8.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
