use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn hash(s: &str) -> u32 {
    let mut hash = 0;
    for c in s.chars() {
        hash += (c as u8) as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn part1(reader: BufReader<File>) -> u64 {
    let s = reader.lines().next().unwrap().unwrap();
    let words: Vec<&str> = s.split(',').collect();

    let mut sum = 0;
    for w in words {
        sum += hash(w) as u64;
    }
    sum
}

fn part2(reader: BufReader<File>) -> usize {
    let mut map: Vec<LinkedList<(&str, u32)>> = vec![LinkedList::new(); 256];
    let s = reader.lines().next().unwrap().unwrap();
    let words: Vec<&str> = s.split(',').collect();
    for w in words {
        for (i, c) in w.chars().enumerate() {
            if c == '=' {
                let h = hash(&w[0..i]) as usize;
                let foc_len = w[i + 1..w.len()].parse::<u32>().unwrap();
                let label = &w[0..i];

                let mut contains = false;
                for lens in map[h].iter_mut() {
                    if lens.0 == label {
                        lens.1 = foc_len;
                        contains = true;
                        break;
                    }
                }

                if !contains {
                    map[h].push_back((&w[0..i], foc_len));
                }
                break;
            } else if c == '-' {
                let h = hash(&w[0..i]) as usize;
                let label = &w[0..i];

                let mut ind: i32 = 0;
                let mut contains = false;
                for (l, _) in map[h].iter() {
                    if l == &label {
                        contains = true;
                        break;
                    }
                    ind += 1;
                }

                if contains {
                    let mut split_list = map[h].split_off(ind as usize);
                    split_list.pop_front();
                    map[h].append(&mut split_list);
                }
                break;
            }
        }
        // println!("\nWORD: {}", w);
        // for i in 0..256 {
        //     if !map[i].is_empty() {
        //         println!("{}", i);
        //         for val in map[i].iter() {
        //             println!("\t {:?}", val);
        //         }
        //     }
        // }
    }
    let mut sum = 0;
    for i in 0..256 {
        if !map[i].is_empty() {
            for (j, val) in map[i].iter().enumerate() {
               sum += (i + 1) * (j + 1) * val.1 as usize; 
            }
        }
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("/home/vesa/code/rust/adventOfCode/input/day15.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
