use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part2(reader: BufReader<File>) -> u32 {
    let mut curr: (usize, usize) = (0, 0);
    let mut next: (usize, usize) = (0, 0);
    let mut lines: Vec<Vec<char>> = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        let mut new_line: Vec<char> = Vec::new();
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == 'S' {
                curr = (y, x);
            }
            new_line.push(c);
        }
        lines.push(new_line);
    }

    
    // find start

    let mut foundpath = false;

    // up
    if curr.0 > 0 {
        let nextchar = lines[curr.0 - 1][curr.1];
        match nextchar {
            '|' => {
                next = (curr.0 - 1, curr.1);
                foundpath = true;
            }
            'F' => {
                next = (curr.0 - 1, curr.1);
                foundpath = true;
            }
            '7' => {
                curr = (curr.0 - 1, curr.1);
                foundpath = true;
            }
            _ => {}
        }
    }

    // down
    if !foundpath && curr.0 + 1 < lines[0].len() {
        let nextchar = lines[curr.0 + 1][curr.1];
        match nextchar {
            '|' => {
                next = (curr.0 + 1, curr.1);
                foundpath = true;
            }
            'J' => {
                next = (curr.0 + 1, curr.1);
                foundpath = true;
            }
            'L' => {
                next = (curr.0 + 1, curr.1);
                foundpath = true;
            }
            _ => {}
        }
    }

    // left
    if !foundpath && curr.1 > 0 {
        let nextchar = lines[curr.0][curr.1 - 1];
        match nextchar {
            '-' => {
                next = (curr.0, curr.1 - 1);
                foundpath = true;
            }
            'L' => {
                curr = (curr.0, curr.1 - 1);
                foundpath = true;
            }
            'F' => {
                next = (curr.0, curr.1 - 1);
                foundpath = true;
            }
            _ => {}
        }
    }

    // right
    if !foundpath && curr.1 + 1 < lines[0].len() {
        let nextchar = lines[curr.0][curr.1 + 1];
        match nextchar {
            '-' => {
                next = (curr.0, curr.1 + 1);
            }
            'J' => {
                next = (curr.0, curr.1 + 1);
            }
            'L' => {
                next = (curr.0, curr.1 + 1);
            }
            _ => {}
        }
    }

    println!("curr {:?}", curr);
    println!("next {:?}", next);

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    set.insert(curr);
    let mut counter = 1;
    loop {
        set.insert(next);
        let nextchar = lines[next.0][next.1];
        if nextchar == 'S' {
            break;
        }
        counter += 1;
        match nextchar {
            '-' => {
                if curr.1 < next.1 {
                    curr = next;
                    next = (next.0, next.1 + 1);
                } else {
                    curr = next;
                    next = (next.0, next.1 - 1);
                }
            }
            '|' => {
                if curr.0 < next.0 {
                    curr = next;
                    next = (next.0 + 1, next.1);
                } else {
                    curr = next;
                    next = (next.0 - 1, next.1);
                }
            }
            'L' => {
                if curr.0 < next.0 {
                    curr = next;
                    next = (next.0, next.1 + 1);
                } else {
                    curr = next;
                    next = (next.0 - 1, next.1);
                }
            }
            '7' => {
                if curr.0 > next.0 {
                    curr = next;
                    next = (next.0, next.1 - 1);
                } else {
                    curr = next;
                    next = (next.0 + 1, next.1);
                }
            }
            'J' => {
                if curr.0 < next.0 {
                    curr = next;
                    next = (next.0, next.1 - 1);
                } else {
                    curr = next;
                    next = (next.0 - 1, next.1);
                }
            }
            'F' => {
                if curr.0 > next.0 {
                    curr = next;
                    next = (next.0, next.1 + 1);
                } else {
                    curr = next;
                    next = (next.0 + 1, next.1);
                }
            }
            _ => {}
        }
    }

    lines[next.0][next.1] = 'J';

    let mut space = 0;
    let mut walls;
    let mut x;
    let mut from_up = true;
    for y in 0..lines.len() {
        walls = 0;
        x = 0;
        while x < lines[0].len() {
            if set.contains(&(y, x)) {
                if lines[y][x] == 'L' {
                    from_up = true;
                } else if lines[y][x] == 'F' {
                    from_up = false;
                } else if lines[y][x] == '-' {
                    
                } else if lines[y][x] == 'J' {
                    if !from_up {
                        walls += 1;
                    }
                } else if lines[y][x] == '7' {
                    if from_up {
                        walls += 1;
                    }
                } else {
                    walls += 1;
                }
            } else if walls % 2 != 0 {
                space += 1;
            }
            x += 1;
        }
    }
    println!("space {}", space);
    // let mut checked: HashSet<(i32, i32)> = HashSet::new();

    // fn loob(
    //     nates: (i32, i32),
    //     checked: &mut HashSet<(i32, i32)>,
    //     set: &HashSet<(usize, usize)>,
    //     lines_height: i32,
    //     lines_width: i32,
    // ) {
    //     // println!("new {:?}", nates);
    //     for y in -1..2 {
    //         for x in -1..2 {
    //             let loc_y: i32 = nates.0 + y;
    //             let loc_x: i32 = nates.1 + x;
    //             // println!("y{} x{}", loc_y, loc_x);
    //             if (y == 0 && x == 0)
    //                 || loc_y < 0
    //                 || loc_x < 0
    //                 || loc_y >= lines_height
    //                 || loc_x >= lines_width
    //                 || set.contains(&(loc_y as usize, loc_x as usize))
    //                 || checked.contains(&(loc_y, loc_x))
    //             {
    //                 continue;
    //             } else {
    //                 // println!("ADDED");
    //                 checked.insert((loc_y, loc_x));
    //                 loob((loc_y, loc_x), checked, set, lines_height, lines_width)
    //             }
    //         }
    //     }
    // }

    // loob(
    //     (0, 0),
    //     &mut checked,
    //     &set,
    //     lines.len() as i32,
    //     lines[0].len() as i32,
    // );

    // println!("checked len {}", checked.len());
    println!("counter {}", counter);

    space
}

