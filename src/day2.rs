use std::fs::File;
use std::io::{self, BufRead};
use std::cmp;

fn read_file_lines(file_path: &str) -> io::Result<u32> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for (line_n, line) in reader.lines().enumerate() {
        match line {
            Ok(line_content) => {
                let words: Vec<&str> = line_content.split_whitespace().collect();
                let mut possible = true;
                for (i, word) in words.iter().enumerate() {
                    match word.parse::<i32>() {
                        Ok(dice) => {
                            if (dice > 12 && words[i + 1].chars().nth(0).unwrap() == 'r')
                                || (dice > 13 && words[i + 1].chars().nth(0).unwrap() == 'g')
                                || (dice > 14 && words[i + 1].chars().nth(0).unwrap() == 'b')
                            {
                                possible = false;
                                break;
                            }
                        }
                        Err(_) => continue,
                    }
                }
                if possible {
                    sum += (line_n + 1) as u32;
                }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }
    Ok(sum)
}

fn read_file_lines2(file_path: &str) -> io::Result<i32> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                let words: Vec<&str> = line_content.split_whitespace().collect();
                let mut min_r = 0;
                let mut min_g = 0;
                let mut min_b = 0;
                for (i, word) in words.iter().enumerate() {
                    match word.parse::<i32>() {
                        Ok(dice) => match words[i + 1].chars().nth(0).unwrap() {
                            'r' => min_r = cmp::max(min_r, dice),
                            'g' => min_g = cmp::max(min_g, dice),
                            'b' => min_b = cmp::max(min_b, dice),
                            _ => continue,
                        },
                        Err(_) => continue,
                    }
                }
                sum += min_r * min_g * min_b;
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }
    Ok(sum)
}

pub fn run(run_part_2: bool) {
    if run_part_2 {
        match read_file_lines2("../input/day2.txt") {
            Ok(sum) => {
                println!("{}", sum);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    } else {
        match read_file_lines("../input/day2.txt") {
            Ok(sum) => {
                println!("{}", sum);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
