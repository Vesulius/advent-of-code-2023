use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1(reader: BufReader<File>) -> i32 {
    let mut set: HashMap<(i32, i32), u8> = HashMap::new();

    let mut y = 0;
    let mut x: i32 = 0;
    let mut d: u8 = 0;
    let mut min_y = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut max_x = 0;

    reader.lines().map_while(Result::ok).for_each(|l| {
        let instructions: Vec<&str> = l.split_whitespace().collect();
        let dir = instructions[0];
        let dist = instructions[1].parse::<i32>().unwrap();

        match dir {
            "U" => {
                d = 1;
                for i in 1..=dist {
                    set.insert((y + i, x), d);
                }
                y += dist;
                if y > max_y {
                    max_y = y;
                }
            }
            "D" => {
                d = 3;
                for i in 1..=dist {
                    set.insert((y - i, x), d);
                }
                y -= dist;
                if y < min_y {
                    min_y = y;
                }
            }
            "L" => {
                d = 4;
                for i in 1..=dist {
                    set.insert((y, x - i), d);
                }
                x -= dist;
                if x < min_x {
                    min_x = x;
                }
            }
            "R" => {
                d = 4;
                for i in 1..=dist {
                    set.insert((y, x + i), d);
                }
                x += dist;
                if x > max_x {
                    max_x = x;
                }
            }
            _ => panic!("Wrong direction"),
        }
    });

    // add 1 row of buffer for the flood fill algo
    min_y -= 1;
    min_x -= 1;
    max_y += 2;
    max_x += 2;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut to_be_visited: Vec<(i32, i32)> = Vec::new();
    to_be_visited.push((min_y, min_x));

    while let Some((y, x)) = to_be_visited.pop() {
        if set.contains_key(&(y, x)) {
            continue;
        }
        visited.insert((y, x));
        if y - 1 > min_y && !visited.contains(&(y - 1, x)) {
            to_be_visited.push((y - 1, x));
        }
        if x - 1 > 0 && !visited.contains(&(y, x - 1)) {
            to_be_visited.push((y, x - 1));
        }
        if y + 1 < max_y && !visited.contains(&(y + 1, x)) {
            to_be_visited.push((y + 1, x));
        }
        if x + 1 < max_x && !visited.contains(&(y, x + 1)) {
            to_be_visited.push((y, x + 1));
        }
    }

    (max_y - min_y) * (max_x - min_x) - visited.len() as i32
}

fn part2(reader: BufReader<File>) -> u64 {
    let mut len: u64 = 0;
    let mut y: i64 = 0;
    let mut x: i64 = 0;
    let mut area: i64 = 0;
    let dirs: [(i64, i64); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    reader.lines().map_while(Result::ok).for_each(|l| {
        let instruc: &str = l.split_whitespace().skip(2).next().unwrap();
        let dir = i64::from_str_radix(&instruc[instruc.len() - 2..instruc.len() - 1], 16).unwrap();
        let dist = i64::from_str_radix(&instruc[2..instruc.len() - 2], 16).unwrap();

        let old_y = y;
        let old_x = x;

        y += dirs[dir as usize].0 * dist;
        x += dirs[dir as usize].1 * dist;

        len += dist as u64;
        area += (old_x * y) - (x * old_y);
    });
    if len % 2 != 0 {
        panic!("Perimeter length is not even");
    }
    // https://en.wikipedia.org/wiki/Shoelace_formula
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    (area / 2).abs() as u64 + len / 2 + 1
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day18.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
