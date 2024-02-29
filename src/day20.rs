use num::integer::lcm;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Type {
    Conj,
    FlipFlop,
    Broad,
}

fn part2(reader: BufReader<File>) -> u64 {
    let mut modules: HashMap<String, (Type, Vec<String>)> = HashMap::new();
    let mut inv_modules: HashMap<String, (Vec<String>, Vec<bool>)> = HashMap::new();

    // bool = isOn
    let mut flipflops: HashMap<String, bool> = HashMap::new();

    reader.lines().map_while(Result::ok).for_each(|line| {
        let split: Vec<&str> = line.split(" -> ").collect();
        let mut it = split[0].chars();
        let typ = match it.next().unwrap() {
            '&' => Type::Conj,
            '%' => Type::FlipFlop,
            _ => Type::Broad,
        };

        let dest: Vec<String> = split[1].split(", ").map(|s| s.to_string()).collect();
        let module: String = it.collect();

        for d in &dest {
            inv_modules
                .entry(d.clone())
                .and_modify(|inv_module| {
                    inv_module.0.push(module.clone());
                    inv_module.1.push(false)
                })
                .or_insert((vec![module.clone()], vec![false]));
        }

        if typ == Type::FlipFlop {
            flipflops.insert(module.clone(), false);
        }

        modules.insert(module, (typ, dest));
    });

    // Solving this requires additional information about the graph

    // The graph has four structures that work as binary timers. When
    // a timer is done it will send a low impulse to lx module. If all structures
    // send the low impulse at the same time, a new low impuse is sent at rx finishing
    // the circuit. However all these clocks have different times. We need
    // therefore to find the smallest common multiple of all structure timers values.

    // The four structures have the following entry- and exit points:
    let structure_points = [("sx", "cl"), ("gs", "nj"), ("fc", "rp"), ("hd", "lb")];
    let mut timers = [0; 4];
    for (i, (entry, exit)) in structure_points.iter().enumerate() {
        let mut runs = 0;
        let mut got_out = false;
        while !got_out {
            // false = low pulse, true = high pulse
            let mut next_modules: VecDeque<(&str, &str, bool)> = VecDeque::new();
            next_modules.push_back((entry, "broadcaster", false));
                runs += 1;

            while !next_modules.is_empty() {
                let (current_module, prev_module, is_high_pulse) =
                    next_modules.pop_front().unwrap();
                if &current_module == exit && !is_high_pulse {
                    timers[i] = runs;
                    got_out = true;
                }
                if let Some((typ, dests)) = modules.get(current_module) {
                    match typ {
                        Type::Broad => {
                            for d in dests {
                                next_modules.push_back((d, current_module, false));
                            }
                        }
                        Type::Conj => {
                            let (inputs, pulses) = inv_modules.get_mut(current_module).unwrap();
                            for (i, input) in inputs.iter().enumerate() {
                                if input == prev_module {
                                    pulses[i] = is_high_pulse;
                                    break;
                                }
                            }
                            let mut all_high = true;
                            for p in pulses {
                                all_high &= *p;
                            }
                            for d in dests {
                                next_modules.push_back((d, current_module, !all_high));
                            }
                        }
                        Type::FlipFlop => {
                            if !is_high_pulse {
                                if let Some(is_on) = flipflops.get_mut(current_module) {
                                    for d in dests {
                                        next_modules.push_back((d, current_module, !*is_on));
                                    }
                                    *is_on = !*is_on;
                                }
                            }
                        }
                    }
                }
            }
        }
        
    }
    timers.iter().fold(1, |acc, &x| lcm(acc, x))
}

fn part1(reader: BufReader<File>) -> u64 {
    let mut modules: HashMap<String, (Type, Vec<String>)> = HashMap::new();
    let mut inv_modules: HashMap<String, (Vec<String>, Vec<bool>)> = HashMap::new();

    // bool = isOn
    let mut flipflops: HashMap<String, bool> = HashMap::new();

    reader.lines().map_while(Result::ok).for_each(|line| {
        let split: Vec<&str> = line.split(" -> ").collect();
        let mut it = split[0].chars();
        let typ = match it.next().unwrap() {
            '&' => Type::Conj,
            '%' => Type::FlipFlop,
            _ => Type::Broad,
        };

        let dest: Vec<String> = split[1].split(", ").map(|s| s.to_string()).collect();
        let module: String = it.collect();

        for d in &dest {
            inv_modules
                .entry(d.clone())
                .and_modify(|inv_module| {
                    inv_module.0.push(module.clone());
                    inv_module.1.push(false)
                })
                .or_insert((vec![module.clone()], vec![false]));
        }

        if typ == Type::FlipFlop {
            flipflops.insert(module.clone(), false);
        }

        modules.insert(module, (typ, dest));
    });

    let mut low_pulses;
    let mut high_pulses;

    low_pulses = 0;
    high_pulses = 0;
    for _ in 0..1000 {
        // false = low pulse, true = high pulse
        let mut next_modules: VecDeque<(&str, &str, bool)> = VecDeque::new();
        next_modules.push_back(("roadcaster", "start", false));
        while !next_modules.is_empty() {
            let (current_module, prev_module, is_high_pulse) = next_modules.pop_front().unwrap();
            if is_high_pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            if let Some((typ, dests)) = modules.get(current_module) {
                match typ {
                    Type::Broad => {
                        for d in dests {
                            next_modules.push_back((d, current_module, false));
                        }
                    }
                    Type::Conj => {
                        let (inputs, pulses) = inv_modules.get_mut(current_module).unwrap();
                        for (i, input) in inputs.iter().enumerate() {
                            if input == prev_module {
                                pulses[i] = is_high_pulse;
                                break;
                            }
                        }
                        let mut all_high = true;
                        for p in pulses {
                            all_high &= *p;
                        }
                        for d in dests {
                            next_modules.push_back((d, current_module, !all_high));
                        }
                    }
                    Type::FlipFlop => {
                        if !is_high_pulse {
                            if let Some(is_on) = flipflops.get_mut(current_module) {
                                for d in dests {
                                    next_modules.push_back((d, current_module, !*is_on));
                                }
                                *is_on = !*is_on;
                            }
                        }
                    }
                }
            } else {
                // panic!("Found dead end");
            }
        }
        if flipflops.values().all(|ff| !ff) {
            break;
        }
    }
    low_pulses * high_pulses
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day20.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
