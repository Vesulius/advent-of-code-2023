use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::Wrapping;
// fn part2(reader: BufReader<File>) -> u32 {
//     let mut sum: u32 = 0;

//     let lines: Vec<Vec<i32>> = reader
//         .lines()
//         .filter_map(Result::ok)
//         .map(|line| {
//             let mut newLine: Vec<i32> = Vec::new();
//             let mut num: u32 = 0;
//             let mut numLen = 0;
//             for c in line.chars() {
//                 if c.is_digit(10) {
//                     num *= 10;
//                     num += c.to_digit(10).unwrap();
//                     numLen += 1;
//                 } else {
//                     if numLen > 0 {
//                         for _ in 0..numLen {
//                             newLine.push(num as i32);
//                         }
//                         numLen = 0;
//                         num = 0;
//                     }
//                     if c == '*' {
//                         newLine.push(-1);
//                     } else {
//                         newLine.push(0);
//                     }
//                 }
//             }
//             for _ in 0..numLen {
//                 newLine.push(num as i32);
//             }
//             newLine
//         })
//         .collect();

//     for row in lines.iter() {
//         for v in row.iter() {
//             print!("\t{}", v);
//         }
//         print!("\n");
//     }
//     let mut num = 1;
//     let mut found = 0;
//     for (y, row) in lines.iter().enumerate() {
//         for (x, item) in row.iter().enumerate() {
//             // check for all of the directions. If there are numbers other than 0 multiply them and sum all the multiplications
//             if item == &-1 {
//                 if x as i32 - 1 >= 0 && row[x - 1] > 0 {
//                     num *= row[x - 1];
//                     print!(" {}", num);
//                     found += 1;
//                     if y as i32 - 1 >= 0 && lines[y - 1][x - 1] > 0 {
//                         num *= lines[y - 1][x - 1];
//                         print!(" {}", num);
//                         found += 1;
//                         if lines[y - 1][x] > 0 {
//                             num *= lines[y - 1][x];
//                             print!(" {}", num);
//                             found += 1;
//                         }
//                     }
//                     if y + 1 < row.len() && lines[y + 1][x - 1] > 0 {
//                         // asumming that rows have the same len
//                         num *= lines[y + 1][x - 1];
//                         print!(" {}", num);
//                         found += 1;
//                         if lines[y + 1][x] > 0 {
//                             num *= lines[y + 1][x];
//                             print!(" {}", num);
//                             found += 1;
//                         }
//                     }
//                 }

//                 if x + 1 < row.len() && row[x + 1] > 0 {
//                     num *= row[x + 1];
//                     print!(" {}", num);
//                     found += 1;
//                     if y as i32 - 1 >= 0 && lines[y - 1][x + 1] > 0 {
//                         num *= lines[y - 1][x + 1];
//                         print!(" {}", num);
//                         found += 1;
//                     }
//                     if y + 1 < row.len() && lines[y + 1][x + 1] > 0 {
//                         // asumming that rows have the same len
//                         num *= lines[y + 1][x + 1];
//                         print!(" {}", num);
//                         found += 1;
//                     }
//                 }
//                 if found == 2 {
//                     sum += num as u32;
//                 }
//                 found = 0;
//                 num = 1;
//             }
//         }
//     }
//     sum
// }

// fn part2(reader: BufReader<File>) -> u32 {
//     let mut sum = 0;

//     let mut map: HashMap<u32, (u32, u32)> = HashMap::new();
//     let lines: Vec<Vec<char>> = reader
//         .lines()
//         .filter_map(Result::ok)
//         .map(|line| line.chars().collect())
//         .collect();

