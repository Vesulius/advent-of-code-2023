use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1(reader: BufReader<File>) -> i32 {
    let heatloss: Vec<Vec<u8>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let height = heatloss.len() as i32;
    let width = heatloss[0].len() as i32;

    let mut next_position: BinaryHeap<(i32, (i32, i32, (i8, i8)))> = BinaryHeap::new();
    let mut costs: HashMap<(i32, i32, (i8, i8)), i32> = HashMap::new();

    next_position.push((0, (0, 0, (0, 0))));

    while let Some((cost, (y, x, dir))) = next_position.pop() {
        if y == height - 1 && x == width - 1 {
            return -cost;
        }
        if let Some(existing_cost) = costs.get(&(y, x, dir)) {
            if -existing_cost < -cost {
                continue;
            }
        }
        for (move_y, move_x) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (move_y, move_x) == dir || (-move_y, -move_x) == dir {
                continue;
            }
            let mut new_cost = cost;
            for i in 1..=3 {
                let new_y = y + (move_y as i32 * i);
                let new_x = x + (move_x as i32 * i);

                if new_x < 0 || new_y < 0 || new_x >= width || new_y >= height {
                    break;
                }
                new_cost -= (heatloss[new_y as usize][new_x as usize]) as i32;
                let key = (new_y, new_x, (move_y, move_x));
                if new_cost > *costs.get(&key).unwrap_or(&-99999999) {
                    costs.insert(key, new_cost);
                    next_position.push((new_cost, key))
                }
            }
        }
    }
    panic!("should not get here")
}

fn part2(reader: BufReader<File>) -> i32 {
    let heatloss: Vec<Vec<u8>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let height = heatloss.len() as i32;
    let width = heatloss[0].len() as i32;

    let mut next_position: BinaryHeap<(i32, (i32, i32, (i8, i8)))> = BinaryHeap::new();
    let mut costs: HashMap<(i32, i32, (i8, i8)), i32> = HashMap::new();

    next_position.push((0, (0, 0, (0, 0))));

    while let Some((cost, (y, x, dir))) = next_position.pop() {
        if y == height - 1 && x == width - 1 {
            return -cost;
        }
        if let Some(existing_cost) = costs.get(&(y, x, dir)) {
            if -existing_cost < -cost {
                continue;
            }
        }
        for (move_y, move_x) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (move_y, move_x) == dir || (-move_y, -move_x) == dir {
                continue;
            }
            let mut new_cost = cost;
            for i in 1..=10 {
                let new_y = y + (move_y as i32 * i);
                let new_x = x + (move_x as i32 * i);
                if new_x < 0 || new_y < 0 || new_x >= width || new_y >= height {
                    break;
                }
                new_cost -= (heatloss[new_y as usize][new_x as usize]) as i32;

                if i < 4 {
                    continue;
                } 

                let key = (new_y, new_x, (move_y, move_x));
                if new_cost > *costs.get(&key).unwrap_or(&-99999999) {
                    costs.insert(key, new_cost);
                    next_position.push((new_cost, key))
                }
            }
        }
    }
    panic!("should not get here")
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day17.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
