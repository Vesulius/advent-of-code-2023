use itertools::Itertools;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1(reader: BufReader<File>) -> u64 {
    let start_area: f64 = 200000000000000.0;
    let end_area: f64 = 400000000000000.0;
    // let start_area: f64 = 7.0;
    // let end_area: f64 = 27.0;

    #[derive(PartialEq, Debug)]
    struct Hail {
        coef: f64,
        offset: f64,
        speed: f64,
        pos: (f64, f64),
    }

    let hails: Vec<Hail> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let split: Vec<f64> = line
                .split(|s| s == ',' || s == '@')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect_vec();
            let x_comp = split[3];
            let y_comp = split[4];
            let x_point = split[0];
            let y_point = split[1];
            let coef = y_comp / x_comp;
            let offset = (coef * (-x_point)) + y_point;
            // speed is relative to x axis, if hail is going "left" speed is negative.
            let speed = (x_comp / x_comp.abs()) * (x_comp.powi(2) + y_comp.powi(2)).sqrt();
            if speed == 0.0 {
                panic!("Need to check this edgecase");
            }
            Hail {
                coef,
                offset,
                speed,
                pos: (x_point, y_point),
            }
        })
        .collect_vec();

    let mut sum = 0;
    for i in 0..hails.len() - 1 {
        for j in (i + 1)..hails.len() {
            let h1 = &hails[i];
            let h2 = &hails[j];
            if h1.coef == h2.coef {
                if h1.offset == h2.offset {
                    panic!("fix if this actually ever happens");
                }
            } else {
                let intersec_x = (h2.offset - h1.offset) / (h1.coef - h2.coef);
                let intersec_y = h1.coef * intersec_x + h1.offset;
                if intersec_x >= start_area
                    && intersec_y >= start_area
                    && intersec_x <= end_area
                    && intersec_y <= end_area
                    && ((h1.speed > 0.0 && intersec_x > h1.pos.0)
                        || (h1.speed < 0.0 && intersec_x < h1.pos.0))
                    && ((h2.speed > 0.0 && intersec_x > h2.pos.0)
                        || (h2.speed < 0.0 && intersec_x < h2.pos.0))
                {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn part2(reader: BufReader<File>) -> f64 {
    #[derive(PartialEq, Debug)]
    struct Hail {
        pos: (f64, f64, f64),
        dir: (f64, f64, f64),
        speed: f64,
    }

    let hails: Vec<Hail> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let split: Vec<f64> = line
                .split(|s| s == ',' || s == '@')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect_vec();
            let speed = (split[3].powi(2) + split[4].powi(2) + split[5].powi(2)).sqrt();
            Hail {
                pos: (split[0], split[1], split[2]),
                dir: (split[3], split[4], split[5]),
                speed,
            }
        })
        .collect_vec();

    let mut x_velos: Vec<f64> = Vec::new();
    let mut y_velos: Vec<f64> = Vec::new();
    let mut z_velos: Vec<f64> = Vec::new();
    for i in 0..hails.len() - 1 {
        for j in (i + 1)..hails.len() {
            let h1 = &hails[i];
            let h2 = &hails[j];

            if h1.dir.0 == h2.dir.0 {
                let diff = h1.pos.0 - h2.pos.0;
                if x_velos.is_empty() {
                    for v in -10000..10000 {
                        if diff % (v as f64 - h1.dir.0) == 0.0 {
                            x_velos.push(v as f64);
                        }
                    }
                } else {
                    let mut new_x_velos: Vec<f64> = Vec::new();
                    for v in &x_velos {
                        if diff % (v - h1.dir.0) == 0.0 {
                            new_x_velos.push(*v)
                        }
                    }
                    if !new_x_velos.is_empty() {
                        x_velos = new_x_velos;
                    }
                }
            }
            if h1.dir.1 == h2.dir.1 {
                let diff = h1.pos.1 - h2.pos.1;
                if y_velos.is_empty() {
                    for v in -10000..10000 {
                        if diff % (v as f64 - h1.dir.1) == 0.0 {
                            y_velos.push(v as f64);
                        }
                    }
                } else {
                    let mut new_y_velos: Vec<f64> = Vec::new();
                    for v in &y_velos {
                        if diff % (v - h1.dir.1) == 0.0 {
                            new_y_velos.push(*v)
                        }
                    }
                    if !new_y_velos.is_empty() {
                        y_velos = new_y_velos;
                    }
                }
            }
            if h1.dir.2 == h2.dir.2 {
                let diff = h1.pos.2 - h2.pos.2;
                if z_velos.is_empty() {
                    for v in -10000..10000 {
                        if diff % (v as f64 - h1.dir.2) == 0.0 {
                            z_velos.push(v as f64);
                        }
                    }
                } else {
                    let mut new_z_velos: Vec<f64> = Vec::new();
                    for v in &z_velos {
                        if diff % (v - h1.dir.2) == 0.0 {
                            new_z_velos.push(*v)
                        }
                    }
                    if !new_z_velos.is_empty() {
                        z_velos = new_z_velos;
                    }
                }
            }
        }
    }
    println!("common x_velos {:?}", x_velos);
    println!("common y_velos {:?}", y_velos);
    println!("common z_velos {:?}", z_velos);

    let rock_vx = x_velos.pop().unwrap();
    let rock_vy = y_velos.pop().unwrap();
    let rock_vz = z_velos.pop().unwrap();
    let h1 = &hails[0];
    let (vx, vy, _vz) = (h1.dir.0 - rock_vx, h1.dir.1 - rock_vy, h1.dir.2 - rock_vz);
    let coef = vy / vx;
    let offset = (coef * (-h1.pos.0)) + h1.pos.1;

    let h2 = &hails[1];
    let (v2x, v2y, _v2z) = (h2.dir.0 - rock_vx, h2.dir.1 - rock_vy, h2.dir.2 - rock_vz);
    let coef2 = v2y / v2x;
    let offset2 = (coef2 * (-h2.pos.0)) + h2.pos.1;
    let x = (offset2 - offset) / (coef - coef2);
    let y = coef * x + offset;
    let time_to_impact = (x - h1.pos.0) / (h1.dir.0 - rock_vx);
    println!("x {}", x);
    println!("y {}", y);
    println!("time {}", time_to_impact);
    let z = h1.pos.2 + (h1.dir.2 - rock_vz) * time_to_impact;
    println!("z {}", z);

    x + y + z
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/test.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
