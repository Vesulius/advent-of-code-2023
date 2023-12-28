use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1(reader: BufReader<File>) -> u32 {
    let n_of_guesses = 10;
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut sum: u32 = 0;
            let guesses: Vec<&str> = parts.iter().skip(2).take(n_of_guesses).cloned().collect();
            for i in (n_of_guesses + 2)..parts.len() {
                if guesses.contains(&parts[i]) {
                    if sum == 0 {
                        sum = 1;
                    } else {
                        sum *= 2;
                    }
                }
            }
            print!("{} : {} : {}\n", line, guesses.join(" "), sum);
            sum
        })
        .sum()
}

fn part2(reader: BufReader<File>) -> u32 {
    let n_of_guesses = 10;
    let mut bonus: Vec<i32> = Vec::new();
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut wins: u32 = 0;
            let guesses: Vec<&str> = parts.iter().skip(2).take(n_of_guesses).cloned().collect();
            for i in (n_of_guesses + 2)..parts.len() {
                if guesses.contains(&parts[i]) {
                    wins += 1;
                }
            }
            println!(
                "{} : {} : {} : {}",
                line,
                guesses.join(" "),
                wins,
                1 + bonus.len()
            );
            for _ in 0..1 + bonus.len() {
                bonus.push(wins as i32 + 1);
            }
            bonus.iter_mut().for_each(|b| *b -= 1);
            bonus.retain(|&b| b > 0);
            1 + bonus.len() as u32
        })
        .sum()
}

pub fn run(run_part_2: bool) {
    let file = File::open("../input/day4.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
