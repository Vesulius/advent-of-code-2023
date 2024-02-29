use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq)]
enum Diff {
    Lesser,
    Greater,
}

#[derive(Clone, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

fn check_diff(d: &Diff, inst_value: &u64, part_value: &u64) -> bool {
    if d == &Diff::Greater {
        return inst_value < part_value;
    } else {
        return inst_value > part_value;
    }
}

fn part1(reader: BufReader<File>) -> u64 {
    let mut it = reader.lines();
    let mut map: HashMap<String, (Vec<(Letter, u64, Diff, String)>, String)> = HashMap::new();
    loop {
        let line = it.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let workflow_ind = line.find('{').unwrap();
        let workflow: String = line[..workflow_ind].to_string();
        let instructions: Vec<&str> = line[workflow_ind + 1..line.len() - 1].split(',').collect();


        let mut mapped_instructions: Vec<(Letter, u64, Diff, String)> = Vec::new();
        for i in instructions.iter().take(instructions.len() - 1) {
            let mut it = i.chars();
            let letter;
            match it.next().unwrap() {
                'a' => letter = Letter::A,
                'x' => letter = Letter::X,
                'm' => letter = Letter::M,
                's' => letter = Letter::S,
                _ => panic!("Bad letter found"),
            }

            let diff;
            match it.next().unwrap() {
                '>' => diff = Diff::Greater,
                '<' => diff = Diff::Lesser,
                _ => panic!("Bad diff found"),
            }

            let mut num: u64 = 0;
            while let Some(n) = it.next().unwrap().to_digit(10) {
                num *= 10;
                num += n as u64;
            }
            let wf: String = it.collect();
            mapped_instructions.push((letter, num, diff, wf));
        }
        map.insert(
            workflow,
            (
                mapped_instructions,
                instructions.last().unwrap().to_string(),
            ),
        );
    }

    let mut parts: Vec<Part> = Vec::new();
    it.for_each(|l| {
        let line = l.unwrap();
        let part_values: Vec<&str> = line[1..line.len() - 1].split(',').collect();

        parts.push(Part {
            x: part_values[0][2..].parse::<u64>().unwrap(),
            m: part_values[1][2..].parse::<u64>().unwrap(),
            a: part_values[2][2..].parse::<u64>().unwrap(),
            s: part_values[3][2..].parse::<u64>().unwrap(),
        });
    });


    let mut sum = 0;
    let mut next_inst;
    for p in parts {
        next_inst = "in";
        loop {
            if next_inst == "A" {
                sum += p.x + p.m + p.a + p.s;
                break;
            } else if next_inst == "R" {
                break;
            }
            let inst: &(Vec<(Letter, u64, Diff, String)>, String) = map.get(next_inst).unwrap();
            next_inst = &inst.1;
            for (letter, inst_value, diff, this_inst) in &inst.0 {
                let this_part_value = match letter {
                    Letter::X => p.x,
                    Letter::M => p.m,
                    Letter::A => p.a,
                    Letter::S => p.s,
                };

                if check_diff(diff, inst_value, &this_part_value) {
                    next_inst = this_inst;
                    break;
                }
            }
        }
    }
    sum
}

