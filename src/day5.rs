use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn part2(mut reader: BufReader<File>) -> u128 {
    let mut seeds: Vec<(u128, u128)> = reader
        .by_ref()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u128>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    println!("Start {:?}", seeds);
    let mut edited: Vec<bool> = vec![false; seeds.len()];
    for line in reader.lines().map_while(Result::ok).skip(1) {
        if !line.is_empty() && line.chars().next().unwrap().is_ascii_digit() {
            let parts: Vec<u128> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let output = parts[0];
            let input = parts[1];
            let range = parts[2];
            print!("seeds");
            for i in 0..seeds.len() {
                if !edited[i] {
                    print!(" {}-{}", seeds[i].0, seeds[i].0 + seeds[i].1);
                }
            }
            print!("\n");
            println!(
                "input {}-{} output {}-{}",
                input,
                input + range,
                output,
                output + range
            );
            for i in 0..seeds.len() {
                if !edited[i] && seeds[i].0 < input + range && seeds[i].0 + seeds[i].1 > input {
                    if seeds[i].0 < input || seeds[i].0 + seeds[i].1 > input + range {
                        println!("Splitting seeds {}-{}", seeds[i].0, seeds[i].1);
                    }
                    if seeds[i].0 < input {
                        seeds.push((seeds[i].0, input - seeds[i].0));
                        println!("Split first {} {}", seeds[i].0, input - seeds[i].0);
                        seeds[i].1 -= input - seeds[i].0;
                        seeds[i].0 = input;
                        edited.push(false);
                    }
                    if seeds[i].0 + seeds[i].1 > input + range {
                        seeds.push((
                            seeds[i].0 + range,
                            seeds[i].0 + seeds[i].1 - (input + range),
                        ));
                        println!(
                            "Split second {} {}",
                            seeds[i].0 + range,
                            seeds[i].0 + seeds[i].1 - (input + range)
                        );
                        seeds[i].1 -= seeds[i].0 + seeds[i].1 - (input + range);
                        edited.push(false);
                    }
                    seeds[i].0 = output + (seeds[i].0 - input);
                    edited[i] = true;
                }
            }
        } else if line.is_empty() {
            for i in 0..edited.len() {
                edited[i] = false;
            }
            println!("Seeds {:?}\n", seeds);
        } else {
            println!("\n{}\n", line);
        }

    }
    println!("Final seeds {:?}\n", seeds);
    seeds.iter().map(|s| s.0).min().unwrap()
}

fn part1(mut reader: BufReader<File>) -> u64 {
    let mut seeds: Vec<u64> = reader
        .by_ref()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Seeds {:?}", seeds);

    let mut edited: Vec<bool> = vec![false; seeds.len()];
    for line in reader.lines().map_while(Result::ok).skip(1) {
        if line.chars().next().unwrap().is_ascii_digit() {
            let parts: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let output = parts[0];
            let input = parts[1];
            let range = parts[2];
            for i in 0..seeds.len() {
                if !edited[i] && seeds[i] >= input && seeds[i] < input + range {
                    seeds[i] = output + (seeds[i] - input);
                    edited[i] = true;
                }
            }
        } else if line.is_empty() {
            for i in 0..edited.len() {
                edited[i] = false;
            }
            println!("Seeds {:?}", seeds);
        }
    }
    *seeds.iter().min().unwrap()
}

pub fn run(run_part_2: bool) {
    let file = File::open("../input/day5.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
// 24261545
