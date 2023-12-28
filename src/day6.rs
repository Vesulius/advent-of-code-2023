use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn distance(n: u32, time: u32) -> u32 {
    n * (time - n)
}

fn part1(reader: BufReader<File>) -> u32 {
    let mut it = reader.lines();
    let times: Vec<u32> = it
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let records: Vec<u32> = it
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut product = 1;
    for i in 0..times.len() {
        let mut sum: u32 = 0;
        for j in 1..times[i] {
            println!("time{} j{} dist{} ", times[i], j, distance(j, times[i]));
            if records[i] < distance(j, times[i]) {
                sum += 1;
            }
        }
        println!("{}", sum);
        product *= sum;
    }
    product
}

fn part2(reader: BufReader<File>) -> u64 {
    let mut it = reader.lines();
    let time: f64 = it.next().unwrap().unwrap().replace([' ', '\t', '\n'], "")[5..]
        .parse()
        .unwrap();
    let record: f64 = it.next().unwrap().unwrap().replace([' ', '\t', '\n'], "")[9..]
        .parse()
        .unwrap();

    // we know that the function of distance when button is pressed is updiside down parabola
    // lets solve: -n² + time*n - record > 0
    let max = (-time - ((time * time) - (4.0 * (-1.0) * -record)).sqrt()) / (2.0 * (-1.0));
    let min = (-time + ((time * time) - (4.0 * (-1.0) * -record)).sqrt()) / (2.0 * (-1.0));
    max.floor() as u64 - min.floor() as u64
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day6.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}