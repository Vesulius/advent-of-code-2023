use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(PartialEq, Clone, Copy)]
enum From {
    Above,
    Below,
    Left,
    Right,
}

fn part1(reader: BufReader<File>) -> u32 {
    let map: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| l.chars().collect())
        .collect_vec();

    let width = map[0].len();
    let height = map.len();

    let mut start = (0, 0);
    for (i, c) in map.first().unwrap().iter().enumerate() {
        if c == &'.' {
            start.1 = i;
            break;
        }
    }

    let mut next_pos: VecDeque<(i32, i32, From)> = VecDeque::new();
    let mut weights: Vec<Vec<u32>> = vec![vec![0; width]; height];
    next_pos.push_back((start.0 as i32, start.1 as i32, From::Above));

    while let Some((y, x, from)) = next_pos.pop_front() {
        let directions = match map[y as usize][x as usize] {
            '>' => {
                if from != From::Right {
                    vec![((0, 1), From::Left)]
                } else {
                    continue;
                }
            }
            '<' => {
                if from != From::Left {
                    vec![((0, -1), From::Right)]
                } else {
                    continue;
                }
            }
            '^' => {
                if from != From::Above {
                    vec![((-1, 0), From::Below)]
                } else {
                    continue;
                }
            }
            'v' => {
                if from != From::Below {
                    vec![((1, 0), From::Above)]
                } else {
                    continue;
                }
            }
            _ => match from {
                From::Below => vec![
                    ((0, 1), From::Left),
                    ((0, -1), From::Right),
                    ((-1, 0), From::Below),
                ],
                From::Above => vec![
                    ((0, 1), From::Left),
                    ((0, -1), From::Right),
                    ((1, 0), From::Above),
                ],
                From::Right => vec![
                    ((0, -1), From::Right),
                    ((1, 0), From::Above),
                    ((-1, 0), From::Below),
                ],
                From::Left => vec![
                    ((0, 1), From::Left),
                    ((1, 0), From::Above),
                    ((-1, 0), From::Below),
                ],
            },
        };
        for (dir, to) in directions {
            let new_y = y + dir.0;
            let new_x = x + dir.1;
            if new_y < 0
                || new_x < 0
                || new_y >= height as i32
                || new_x >= width as i32
                || map[new_y as usize][new_x as usize] == '#'
                || weights[new_y as usize][new_x as usize] > weights[y as usize][x as usize] + 1
            {
                continue;
            }
            weights[new_y as usize][new_x as usize] = weights[y as usize][x as usize] + 1;
            next_pos.push_back((new_y, new_x, to));
        }
    }
    for w in &weights {
        println!("{:?}", w);
    }

    for w in weights.last().unwrap().iter() {
        if w != &0 {
            return *w;
        }
    }
    0
}

