use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use itertools::Itertools;

fn part1(reader: BufReader<File>) -> usize {
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| l.chars().collect())
        .collect_vec();

    let width = lines[0].len();
    let height = lines.len();
    let mut max_val: Vec<usize> = vec![0; width];

    let mut sum: usize = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '#' => {
                    max_val[x] = y + 1;
                }
                'O' => {
                    sum += height - max_val[x];
                    max_val[x] += 1;
                }
                _ => {}
            }
        }
    }
    sum
}

fn part2(reader: BufReader<File>) -> usize {
    let mut lines: Vec<Vec<u8>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => 2,
                    'O' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect_vec();

    let mut set: HashMap<Vec<Vec<u8>>, u32> = HashMap::new();
    let width = lines[0].len();
    let height = lines.len();

    // simulate until pattern if found
    let mut cycle: u32 = 0;
    let mut remains = 1000000000;
    let mut cycle_found = false;
    loop {
        if remains == 0 {
            break;
        }
        // if round < 20 {
        //     println!();
        //     for l in lines.iter() {
        //         for c in l.iter() {
        //             match c {
        //                 2 => print!("#"),
        //                 1 => print!("O"),
        //                 _ => print!("."),
        //             }
        //         }
        //         println!();
        //     }
        // }
        for y in 0..height {
            for x in 0..width {
                if lines[y][x] == 1 {
                    lines[y][x] = 0;
                    let mut i: i32 = y as i32 - 1;
                    loop {
                        if i < 0 || lines[i as usize][x] != 0 {
                            lines[(i + 1) as usize][x] = 1;
                            break;
                        }
                        i -= 1;
                    }
                }
            }
        }
        for y in 0..height {
            for x in 0..width {
                if lines[y][x] == 1 {
                    lines[y][x] = 0;
                    let mut i = x as i32 - 1;
                    loop {
                        if i < 0 || lines[y][i as usize] != 0 {
                            lines[y][(i + 1) as usize] = 1;
                            break;
                        }
                        i -= 1;
                    }
                }
            }
        }
        for y in (0..height).rev() {
            for x in 0..width {
                if lines[y][x] == 1 {
                    lines[y][x] = 0;
                    let mut i = y + 1;
                    loop {
                        if i == height || lines[i][x] != 0 {
                            lines[i - 1][x] = 1;
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
        for y in 0..height {
            for x in (0..width).rev() {
                if lines[y][x] == 1 {
                    lines[y][x] = 0;
                    let mut i = x + 1;
                    loop {
                        if i >= height || lines[y][i] != 0 {
                            lines[y][i - 1] = 1;
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
        cycle += 1;
        if !cycle_found {
            if let Some(n) = set.get(&lines) {
                cycle_found = true;
                remains %= cycle - n;
                println!(
                    "n {} cycle {} diff {} remains {}",
                    n,
                    cycle,
                    cycle - n,
                    remains
                );
            } else {
                set.insert(lines.clone(), cycle);
            }
        }
    
        remains -= 1;
         }
    for l in lines.iter() {
        for c in l.iter() {
            match c {
                2 => print!("#"),
                1 => print!("O"),
                _ => print!("."),
            }
        }
        println!();
    }
    let mut sum: usize = 0;
    for (y, line) in lines.iter().enumerate() {
        for c in line.iter() {
            if c == &1 {
                sum += height - y;
            }
        }
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("/home/vesa/code/rust/adventOfCode/input/day14.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
