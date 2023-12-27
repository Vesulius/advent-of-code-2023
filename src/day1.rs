use std::fs::File;
use std::io::{self, BufRead};

fn read_file_lines(file_path: &str) -> io::Result<u32> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let mut num = 0;
        match line {
            Ok(line_content) => {
                for c in line_content.chars() {
                    if c.is_digit(10) {
                        num += 10 * c.to_digit(10).unwrap();
                        break;
                    }
                }
                for c in line_content.chars().rev() {
                    if c.is_digit(10) {
                        num += c.to_digit(10).unwrap();
                        break;
                    }
                }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
        sum += num;
    }

    Ok(sum)
}

pub fn run() {
    match read_file_lines("../input/day1.txt") {
        Ok(sum) => {
            println!("{}", sum);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
