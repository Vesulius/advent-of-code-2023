use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1(reader: BufReader<File>) -> u32 {
    let mut sum = 0;

    fn loob(
        springs: &Vec<char>,
        nums: &Vec<u32>,
        next: usize,
        n: u32,
        skipped: usize,
        choises: Vec<char>,
    ) -> u32 {
        let mut n = n;
        let mut next = next;
        let mut ret = 0;
        let mut skipped = skipped;
        let mut choises = choises.clone();
        for c in springs.iter().skip(skipped) {
            skipped += 1;
            // println!(
            //     "{}=={} {} in {:?}",
            //     next,
            //     n,
            //     c,
            //     springs.clone().iter().skip(skipped).collect::<Vec<&char>>()
            // );
            match c {
                '#' => {
                    choises.push('#');
                    n += 1;
                }
                '.' => {
                    choises.push('.');
                    if n > 0 {
                        if next == nums.len() || nums[next] != n {
                            return 0;
                        } else {
                            next += 1;
                            n = 0;
                        }
                    }
                }
                '?' => {
                    // case #
                    let mut choises1 = choises.clone();
                    choises1.push('#');

                    ret += loob(springs, nums, next, n + 1, skipped, choises1);

                    let mut choises2 = choises.clone();
                    choises2.push('.');

                    // case .
                    if n > 0 {
                        if next == nums.len() || nums[next] != n {
                            return ret;
                        } else {
                            ret += loob(springs, nums, next + 1, 0, skipped, choises2);
                        }
                    } else {
                        ret += loob(springs, nums, next, 0, skipped, choises2);
                    }
                    return ret;
                }
                _ => {}
            }
        }
        if (n == 0 && next != nums.len())
            || n > 0 && (next == nums.len() || nums[next] != n || next != nums.len() - 1)
        {
 
            return 0;
        }

        1
    }

    reader.lines().map_while(Result::ok).for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs: Vec<char> = parts[0].chars().collect();
        let nums: Vec<u32> = parts[1].split(',').map(|n| n.parse().unwrap()).collect();
        println!("line {}", line);
        let ret = loob(&springs, &nums, 0, 0, 0, Vec::new());
        println!("\ntotal: {}\n", ret);
        sum += ret;
    });
    sum
}

fn part2(reader: BufReader<File>) -> u64 {
    let mut sum: u64 = 0;
    let cache: HashMap<(u32, u32, u32), u64> = HashMap::new();
    fn loob(
        cache: &mut HashMap<(u32, u32, u32), u64>,
        springs: &Vec<char>,
        nums: &Vec<u32>,
        next: usize,
        n: u32,
        skipped: usize,
    ) -> u64 {
        let mut n = n;
        let mut next = next;
        let mut ret = 0;
        let mut skipped = skipped;
        for c in springs.iter().skip(skipped) {
            skipped += 1;
            match c {
                '#' => {
                    n += 1;
                }
                '.' => {
                    if n > 0 {
                        if next == nums.len() || nums[next] != n {
                            return 0;
                        } else {
                            next += 1;
                            n = 0;
                        }
                    }
                }
                '?' => {
                    // case #

                    if let Some(cached_val) = cache.get(&(next as u32, n + 1, skipped as u32)) {
                        ret += cached_val;
                    } else {
                        let val = loob(cache, springs, nums, next, n + 1, skipped);
                        cache.insert((next as u32, n + 1, skipped as u32), val);
                        ret += val;
                    }


                    // case .
                    if n > 0 {
                        if next == nums.len() || nums[next] != n {
                            return ret;
                        } else {
                            if let Some(cached_val) =
                                cache.get(&(next as u32 + 1, 0, skipped as u32))
                            {
                                ret += cached_val;
                            } else {
                                let val = loob(
                                    cache,
                                    springs,
                                    nums,
                                    next + 1,
                                    0,
                                    skipped
                                );
                                cache.insert((next as u32 + 1, 0, skipped as u32), val);
                                ret += val;
                            }
                        }
                    } else {
                        if let Some(cached_val) = cache.get(&(next as u32, 0, skipped as u32)) {
                            ret += cached_val;
                        } else {
                            let val = loob(cache, springs, nums, next, 0, skipped);
                            cache.insert((next as u32, 0, skipped as u32), val);
                            ret += val;
                        }
                    }
                    return ret;
                }
                _ => {}
            }
        }
        if (n == 0 && next != nums.len())
            || n > 0 && (next == nums.len() || nums[next] != n || next != nums.len() - 1)
        {
            return 0;
        }
        1
    }

    reader.lines().map_while(Result::ok).for_each(|line| {
        let (springs, nums) = line.split_once(' ').unwrap();
        let springs: Vec<char> = std::iter::once(springs)
            .cycle()
            .take(5)
            .join("?")
            .chars()
            .collect();
        let nums = nums
            .split(',')
            .map(|number| number.parse::<u32>().unwrap())
            .collect_vec();
        let nums_len = nums.len();
        let nums = nums.into_iter().cycle().take(5 * nums_len).collect_vec();
        // println!("{} {:?}", springs.iter().collect::<String>(), nums);
        let ret = loob(&mut cache.clone(), &springs, &nums, 0, 0, 0);
        // println!("\ntotal: {}\n", ret);
        sum += ret;
    });
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day12.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