fn part1(reader: BufReader<File>) -> u32 {
    let mut curr: (usize, usize) = (0, 0);
    let mut next: (usize, usize) = (0, 0);
    let mut lines: Vec<Vec<char>> = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        let mut new_line: Vec<char> = Vec::new();
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == 'S' {
                curr = (y, x);
            }
            new_line.push(c);
        }
        lines.push(new_line);
    }

    // find start

    let mut foundpath = false;

    // up
    if curr.0 > 0 {
        let nextchar = lines[curr.0 - 1][curr.1];
        match nextchar {
            '|' => {
                next = (curr.0 - 1, curr.1);
                foundpath = true;
            }
            'F' => {
                next = (curr.0 - 1, curr.1);
                foundpath = true;
            }
            '7' => {
                curr = (curr.0 - 1, curr.1);
                foundpath = true;
            }
            _ => {}
        }
    }

    // down
    if !foundpath && curr.0 + 1 < lines[0].len() {
        let nextchar = lines[curr.0 + 1][curr.1];
        match nextchar {
            '|' => {
                next = (curr.0 + 1, curr.1);
                foundpath = true;
            }
            'J' => {
                next = (curr.0 + 1, curr.1);
                foundpath = true;
            }
            'L' => {
                next = (curr.0 + 1, curr.1);
                foundpath = true;
            }
            _ => {}
        }
    }

    // left
    if !foundpath && curr.1 > 0 {
        let nextchar = lines[curr.0][curr.1 - 1];
        match nextchar {
            '-' => {
                next = (curr.0, curr.1 - 1);
                foundpath = true;
            }
            'L' => {
                curr = (curr.0, curr.1 - 1);
                foundpath = true;
            }
            'F' => {
                next = (curr.0, curr.1 - 1);
                foundpath = true;
            }
            _ => {}
        }
    }

    // right
    if !foundpath && curr.1 + 1 < lines[0].len() {
        let nextchar = lines[curr.0][curr.1 + 1];
        match nextchar {
            '-' => {
                next = (curr.0, curr.1 + 1);
            }
            'J' => {
                next = (curr.0, curr.1 + 1);
            }
            'L' => {
                next = (curr.0, curr.1 + 1);
            }
            _ => {}
        }
    }

    let mut counter = 1;
    loop {
        let nextchar = lines[next.0][next.1];
        println!("{} curr {:?} next {:?} {}", counter, curr, next, nextchar);
        if nextchar == 'S' {
            break;
        }
        counter += 1;
        match nextchar {
            '-' => {
                if curr.1 < next.1 {
                    curr = next;
                    next = (next.0, next.1 + 1);
                } else {
                    curr = next;
                    next = (next.0, next.1 - 1);
                }
            }
            '|' => {
                if curr.0 < next.0 {
                    curr = next;
                    next = (next.0 + 1, next.1);
                } else {
                    curr = next;
                    next = (next.0 - 1, next.1);
                }
            }
            'L' => {
                if curr.0 < next.0 {
                    curr = next;
                    next = (next.0, next.1 + 1);
                } else {
                    curr = next;
                    next = (next.0 - 1, next.1);
                }
            }
            '7' => {
                if curr.0 > next.0 {
                    curr = next;
                    next = (next.0, next.1 - 1);
                } else {
                    curr = next;
                    next = (next.0 + 1, next.1);
                }
            }
            'J' => {
                if curr.0 < next.0 {
                    curr = next;
                    next = (next.0, next.1 - 1);
                } else {
                    curr = next;
                    next = (next.0 - 1, next.1);
                }
            }
            'F' => {
                if curr.0 > next.0 {
                    curr = next;
                    next = (next.0, next.1 + 1);
                } else {
                    curr = next;
                    next = (next.0 + 1, next.1);
                }
            }
            _ => {}
        }
    }
    println!("counter {}", counter);
    counter / 2
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day10.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
