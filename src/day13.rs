use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1(reader: BufReader<File>) -> usize {
    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut sum: usize = 0;
    let mut it = reader.lines();
    while let Some(line) = it.next() {
        let line = line.unwrap();
        if line.is_empty() {
            if rows.len() > 0 {
                let mut found_ref = false;
                for i in 1..rows.len() {
                    if rows[i - 1] == rows[i] {
                        found_ref = true;
                        for j in 0..cmp::min(i, rows.len() - i) {
                            found_ref &= rows[i - 1 - j] == rows[i + j];
                        }
                        if found_ref {
                            sum += 100 * i;
                            break;
                        }
                    }
                }
                if !found_ref {
                    for i in 1..columns.len() {
                        if columns[i - 1] == columns[i] {
                            found_ref = true;
                            for j in 0..cmp::min(i, columns.len() - i) {
                                found_ref &= columns[i - 1 - j] == columns[i + j];
                            }
                            if found_ref {
                                sum += i;
                                break;
                            }
                        }
                    }
                }
            }
            // next board
            rows.clear();
            columns.clear();
            if let Some(nextline) = it.next() {
                rows.push(nextline.unwrap().chars().collect());
                for c in &rows[0] {
                    columns.push(vec![*c; 1]);
                }
            } else {
                return sum;
            }
        } else {
            rows.push(line.chars().collect());
            for (i, c) in rows[rows.len() - 1].iter().enumerate() {
                columns[i].push(*c);
            }
        }
    }
    sum
}

fn part2(reader: BufReader<File>) -> usize {
    fn almost_eq(v1: &Vec<char>, v2: &Vec<char>, mistakes: &mut u32) -> bool {
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                *mistakes += 1;
            }
        }
        mistakes <= &mut 1
    }

    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut sum: usize = 0;
    let mut it = reader.lines();
    while let Some(line) = it.next() {
        let line = line.unwrap();
        if line.is_empty() {
            if rows.len() > 0 {
                let mut mistakes;
                let mut found_ref = false;
                for i in 1..rows.len() {
                    mistakes = 0;
                    if almost_eq(&rows[i - 1], &rows[i], &mut mistakes) {
                        found_ref = true;
                        for j in 0..(cmp::min(i, rows.len() - i) - 1) {
                            found_ref &=
                                almost_eq(&rows[i - 2 - j], &rows[i + j + 1], &mut mistakes);
                        }
                        if found_ref && mistakes == 1 {
                            sum += 100 * i;
                            break;
                        } else {
                            found_ref = false;
                        }
                        
                    }
                }
                if !found_ref {
                    for i in 1..columns.len() {
                        mistakes = 0;
                        if almost_eq(&columns[i - 1], &columns[i], &mut mistakes) {
                            found_ref = true;
                            for j in 0..(cmp::min(i, columns.len() - i) - 1) {
                                found_ref &= almost_eq(
                                    &columns[i - 2 - j],
                                    &columns[i + j + 1],
                                    &mut mistakes,
                                );
                            }
                            if found_ref && mistakes == 1 {
                                sum += i;
                                break;
                            }
                        }
                    }
                }
            }
            // next board
            rows.clear();
            columns.clear();
            if let Some(nextline) = it.next() {
                rows.push(nextline.unwrap().chars().collect());
                for c in &rows[0] {
                    columns.push(vec![*c; 1]);
                }
            } else {
                return sum;
            }
        } else {
            rows.push(line.chars().collect());
            for (i, c) in rows[rows.len() - 1].iter().enumerate() {
                columns[i].push(*c);
            }
        }
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("/home/vesa/code/rust/adventOfCode/input/day13.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