//     for (y, line) in lines.iter().enumerate() {
//         let mut num = 0;
//         let mut stars: Vec<u32> = Vec::new();
//         for (x, char) in line.iter().enumerate() {
//             if char.is_numeric() {
//                 if num == 0 {
//                     if x as i32 - 1 >= 0 {
//                         if y + 1 < lines.len() && lines[y + 1][x - 1] == '*' {
//                             stars.push((y as u32 + 1) * 10000 + (x as u32 - 1));
//                         }
//                         if y as i32 - 1 >= 0 && lines[y - 1][x - 1] == '*' {
//                             stars.push((y as u32 - 1) * 10000 + (x as u32 - 1));
//                         }
//                         if lines[y][x - 1] == '*' {
//                             stars.push((y as u32) * 10000 + (x as u32 - 1));
//                         }
//                     }
//                 }
//                 num *= 10;
//                 num += char.to_digit(10).unwrap();
//                 if (y as i32 - 1 >= 0 && lines[y - 1][x] == '*') {
//                     stars.push((y as u32 - 1) * 10000 + x as u32);
//                 }
//                 if (y + 1 < lines.len() && lines[y + 1][x] == '*') {
//                     stars.push((y as u32 + 1) * 10000 + x as u32);
//                 }
//             } else {
//                 if num != 0 {
//                     if lines[y][x] == '*' {
//                         stars.push((y as u32) * 10000 + x as u32);
//                     }
//                     if (y + 1 < lines.len() && lines[y + 1][x] == '*') {
//                         stars.push((y as u32 + 1) * 10000 + x as u32);
//                     }
//                     if (y as i32 - 1 >= 0 && lines[y - 1][x] == '*') {
//                         stars.push((y as u32 - 1) * 10000 + x as u32);
//                     }
//                     for star in &stars {
//                         match map.get(&star) {
//                             Some(&(n, val)) => {
//                                 map.insert(*star, (n + 1, val * num));
//                             }
//                             None => {
//                                 map.insert(*star, (1, num));
//                             }
//                         }
//                     }
//                     stars.clear();
//                     num = 0;
//                 }
//             }
//             for star in &stars {
//                 match map.get(&star) {
//                     Some(&(n, val)) => {
//                         print!(" {}", star);
//                         print!(" {}", n);
//                         print!(" {}\n", val);
//                         map.insert(*star, (n + 1, val * num));
//                     }
//                     None => {
//                         map.insert(*star, (1, num));
//                     }
//                 }
//             }
//         }
//     }
//     for (n, num) in map.values() {
//         if n == &2 {
//             sum += num;
//         }
//     }
//     sum
// }

// fn part2(reader: BufReader<File>) -> u32 {
//     let mut sum = 0;

//     let mut map: HashMap<u32, u32> = HashMap::new();
//     let mut starN = 0;
//     let mut lines: Vec<Vec<char>> = Vec::new();
//     for (y, line) in reader.lines().enumerate() {
//         let mut newLine: Vec<char> = Vec::new();
//         match line {
//             Ok(line_content) => {
//                 for (x, c) in line_content.chars().enumerate() {
//                     if c == '*' {
//                         map.insert(10000 * (y - 1) as u32 + x as u32, starN);
//                         map.insert(10000 * (y + 1) as u32 + x as u32, starN);
//                         map.insert(10000 * (y - 1) as u32 + (x - 1) as u32, starN);
//                         map.insert(10000 * (y + 1) as u32 + (x - 1) as u32, starN);
//                         map.insert(10000 * (y + 1) as u32 + (x + 1) as u32, starN);
//                         map.insert(10000 * (y - 1) as u32 + (x + 1) as u32, starN);
//                         map.insert(10000 * y as u32 + (x - 1) as u32, starN);
//                         map.insert(10000 * y as u32 + (x + 1) as u32, starN);
//                         starN += 1;
//                     }
//                     newLine.push(c);
//                 }
//                 lines.push(newLine);
//             }
//             Err(err) => eprintln!("Error reading line: {}", err),
//         }
//     }
//     for (y, row) in lines.iter().enumerate() {
//         let mut num = 0;
//         let mut set: HashSet<u32> = HashSet::new();
//         for (x, c) in row.iter().enumerate() {
//             if c.is_numeric() {
//                 num *= 10;
//                 num += c.to_digit(10).unwrap();
//                 let hash = y as u32 * 10000 + x as u32;
//                 match map.get(&hash) {
//                     Some(n) => {
//                         set.insert(*n);
//                     }
//                     None => {}
//                 }
//             } else {
//                 if num != 0 {}
//             }
//         }
//     }

//     sum
// }

