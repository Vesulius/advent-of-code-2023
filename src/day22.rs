use itertools::Itertools;
use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    start: (u32, u32, u32),
    end: (u32, u32, u32),
}

fn part1(reader: BufReader<File>) -> u64 {
    let mut max_y = 0;
    let mut max_x = 0;
    let mut bricks: Vec<Brick> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let split: Vec<u32> = line
                .split(|c| c == '~' || c == ',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            max_y = max(max_y, split[4]);
            max_x = max(max_x, split[3]);

            Brick {
                start: (split[0], split[1], split[2]),
                end: (split[3], split[4], split[5]),
            }
        })
        .collect_vec();

    bricks.sort_by(|b1, b2| b1.end.2.cmp(&b2.end.2));
    let mut topology: Vec<Vec<u32>> = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    for b in &mut bricks {
        let mut max_top = 0;
        for y in b.start.1..=b.end.1 {
            for x in b.start.0..=b.end.0 {
                max_top = max(max_top, topology[y as usize][x as usize]);
            }
        }
        let brick_height = b.end.2 - b.start.2;
        b.start.2 = max_top;
        b.end.2 = max_top + brick_height;
        for y in b.start.1..=b.end.1 {
            for x in b.start.0..=b.end.0 {
                topology[y as usize][x as usize] = b.end.2 + 1;
            }
        }
    }

    fn can_be_destroyed(
        destroyed_brick_index: usize,
        bricks: &Vec<Brick>,
        max_x: u32,
        max_y: u32,
    ) -> bool {
        let mut topology: Vec<Vec<u32>> = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
        for (i, b) in bricks.iter().enumerate() {
            if i == destroyed_brick_index {
                continue;
            }
            let mut max_top = 0;
            for y in b.start.1..=b.end.1 {
                for x in b.start.0..=b.end.0 {
                    max_top = max(max_top, topology[y as usize][x as usize]);
                    topology[y as usize][x as usize] = b.end.2 + 1;
                }
            }
            if b.start.2 != max_top {
                return false;
            }
        }
        true
    }

    bricks.sort_by(|b1, b2| b1.start.2.cmp(&b2.start.2));
    let mut sum = 0;
    for i in 0..bricks.len() {
        if can_be_destroyed(i, &bricks, max_x, max_y) {
            sum += 1;
        }
    }

    sum
}

fn part2(reader: BufReader<File>) -> u64 {
    let mut max_y = 0;
    let mut max_x = 0;
    let mut bricks: Vec<Brick> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let split: Vec<u32> = line
                .split(|c| c == '~' || c == ',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            max_y = max(max_y, split[4]);
            max_x = max(max_x, split[3]);

            Brick {
                start: (split[0], split[1], split[2]),
                end: (split[3], split[4], split[5]),
            }
        })
        .collect_vec();

    bricks.sort_by(|b1, b2| b1.end.2.cmp(&b2.end.2));
    let mut topology: Vec<Vec<u32>> = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    for b in &mut bricks {
        let mut max_top = 0;
        for y in b.start.1..=b.end.1 {
            for x in b.start.0..=b.end.0 {
                max_top = max(max_top, topology[y as usize][x as usize]);
            }
        }
        let brick_height = b.end.2 - b.start.2;
        b.start.2 = max_top;
        b.end.2 = max_top + brick_height;
        for y in b.start.1..=b.end.1 {
            for x in b.start.0..=b.end.0 {
                topology[y as usize][x as usize] = b.end.2 + 1;
            }
        }
    }

    fn can_be_destroyed(
        destroyed_brick_index: usize,
        bricks: &mut Vec<Brick>,
        max_x: u32,
        max_y: u32,
    ) -> u64 {
        let mut fallen_bricks = 0;
        let mut topology: Vec<Vec<u32>> = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
        for (i, b) in bricks.iter_mut().enumerate() {
            if i == destroyed_brick_index {
                continue;
            }
            let mut max_top = 0;
            for y in b.start.1..=b.end.1 {
                for x in b.start.0..=b.end.0 {
                    max_top = max(max_top, topology[y as usize][x as usize]);
                    topology[y as usize][x as usize] = b.end.2 + 1;
                }
            }
            if b.start.2 != max_top {
                fallen_bricks += 1;
            }
            let brick_height = b.end.2 - b.start.2;
            b.start.2 = max_top;
            b.end.2 = max_top + brick_height;
            for y in b.start.1..=b.end.1 {
                for x in b.start.0..=b.end.0 {
                    topology[y as usize][x as usize] = b.end.2 + 1;
                }
            }
        }
        fallen_bricks
    }

    bricks.sort_by(|b1, b2| b1.start.2.cmp(&b2.start.2));
    let mut sum = 0;
    for i in 0..bricks.len() {
        sum += can_be_destroyed(i, &mut bricks.clone(), max_x, max_y);
    }

    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day22.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