fn part2(reader: BufReader<File>) -> i32 {
    let map: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| l.chars().collect())
        .collect_vec();

    let mut start: (i32, i32) = (0, 0);
    for (i, c) in map.first().unwrap().iter().enumerate() {
        if c == &'.' {
            start.1 = i as i32;
            break;
        }
    }
    let mut end: (i32, i32) = ((map.len() - 1) as i32, 0);
    for (i, c) in map[map.len() - 1].iter().enumerate() {
        if c == &'.' {
            end.1 = i as i32;
            break;
        }
    }
    println!("start {:?} end {:?}", start, end);
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut pruned: HashMap<(i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
    let mut next_path: VecDeque<(i32, i32, i32, i32, From)> = VecDeque::new();
    next_path.push_back((start.0, start.1, start.0, start.1, From::Above));
    let mut weights: Vec<Vec<i32>> = vec![vec![-1; width as usize]; height as usize];
    weights[start.0 as usize][start.1 as usize] = 0;
    while let Some((y, x, start_y, start_x, from)) = next_path.pop_front() {
        let mut y = y;
        let mut x = x;
        let mut from = from;
        let curr_start_point = (start_y, start_x);
        loop {
            if weights[y as usize][x as usize] == -1 {
                weights[y as usize][x as usize] = 1;
            }
            let directions = match from {
                From::Below => vec![
                    ((0, 1), From::Left),
                    ((0, -1), From::Right),
                    ((-1, 0), From::Below),
                ],
                From::Above => vec![
                    ((0, 1), From::Left),
                    ((0, -1), From::Right),
                    ((1, 0), From::Above),
                ],
                From::Right => vec![
                    ((0, -1), From::Right),
                    ((1, 0), From::Above),
                    ((-1, 0), From::Below),
                ],
                From::Left => vec![
                    ((0, 1), From::Left),
                    ((1, 0), From::Above),
                    ((-1, 0), From::Below),
                ],
            };
            let mut next_positions = Vec::new();
            for (dir, from) in directions {
                let new_y = y + dir.0;
                let new_x = x + dir.1;
                if new_y < -1
                    || new_x < 0
                    || new_x >= width
                    || new_y >= height
                    || map[new_y as usize][new_x as usize] == '#'
                    || weights[new_y as usize][new_x as usize] > -1
                {
                    if new_y == 7 && new_x == 7 {
                        println!("FOUDN END");
                    }
                    if pruned.contains_key(&(new_y, new_x)) {
                        let len = weights[y as usize][x as usize] + 1;
                        pruned
                            .entry(curr_start_point)
                            .and_modify(|v| v.push((new_y, new_x, len)))
                            .or_insert(vec![(new_y, new_x, len); 1]);
                        pruned
                            .entry((new_y, new_x))
                            .and_modify(|v| v.push((curr_start_point.0, curr_start_point.1, len)))
                            .or_insert(vec![(curr_start_point.0, curr_start_point.1, len); 1]);
                    }
                    continue;
                }
                next_positions.push((new_y, new_x, from));
            }
            if next_positions.len() == 1 {
                let (new_y, new_x, new_from) = next_positions[0];
                weights[new_y as usize][new_x as usize] = weights[y as usize][x as usize] + 1;
                if new_y == end.0 && new_x == end.1 {
                    let len = weights[y as usize][x as usize] + 1;
                    pruned
                        .entry(curr_start_point)
                        .and_modify(|v| v.push((new_y, new_x, len)))
                        .or_insert(vec![(new_y, new_x, len); 1]);
                    pruned
                        .entry((new_y, new_x))
                        .and_modify(|v| v.push((curr_start_point.0, curr_start_point.1, len)))
                        .or_insert(vec![(curr_start_point.0, curr_start_point.1, len); 1]);
                }
                y = new_y;
                x = new_x;
                from = new_from;
            } else if next_positions.len() > 1 {
                println!("found multiple y{} x{}", y, x);
                let len = weights[y as usize][x as usize];
                pruned
                    .entry(curr_start_point)
                    .and_modify(|v| v.push((y, x, len)))
                    .or_insert(vec![(y, x, len); 1]);
                pruned
                    .entry((y, x))
                    .and_modify(|v| v.push((curr_start_point.0, curr_start_point.1, len)))
                    .or_insert(vec![(curr_start_point.0, curr_start_point.1, len); 1]);
                for (new_y, new_x, from) in next_positions {
                    next_path.push_back((new_y, new_x, y, x, from));
                }
                break;
            } else if true {
                break;
            }
        }
    }
    for w in &weights {
        for n in w {
            if n == &-1 {
                print!("# ");
            } else {
                print!("{} ", n);
            }
        }
        println!();
    }
   
    
    fn new_path(
        pos: (i32, i32),
        pruned: &HashMap<(i32, i32), Vec<(i32, i32, i32)>>,
        mut visited: Vec<(i32, i32)>,
        path_len: i32,
        mut longest_path: i32,
        end: &(i32, i32),
    ) -> i32 {
        if &pos == end {
            return path_len;
        }
        visited.push(pos);
        for (y, x, len) in pruned.get(&pos).unwrap() {
            if visited.contains(&(*y, *x)) {
                continue;
            }
            longest_path = max(
                longest_path,
                new_path(
                    (*y, *x),
                    pruned,
                    visited.clone(),
                    path_len + len,
                    longest_path,
                    end,
                ),
            );
        }
        longest_path
    }

    new_path(start, &pruned, Vec::new(), 0, 0, &end)
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day23.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
