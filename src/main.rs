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

fn main() {
    let mut day = 10;
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
        _ => {}
    }
}
