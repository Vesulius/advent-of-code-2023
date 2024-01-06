use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part2(reader: BufReader<File>) -> i64 {
    let mut lines: Vec<Vec<char>> = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        let mut new_line: Vec<char> = Vec::new();
        let mut found = false;
        for (x, c) in line.unwrap().chars().enumerate() {
            println!("y{} x{}", y, x);
            if c == '#' {
                found = true;
            }
            new_line.push(c);
        }
        if !found {
            for i in 0..new_line.len() {
                new_line[i] = '-';
            }
        }

        lines.push(new_line);
    }

    let mut empty_columns = 0;
    let mut empty_rows;
    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    for x in 0..lines[0].len() {
        let mut found = false;
        empty_rows = 0;
        for y in 0..lines.len() {
            if lines[y][x] == '-' {
                empty_rows += 1;
            }
            if lines[y][x] == '#' {
                println!("empty rows{} col{}", empty_rows, empty_columns);
                galaxies.push((
                    y as i64 + empty_rows * 1000000 - empty_rows,
                    x as i64 + empty_columns * 1000000 - empty_columns,
                ));
                found = true;
            }
        }
        if !found {
            empty_columns += 1;
        }
    }
    println!("galaxies {:?}", galaxies);
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }
    sum
}

fn part1(reader: BufReader<File>) -> i32 {
    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    let mut lines: Vec<Vec<char>> = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        let mut new_line: Vec<char> = Vec::new();
        let mut found = false;
        for (x, c) in line.unwrap().chars().enumerate() {
            println!("y{} x{}", y, x);
            if c == '#' {
                found = true;
            }
            new_line.push(c);
        }
        if !found {
            lines.push(new_line.clone());
        }
        lines.push(new_line);
    }
    let mut empty_columns = 0;
    for x in 0..lines[0].len() {
        let mut found = false;
        for y in 0..lines.len() {
            if lines[y][x] == '#' {
                galaxies.push((y as i32, x as i32 + empty_columns));
                found = true;
            }
        }
        if !found {
            empty_columns += 1;
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("/home/vesa/code/rust/adventOfCode/input/day11.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
