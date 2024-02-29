use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let mut day = 17;
    let mut run_part_2 = false;
    let args: Vec<String> = env::args().collect();

    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "day" => {
                day = it.next().unwrap().parse().unwrap();
            }
            "part" => {
                if it.next().unwrap().parse::<u32>().unwrap() == 2 {
                    run_part_2 = true;
                }
            }
            _ => {}
        }
    }

    match day {
        1 => day1::run(run_part_2),
        2 => day2::run(run_part_2),
        3 => day3::run(run_part_2),
        4 => day4::run(run_part_2),
        5 => day5::run(run_part_2),
        6 => day6::run(run_part_2),
        7 => day7::run(run_part_2),
        8 => day8::run(run_part_2),
        9 => day9::run(run_part_2),
        10 => day10::run(run_part_2),
        11 => day11::run(run_part_2),
        12 => day12::run(run_part_2),
        13 => day13::run(run_part_2),
        14 => day14::run(run_part_2),
        15 => day15::run(run_part_2),
        16 => day16::run(run_part_2),
        17 => day17::run(run_part_2),
        18 => day18::run(run_part_2),
        19 => day19::run(run_part_2),
        20 => day20::run(run_part_2),
        21 => day21::run(run_part_2),
        22 => day22::run(run_part_2),
        23 => day23::run(run_part_2),
        24 => day24::run(run_part_2),
        25 => day25::run(run_part_2),
        _ => {}
    }
}
