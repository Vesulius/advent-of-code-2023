use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1(reader: BufReader<File>) -> i32 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    reader.lines().map_while(Result::ok).for_each(|line| {
        let mut nodes: Vec<&str> = line.split_whitespace().collect();
        nodes[0] = &nodes[0][..3];
        for i in 1..nodes.len() {
            if graph.contains_key(nodes[0]) {
                let neighs = graph.get_mut(nodes[0]).unwrap();
                (*neighs).push(nodes[i].to_string());
            } else {
                graph.insert(nodes[0].to_string(), vec![nodes[i].to_string()]);
            }
            if graph.contains_key(nodes[i]) {
                let neighs = graph.get_mut(nodes[i]).unwrap();
                (*neighs).push(nodes[0].to_string());
            } else {
                graph.insert(nodes[i].to_string(), vec![nodes[0].to_string()]);
            }
        }
    });

    let keys: Vec<_> = graph.keys().cloned().collect();
    let mut counts: HashMap<(&String, &String), i32> = HashMap::new();

    for _ in 0..1000 {
        let mut visited: HashMap<&String, &String> = HashMap::new();
        let mut next_node: VecDeque<&String> = VecDeque::new();

        let start_ind = rand::thread_rng().gen_range(0..keys.len()) as usize;
        let goal_ind = rand::thread_rng().gen_range(0..keys.len()) as usize;

        let start = &keys[start_ind];
        let goal = &keys[goal_ind];

        next_node.push_back(start);

        // bfs
        'outer: while let Some(node) = next_node.pop_front() {
            for n in graph.get(node).unwrap() {
                if !visited.contains_key(n) {
                    visited.insert(n, node);
                    if n == goal {
                        break 'outer;
                    }
                    next_node.push_back(n);
                }
            }
        }

        let mut loopback = goal;
        // loop path back
        while let Some(node) = visited.get(loopback) {
            let key = if node > &loopback {
                (*node, loopback)
            } else {
                (loopback, *node)
            };
            *counts.entry(key).or_insert(0) += 1;
            if node == &start {
                break;
            }
            loopback = node;
        }
    }

    let mut sorted_counts: Vec<_> = counts.iter().collect();
    sorted_counts.sort_by_key(|&(_, &v)| std::cmp::Reverse(v));

    let top_three: Vec<_> = sorted_counts.iter().take(3).map(|v| v.0).collect();
    dbg!(&top_three);

    let start_ind = rand::thread_rng().gen_range(0..keys.len()) as usize;
    let start = &keys[start_ind];

    let mut visited: HashSet<&String> = HashSet::new();
    let mut next_node: VecDeque<&String> = VecDeque::new();

    next_node.push_back(start);

    while let Some(node) = next_node.pop_front() {
        for n in graph.get(node).unwrap() {
            if top_three.contains(&&(node, n))
                || top_three.contains(&&(n, node))
                || visited.contains(n)
            {
                continue;
            }
            visited.insert(n);
            next_node.push_back(n);
        }
    }
    visited.len() as i32 * (keys.len() as i32 - visited.len() as i32)
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day25.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("NO PUZZLE");
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
