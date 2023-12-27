use std::fs::File;
use std::io::{self, BufRead};

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
                            if (dice > 12 && words[i + 1].chars().nth(0).unwrap() == 'r') || (dice > 13 && words[i + 1].chars().nth(0).unwrap() == 'g') || (dice > 14 && words[i + 1].chars().nth(0).unwrap() == 'b') {
                                possible = false;
                                break;
                            }
                        },
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

pub fn run() {
    match read_file_lines("../input/day2.txt") {
        Ok(sum) => {
            println!("{}", sum);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
