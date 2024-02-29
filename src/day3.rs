use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::Wrapping;

fn part2(reader: BufReader<File>) -> u32 {
    let mut sum = 0;

    let mut id: u8 = 0;
    let lines: Vec<Vec<(i32, u8)>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut new_line: Vec<(i32, u8)> = Vec::new();
            let mut num: i32 = 0;
            let mut num_len = 0;
            for c in line.chars() {
                if c.is_ascii_digit() {
                    num *= 10;
                    num += c.to_digit(10).unwrap() as i32;
                    num_len += 1;
                } else {
                    if num_len > 0 {
                        for _ in 0..num_len {
                            new_line.push((num, id));
                        }
                        id = (Wrapping(id) + Wrapping(1)).0;
                        num_len = 0;
                        num = 0;
                    }
                    if c == '*' {
                        new_line.push((-1, 0));
                    } else {
                        new_line.push((0, 0));
                    }
                }
            }
            for _ in 0..num_len {
                new_line.push((num, id));
            }
            id = (Wrapping(id) + Wrapping(1)).0;
            new_line
        })
        .collect();
    for (_, line) in lines.iter().enumerate() {
        for (_, (curr, _)) in line.iter().enumerate() {
            print!("{}\t", curr);
        }
        println!();
    }
    let mut set: HashSet<u8> = HashSet::new();
    let mut num = 1;
    for (y, line) in lines.iter().enumerate() {
        for (x, (curr, _)) in line.iter().enumerate() {
            if curr == &-1 {
                if y > 0 {
                    if lines[y - 1][x].0 > 0 && !set.contains(&lines[y - 1][x].1) {
                        num *= lines[y - 1][x].0;
                        set.insert(lines[y - 1][x].1);
                    }
                    if x > 0 && lines[y - 1][x - 1].0 > 0 && !set.contains(&lines[y - 1][x - 1].1) {
                        num *= lines[y - 1][x - 1].0;
                        set.insert(lines[y - 1][x - 1].1);
                    }
                    if x + 1 < line.len()
                        && lines[y - 1][x + 1].0 > 0
                        && !set.contains(&lines[y - 1][x + 1].1)
                    {
                        num *= lines[y - 1][x + 1].0;
                        set.insert(lines[y - 1][x + 1].1);
                    }
                }
                if y + 1 < line.len() {
                    if lines[y + 1][x].0 > 0 && !set.contains(&lines[y + 1][x].1) {
                        num *= lines[y + 1][x].0;
                        set.insert(lines[y + 1][x].1);
                    }
                    if x > 0 && lines[y + 1][x - 1].0 > 0 && !set.contains(&lines[y + 1][x - 1].1) {
                        num *= lines[y + 1][x - 1].0;
                        set.insert(lines[y + 1][x - 1].1);
                    }
                    if x + 1 < line.len()
                        && lines[y + 1][x + 1].0 > 0
                        && !set.contains(&lines[y + 1][x + 1].1)
                    {
                        num *= lines[y + 1][x + 1].0;
                        set.insert(lines[y + 1][x + 1].1);
                    }
                }
                if x > 0 && lines[y][x - 1].0 > 0 && !set.contains(&lines[y][x - 1].1) {
                    num *= lines[y][x - 1].0;
                    set.insert(lines[y][x - 1].1);
                }
                if x + 1 < line.len() && lines[y][x + 1].0 > 0 && !set.contains(&lines[y][x + 1].1)
                {
                    num *= lines[y][x + 1].0;
                    set.insert(lines[y][x + 1].1);
                }
            }
            if set.len() == 2 {
                sum += num as u32;
            }
            num = 1;
            set.clear();
        }
    }
    sum
}

fn part1(reader: BufReader<File>) -> u32 {
    let mut sum = 0;

    let lines: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    for (y, line) in lines.iter().enumerate() {
        let mut num = 0;
        let mut used = false;
        for (x, char) in line.iter().enumerate() {
            if char.is_numeric() {
                if num == 0 {
                    used = x as i32 - 1 >= 0
                        && ((y + 1 < lines.len() && lines[y + 1][x - 1] != '.')
                            || (y as i32 - 1 >= 0 && lines[y - 1][x - 1] != '.')
                            || (lines[y][x - 1] != '.'));
                }
                num *= 10;
                num += char.to_digit(10).unwrap();
                used = used
                    || (y as i32 - 1 >= 0 && lines[y - 1][x] != '.')
                    || (y + 1 < lines.len() && lines[y + 1][x] != '.');
            } else {
                if num != 0 {
                    if used
                        || (lines[y][x] != '.')
                        || (y + 1 < lines.len() && lines[y + 1][x] != '.')
                        || (y as i32 - 1 >= 0 && lines[y - 1][x] != '.')
                    {
                        sum += num;
                    }
                }
                num = 0;
                used = false;
            }
        }
        if used {
            sum += num;
        }
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("../input/day3.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
