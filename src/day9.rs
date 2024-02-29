use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part2(reader: BufReader<File>) -> i32 {
    let nums: Vec<Vec<i32>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .rev()
                .collect()
        })
        .collect();

    let mut all_zeroes;
    let mut diff;
    let mut sum = 0;
    let mut row_count;
    for mut row in nums {
        println!("{:?}", row);
        row_count = 0;
        all_zeroes = false;
        while !all_zeroes {
            row_count += 1;
            all_zeroes = true;
            for i in 0..(row.len() - row_count) {
                diff = row[i + 1] - row[i];
                all_zeroes = all_zeroes && diff == 0;
                row[i] = diff;
            }
            println!("{} {:?}", row_count, row);
        }
        let mut cum = 0;
        for n in row.iter().skip(row.len() - row_count) {
            cum += n;
        }

        sum += cum;
    }
    sum
}

fn part1(reader: BufReader<File>) -> i32 {
    let nums: Vec<Vec<i32>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let mut all_zeroes;
    let mut diff;
    let mut sum = 0;
    let mut row_count;
    for mut row in nums {
        println!("{:?}", row);
        row_count = 0;
        all_zeroes = false;
        while !all_zeroes {
            row_count += 1;
            all_zeroes = true;
            for i in 0..(row.len() - row_count) {
                diff = row[i + 1] - row[i];
                all_zeroes = all_zeroes && diff == 0;
                row[i] = diff;
            }
            println!("{} {:?}", row_count, row);
        }
        let mut cum = 0;
        for n in row.iter().skip(row.len() - row_count) {
            cum += n;
        }

        sum += cum;
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day9.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
