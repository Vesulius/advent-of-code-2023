use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use itertools::Itertools;

fn rec(
    mut n: (i32, i32),
    mut d: (i8, i8),
    set: &mut HashSet<((i32, i32), (i8, i8))>,
    height: usize,
    width: usize,
    paths: &Vec<Vec<char>>,
) {
    loop {
        if n.0 < 0 || n.1 < 0 || n.0 >= height as i32 || n.1 >= width as i32 {
            return;
        }
        if set.contains(&(n, d)) {
            return;
        }

        set.insert((n, d));

        match paths[n.0 as usize][n.1 as usize] {
            '|' => {
                if d.0 == 0 {
                    rec((n.0 + 1, n.1), (1, 0), set, height, width, paths);
                    rec((n.0 - 1, n.1), (-1, 0), set, height, width, paths);
                    return;
                }
            }
            '-' => {
                if d.1 == 0 {
                    rec((n.0, n.1 + 1), (0, 1), set, height, width, paths);
                    rec((n.0, n.1 - 1), (0, -1), set, height, width, paths);
                    return;
                }
            }
            '/' => {
                d = (-d.1, -d.0);
            }
            '\\' => {
                d = (d.1, d.0);
            }
            _ => {}
        }
        n.0 += d.0 as i32;
        n.1 += d.1 as i32;
    }
}

fn part1(reader: BufReader<File>) -> u32 {
    let paths: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| l.chars().collect())
        .collect_vec();

    let height = paths.len();
    let width = paths[0].len();
    let mut set: HashSet<((i32, i32), (i8, i8))> = HashSet::new();

    rec((0, 0), (0, 1), &mut set, height, width, &paths);

    let mut visited = vec![vec!['.'; width]; height];
    for (n, _) in set.iter() {
        visited[n.0 as usize][n.1 as usize] = 'y';
    }
    let mut counter = 0;
    for y in 0..height {
        for x in 0..width {
            if visited[y][x] == 'y' {
                counter += 1;
            }
        }
    }

    counter
}

fn part2(reader: BufReader<File>) -> usize {
    let paths: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| l.chars().collect())
        .collect_vec();

    let height = paths.len();
    let width = paths[0].len();

    let mut max = 0;

    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut dir_ind: i32 = -1;
    for (i, (yrang, xrang)) in [
        ((0..1), (0..width)),
        ((height - 1..height), (0..width)),
        ((0..height), (0..1)),
        ((0..height), (width - 1..width)),
    ]
    .iter()
    .enumerate()
    {
        for y in yrang.clone() {
            if y == 0 || y == yrang.clone().last().unwrap() {
                dir_ind += 1;
            }
            for x in xrang.clone() {
                let mut set: HashSet<((i32, i32), (i8, i8))> = HashSet::new();
                rec(
                    (y as i32, x as i32),
                    dirs[i],
                    &mut set,
                    height,
                    width,
                    &paths,
                );

                let mut visited = vec![vec!['.'; width]; height];
                for (n, _) in set.iter() {
                    visited[n.0 as usize][n.1 as usize] = 'y';
                }
                let mut counter = 0;
                for y in 0..height {
                    for x in 0..width {
                        if visited[y][x] == 'y' {
                            counter += 1;
                        }
                    }
                }
                if counter > max {
                    max = counter;
                }
            }
        }
    }
    max
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day16.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