fn part2(reader: BufReader<File>) -> u64 {
    let mut it = reader.lines();
    let mut map: HashMap<String, (Vec<(Letter, u64, Diff, String)>, String)> = HashMap::new();
    loop {
        let line = it.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let workflow_ind = line.find('{').unwrap();
        let workflow: String = line[..workflow_ind].to_string();
        let instructions: Vec<&str> = line[workflow_ind + 1..line.len() - 1].split(',').collect();


        let mut mapped_instructions: Vec<(Letter, u64, Diff, String)> = Vec::new();
        for i in instructions.iter().take(instructions.len() - 1) {
            let mut it = i.chars();
            let letter;
            match it.next().unwrap() {
                'a' => letter = Letter::A,
                'x' => letter = Letter::X,
                'm' => letter = Letter::M,
                's' => letter = Letter::S,
                _ => panic!("Bad letter found"),
            }

            let diff;
            match it.next().unwrap() {
                '>' => diff = Diff::Greater,
                '<' => diff = Diff::Lesser,
                _ => panic!("Bad diff found"),
            }

            let mut num = 0;
            while let Some(n) = it.next().unwrap().to_digit(10) {
                num *= 10;
                num += n as u64;
            }
            let wf: String = it.collect();
            mapped_instructions.push((letter, num, diff, wf));
        }
        map.insert(
            workflow,
            (
                mapped_instructions,
                instructions.last().unwrap().to_string(),
            ),
        );
    }

    fn recc(
        next_inst: &str,
        mut low: Part,
        mut high: Part,
        map: &HashMap<String, (Vec<(Letter, u64, Diff, String)>, String)>,
        accepted_parts: &mut Vec<(Part, Part)>,
    ) {
        if next_inst == "R" {
            return;
        } else if next_inst == "A" {
            accepted_parts.push((low.clone(), high.clone()));
            return;
        }
        let inst: &(Vec<(Letter, u64, Diff, String)>, String) = map.get(next_inst).unwrap();
        for (letter, inst_value, diff, this_inst) in &inst.0 {
            if diff == &Diff::Greater {
                match letter {
                    Letter::X => {
                        if high.x > *inst_value {
                            let mut low_copy = low.clone();
                            low_copy.x = *inst_value + 1;
                            recc(
                                this_inst,
                                low_copy,
                                high.clone(),
                                map,
                                accepted_parts,
                            );
                            high.x = *inst_value;
                        }
                    }
                    Letter::M => {
                        if high.m > *inst_value {
                            let mut low_copy = low.clone();
                            low_copy.m = *inst_value + 1;
                            recc(
                                this_inst,
                                low_copy,
                                high.clone(),
                                map,
                                accepted_parts,
                            );
                            high.m = *inst_value;
                        }
                    }
                    Letter::A => {
                        if high.a > *inst_value {
                            let mut low_copy = low.clone();
                            low_copy.a = *inst_value + 1;
                            recc(
                                this_inst,
                                low_copy,
                                high.clone(),
                                map,
                                accepted_parts,
                            );
                            high.a = *inst_value;
                        }
                    }
                    Letter::S => {
                        if high.s > *inst_value {
                            let mut low_copy = low.clone();
                            low_copy.s = *inst_value + 1;
                            recc(
                                this_inst,
                                low_copy,
                                high.clone(),
                                map,
                                accepted_parts,
                            );
                            high.s = *inst_value;
                        }
                    }
                };
            } else {
                match letter {
                    Letter::X => {
                        if low.x < *inst_value {
                            let mut high_copy = high.clone();
                            high_copy.x = *inst_value - 1;
                            recc(
                                this_inst,
                                low.clone(),
                                high_copy,
                                map,
                                accepted_parts,
                            );
                            low.x = *inst_value;
                        }
                    }
                    Letter::M => {
                        if low.m < *inst_value {
                            let mut high_copy = high.clone();
                            high_copy.m = *inst_value - 1;
                            recc(
                                this_inst,
                                low.clone(),
                                high_copy,
                                map,
                                accepted_parts,
                            );
                            low.m = *inst_value;
                        }
                    }
                    Letter::A => {
                        if low.a < *inst_value {
                            let mut high_copy = high.clone();
                            high_copy.a = *inst_value - 1;
                            recc(
                                this_inst,
                                low.clone(),
                                high_copy,
                                map,
                                accepted_parts,
                            );
                            low.a = *inst_value;
                        }
                    }
                    Letter::S => {
                        if low.s < *inst_value {
                            let mut high_copy = high.clone();
                            high_copy.s = *inst_value - 1;
                            recc(
                                this_inst,
                                low.clone(),
                                high_copy,
                                map,
                                accepted_parts,
                            );
                            low.s = *inst_value;
                        }
                    }
                };
            }
        }
        recc(&inst.1, low, high, map, accepted_parts);
    }

    let low = Part {
        x: 1,
        m: 1,
        a: 1,
        s: 1,
    };
    let high = Part {
        x: 4000,
        m: 4000,
        a: 4000,
        s: 4000,
    };

    let next_inst = "in";
    let mut accepted_parts: Vec<(Part, Part)> = Vec::new();
    recc(
        next_inst,
        low.clone(),
        high.clone(),
        &map,
        &mut accepted_parts,
    );
    let mut sum: u64 = 0;
    for (low, high) in accepted_parts {
        let total_letters: u64 =
            (high.x + 1 - low.x) * (high.a + 1 - low.a) * (high.m + 1 - low.m) * (high.s + 1 - low.s);
        sum += total_letters;
    }
    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day19.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
