use std::fs::File;
use std::io::{self, BufRead};

fn read_file_lines2(file_path: &str) -> io::Result<u32> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let num_strings = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;

    for line in reader.lines() {
        let mut first = 0;
        let mut second = 0;
        match line {
            Ok(line_content) => {
                let mut i = 0;
                let mut c;
                while i < line_content.len() {
                    c = line_content.chars().nth(i).unwrap();
                    if c.is_digit(10) {
                        if first == 0 {
                            first = 10 * c.to_digit(10).unwrap();
                            second = c.to_digit(10).unwrap();
                        } else {
                            second = c.to_digit(10).unwrap();
                        }
                    } else {
                        let mut found;
                        for (k, numString) in num_strings.iter().enumerate() {
                            found = true;
                            for (j, char) in numString.chars().enumerate() {
                                c = line_content.chars().nth(i).unwrap();
                                i += 1;
                                print!(" i:{} {} == {}", i, char, c);
                                if c != char {
                                    i -= j + 1;
                                    found = false;
                                    break;
                                } else if i >= line_content.len() {
                                    found = j + 1 == numString.len();
                                    break;
                                }
                            }
                            if found {
                                if first == 0 {
                                    first = 10 * (k + 1) as u32;
                                    second = (k + 1) as u32;
                                } else {
                                    second = (k + 1) as u32;
                                }
                                i -= num_strings[k].len();
                                break;
                            } else if i >= line_content.len() {
                                break;
                            }
                        }
                    }
                    i += 1;
                }
                sum += first;
                sum += second;
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }
    Ok(sum)
}

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

pub fn run(run_part_2: bool) {
    if run_part_2 {
        match read_file_lines2("../input/day1.txt") {
            Ok(sum) => {
                println!("{}", sum);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    } else {
        match read_file_lines("../input/day1.txt") {
            Ok(sum) => {
                println!("{}", sum);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