fn part2(reader: BufReader<File>) -> u32 {
    let mut sum = 0;

    let mut id: u8 = 0;
    let lines: Vec<Vec<(i32, u8)>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut newLine: Vec<(i32, u8)> = Vec::new();
            let mut num: i32 = 0;
            let mut numLen = 0;
            for c in line.chars() {
                if c.is_ascii_digit() {
                    num *= 10;
                    num += c.to_digit(10).unwrap() as i32;
                    numLen += 1;
                } else {
                    if numLen > 0 {
                        for _ in 0..numLen {
                            newLine.push((num, id));
                        }
                        id = (Wrapping(id) + Wrapping(1)).0;
                        numLen = 0;
                        num = 0;
                    }
                    if c == '*' {
                        newLine.push((-1, 0));
                    } else {
                        newLine.push((0, 0));
                    }
                }
            }
            for _ in 0..numLen {
                newLine.push((num, id));
            }
            id = (Wrapping(id) + Wrapping(1)).0;
            newLine
        })
        .collect();
    for (y, line) in lines.iter().enumerate() {
        for (x, (curr, _)) in line.iter().enumerate() {
            print!("{}\t", curr);
        }
        println!();
    }
    let mut set: HashSet<u8> = HashSet::new();
    let mut num = 1;
    for (y, line) in lines.iter().enumerate() {
        for (x, (curr, _)) in line.iter().enumerate() {
            if curr == &-1 {
                if y > 0 {
                    if lines[y - 1][x].0 > 0 && !set.contains(&lines[y - 1][x].1) {
                        num *= lines[y - 1][x].0;
                        set.insert(lines[y - 1][x].1);
                    }
                    if x > 0 && lines[y - 1][x - 1].0 > 0 && !set.contains(&lines[y - 1][x - 1].1) {
                        num *= lines[y - 1][x - 1].0;
                        set.insert(lines[y - 1][x - 1].1);
                    }
                    if x + 1 < line.len()
                        && lines[y - 1][x + 1].0 > 0
                        && !set.contains(&lines[y - 1][x + 1].1)
                    {
                        num *= lines[y - 1][x + 1].0;
                        set.insert(lines[y - 1][x + 1].1);
                    }
                }
                if y + 1 < line.len() {
                    if lines[y + 1][x].0 > 0 && !set.contains(&lines[y + 1][x].1) {
                        num *= lines[y + 1][x].0;
                        set.insert(lines[y + 1][x].1);
                    }
                    if x > 0 && lines[y + 1][x - 1].0 > 0 && !set.contains(&lines[y + 1][x - 1].1) {
                        num *= lines[y + 1][x - 1].0;
                        set.insert(lines[y + 1][x - 1].1);
                    }
                    if x + 1 < line.len()
                        && lines[y + 1][x + 1].0 > 0
                        && !set.contains(&lines[y + 1][x + 1].1)
                    {
                        num *= lines[y + 1][x + 1].0;
                        set.insert(lines[y + 1][x + 1].1);
                    }
                }
                if x > 0 && lines[y][x - 1].0 > 0 && !set.contains(&lines[y][x - 1].1) {
                    num *= lines[y][x - 1].0;
                    set.insert(lines[y][x - 1].1);
                }
                if x + 1 < line.len() && lines[y][x + 1].0 > 0 && !set.contains(&lines[y][x + 1].1)
                {
                    num *= lines[y][x + 1].0;
                    set.insert(lines[y][x + 1].1);
                }
            }
            if set.len() == 2 {
                sum += num as u32;
            }
            num = 1;
            set.clear();
        }
    }
    sum
}

fn part1(reader: BufReader<File>) -> u32 {
    let mut sum = 0;

    let lines: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    for (y, line) in lines.iter().enumerate() {
        let mut num = 0;
        let mut used = false;
        for (x, char) in line.iter().enumerate() {
            if char.is_numeric() {
                if num == 0 {
                    used = x as i32 - 1 >= 0
                        && ((y + 1 < lines.len() && lines[y + 1][x - 1] != '.')
                            || (y as i32 - 1 >= 0 && lines[y - 1][x - 1] != '.')
                            || (lines[y][x - 1] != '.'));
                }
                num *= 10;
                num += char.to_digit(10).unwrap();
                used = used
                    || (y as i32 - 1 >= 0 && lines[y - 1][x] != '.')
                    || (y + 1 < lines.len() && lines[y + 1][x] != '.');
            } else {
                if num != 0 {
                    if used
                        || (lines[y][x] != '.')
                        || (y + 1 < lines.len() && lines[y + 1][x] != '.')
                        || (y as i32 - 1 >= 0 && lines[y - 1][x] != '.')
                    {
                        sum += num;
                    }
                }
                num = 0;
                used = false;
            }
        }
        if used {
            sum += num;
        }
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("../input/day3.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
