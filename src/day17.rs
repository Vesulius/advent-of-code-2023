use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
// fn part1(reader: BufReader<File>) -> u64 {
//     let mut paths: Vec<Vec<u32>> = reader
//         .lines()
//         .map_while(Result::ok)
//         .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
//         .collect_vec();

//     let height = paths.len();
//     let width = paths[0].len();

//     let weights: Vec<Vec<u32>> = paths.clone();
//     let mut dirs: Vec<Vec<i32>> = vec![vec![0; width]; height];

//     let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
//     visited[height - 1][width - 1] = true;

//     for y in (0..height).rev() {
//         for x in (0..width).rev() {
//             if !visited[y][x] {
//                 continue;
//             }
//             if x > 0
//                 && (paths[y][x - 1] == weights[y][x - 1]
//                     || paths[y][x] + weights[y][x - 1] < paths[y][x - 1])
//                 && dirs[y][x] > -2
//             {
//                 visited[y][x - 1] = true;
//                 dirs[y][x - 1] = dirs[y][x] - 1;
//                 paths[y][x - 1] = paths[y][x] + weights[y][x - 1];
//             }
//             if y > 0
//                 && (paths[y - 1][x] == weights[y - 1][x]
//                     || paths[y][x] + weights[y - 1][x] < paths[y - 1][x])
//                 && dirs[y][x] < 2
//             {
//                 visited[y - 1][x] = true;
//                 dirs[y - 1][x] = dirs[y][x] + 1;
//                 paths[y - 1][x] = paths[y][x] + weights[y - 1][x];
//             }
//         }
//     }
//     println!("PATHS:");
//     for p in paths {
//         println!("{:?}", p);
//     }
//     println!("DIRS:");
//     for d in dirs {
//         println!("{:?}", d);
//     }
//     println!("VIS:");
//     for v in visited {
//         println!("{:?}", v.iter().map(|v| if *v { 'y' } else { 'n' }).collect::<String>);
//     }
//     0
// }

// fn part1(reader: BufReader<File>) -> u32 {
//     let weights: Vec<Vec<u32>> = reader
//         .lines()
//         .map_while(Result::ok)
//         .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
//         .collect();

//     let height = weights.len();
//     let width = weights[0].len();

//     let mut paths = vec![vec![999999999; width]; height];
//     paths[0][0] = 0;
//     let mut visited = vec![vec![false; width]; height];

//     let mut queue: PriorityQueue<(usize, usize, (i8, i8)), i32> = PriorityQueue::new();
//     queue.push((0, 0, (0, 0)), 0);

//     while !queue.is_empty() {
//         let ((y, x, d), _) = queue.pop().unwrap();

//         if y + 1 < height
//             && (!visited[y][x] || d.0 < 3)
//             && d.0 >= 0
//             && (paths[y][x] + weights[y + 1][x] <= paths[y + 1][x])
//         {
//             paths[y + 1][x] = paths[y][x] + weights[y + 1][x];
//             queue.push((y + 1, x, (d.0 + 1, 0)), -(paths[y + 1][x] as i32));
//         }

//         if y > 0
//             && (!visited[y][x] || d.0 > -3)
//             && d.0 <= 0
//             && (paths[y][x] + weights[y - 1][x] <= paths[y - 1][x])
//         {
//             paths[y - 1][x] = paths[y][x] + weights[y - 1][x];
//             queue.push((y - 1, x, (d.0 - 1, 0)), -(paths[y - 1][x] as i32));
//         }

//         if x + 1 < width
//             && (!visited[y][x] || d.1 < 3)
//             && d.1 >= 0
//             && (paths[y][x] + weights[y][x + 1] <= paths[y][x + 1])
//         {
//             paths[y][x + 1] = paths[y][x] + weights[y][x + 1];
//             queue.push((y, x + 1, (0, d.1 + 1)), -(paths[y][x + 1] as i32));
//         }

//         if x > 0
//             && (!visited[y][x] || d.1 > -3 )
//             && d.1 <= 0
//             && (paths[y][x] + weights[y][x - 1] <= paths[y][x - 1])
//         {
//             paths[y][x - 1] = paths[y][x] + weights[y][x - 1];
//             queue.push((y, x - 1, (0, d.1 - 1)), -(paths[y][x - 1] as i32));
//         }
//         visited[y][x] = true;
//     }
//     println!("PATHS: ");
//     for p in &paths {
//         println!("{:?}", p);
//     }
//     paths[height - 1][width - 1]
// }

// fn part1(reader: BufReader<File>) -> u32 {
//     let weights: Vec<Vec<u32>> = reader
//         .lines()
//         .map_while(Result::ok)
//         .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
//         .collect();

//     let height = weights.len();
//     let width = weights[0].len();

//     let mut paths = vec![vec![999999999; width]; height];
//     paths[0][0] = 0;
//     let mut visited = vec![vec![false; width]; height];

//     let mut queue: PriorityQueue<(usize, usize, (i8, i8)), i32> = PriorityQueue::new();
//     queue.push((0, 0, (0, 0)), 0);

//     while !queue.is_empty() {
//         let ((y, x, d), _) = queue.pop().unwrap();

//         let mut cum: u32 = paths[y][x];
//         for i in 1..4 {
//             if y + i < height
//                 && (d.0 + i as i8) <= 3
//                 && d.0 <= 0
//                 && (cum + weights[y + i][x] <= paths[y + i][x])
//             {
//                 cum += weights[y + i][x];
//                 paths[y + i][x] = cum;
//                 queue.push((y + i, x, (d.0 + i as i8, 0)), -(paths[y + i][x] as i32));
//             }
//         }
//         let mut cum: u32 = paths[y][x];
//         for i in 1..4 {
//             if y as i32 - i as i32 >= 0
//                 && (d.0 - i as i8) >= -3
//                 && d.0 <= 0
//                 && (cum + weights[y - i][x] <= paths[y - i][x])
//             {
//                 cum += weights[y - i][x];
//                 paths[y - i][x] = cum;
//                 queue.push((y - i, x, (d.0 - i as i8, 0)), -(paths[y - i][x] as i32));
//             }
//         }
//         let mut cum: u32 = paths[y][x];
//         for i in 1..4 {
//             if x + i < width
//                 && (d.1 + i as i8) <= 3
//                 && d.1 >= 0
//                 && (cum + weights[y][x + i] <= paths[y][x + i])
//             {
//                 cum += weights[y][x + i];
//                 paths[y][x + i] = cum;
//                 queue.push((y, x + i, (0, d.1 + i as i8)), -(paths[y][x + i] as i32));
//             }
//         }
//         let mut cum: u32 = paths[y][x];
//         for i in 1..4 {
//             if x as i32 - i as i32 >= 0
//                 && (d.1 - i as i8) >= -3
//                 && d.1 <= 0
//                 && (cum + weights[y][x - i] <= paths[y][x - i])
//             {
//                 cum += weights[y][x - i];
//                 paths[y][x - i] = cum;
//                 queue.push((y, x - i, (0, d.1 - i as i8)), -(paths[y][x - i] as i32));
//             }
//         }
//         visited[y][x] = true;
//     }
//     println!("PATHS: ");
//     for p in &paths {
//         println!("{:?}", p);
//     }
//     paths[height - 1][width - 1]
// }

// #[derive(Clone)]
// enum From {
//     Below(i32),
//     Above(i32),
//     Left(i32),
//     Right(i32),
//     NotVisited,
// }

// impl PartialEq for From {
//     fn eq(&self, other: &Self) -> bool {
//         matches!(
//             (self, other),
//             (From::Below(_), From::Below(_))
//                 | (From::Above(_), From::Above(_))
//                 | (From::Left(_), From::Left(_))
//                 | (From::Right(_), From::Right(_))
//                 | (From::NotVisited, From::NotVisited)
//         )
//     }
// }

// fn part1(reader: BufReader<File>) -> u32 {
//     let heatloss: Vec<Vec<u32>> = reader
//         .lines()
//         .map_while(Result::ok)
//         .map(|l| {
//             l.chars()
//                 .map(|c| c.to_digit(10).unwrap())
//                 .collect::<Vec<u32>>()
//         })
//         .collect();

//     let height = heatloss.len();
//     let width = heatloss[0].len();

//     let mut path_heat = vec![vec![99999999; width]; height];
//     let mut froms = vec![vec![vec![From::NotVisited]; width]; height];
//     path_heat[0][0] = 0;
//     froms[0][0].push(From::Above(0));

//     let mut next_path: VecDeque<(i32, i32, From)> = VecDeque::new();
//     next_path.push_back((0, 0, From::Above(0)));

//     while let Some((y, x, from)) = next_path.pop_front() {
//         println!("y {} x {}", y, x);
//         // dirs are y, x
//         let next_positions = match from {
//             From::Below(steps) => {
//                 vec![
//                     (0, 1, From::Right(1)),
//                     (0, -1, From::Left(1)),
//                     // (1, 0, Dir::Down(0)),
//                     (-1, 0, From::Below(steps + 1)),
//                 ]
//             }
//             From::Above(steps) => {
//                 vec![
//                     (0, 1, From::Right(1)),
//                     (0, -1, From::Left(1)),
//                     (1, 0, From::Above(steps + 1)),
//                     // (-1, 0, Dir::Up(0)),
//                 ]
//             }
//             From::Right(steps) => {
//                 vec![
//                     (0, 1, From::Right(steps + 1)),
//                     // (0, -1, Dir::Left(0)),
//                     (1, 0, From::Above(1)),
//                     (-1, 0, From::Below(1)),
//                 ]
//             }
//             From::Left(steps) => {
//                 vec![
//                     // (0, 1, Dir::Right(0)),
//                     (0, -1, From::Left(steps + 1)),
//                     (1, 0, From::Above(1)),
//                     (-1, 0, From::Below(1)),
//                 ]
//             }
//             _ => panic!("oh no"),
//         };

//         for (pos_y, pos_x, next_from) in next_positions {
//             let new_y = y + pos_y;
//             let new_x = x + pos_x;
//             let dir_val = match next_from {
//                 From::Below(val) => val,
//                 From::Above(val) => val,
//                 From::Left(val) => val,
//                 From::Right(val) => val,
//                 _ => panic!("oh no"),
//             };
//             if new_x < 0
//                 || new_y < 0
//                 || new_x >= width as i32
//                 || new_y >= height as i32
//                 || dir_val > 2
//                 || (path_heat[y as usize][x as usize] + heatloss[new_y as usize][new_x as usize]
//                     > path_heat[new_y as usize][new_x as usize]
//                     && froms[new_y as usize][new_x as usize].contains(&next_from))
//             {
//                 continue;
//             }
//             froms[new_y as usize][new_x as usize].push(next_from.clone());
//             if path_heat[y as usize][x as usize] + heatloss[new_y as usize][new_x as usize]
//                 < path_heat[new_y as usize][new_x as usize]
//             {
//                 path_heat[new_y as usize][new_x as usize] =
//                     path_heat[y as usize][x as usize] + heatloss[new_y as usize][new_x as usize];
//             }
//             next_path.push_back((new_y, new_x, next_from));
//         }
//     }

//     println!("PATHS: ");
//     for p in &path_heat {
//         println!("{:?}", p);
//     }
//     path_heat[height - 1][width - 1] - heatloss[0][0]
// }

// fn part1(reader: BufReader<File>) -> u32 {
//     let heatloss: Vec<Vec<u32>> = reader
//         .lines()
//         .map_while(Result::ok)
//         .map(|l| {
//             l.chars()
//                 .map(|c| c.to_digit(10).unwrap())
//                 .collect::<Vec<u32>>()
//         })
//         .collect();

//     let height = heatloss.len();
//     let width = heatloss[0].len();

//     let mut path_heat = vec![vec![999999999; width]; height];
//     path_heat[0][0] = 0;

//     let mut next_path: VecDeque<(i32, i32, From)> = VecDeque::new();
//     next_path.push_back((0, 0, From::Above(0)));

//     while let Some((y, x, from)) = next_path.pop_front() {
//         // dirs are y, x
//         let next_positions = match from {
//             From::Below(steps) => {
//                 vec![
//                     (0, 1, From::Right(1)),
//                     (0, -1, From::Left(1)),
//                     // (1, 0, Dir::Down(0)),
//                     (-1, 0, From::Below(steps + 1)),
//                 ]
//             }
//             From::Above(steps) => {
//                 vec![
//                     (0, 1, From::Right(1)),
//                     (0, -1, From::Left(1)),
//                     (1, 0, From::Above(steps + 1)),
//                     // (-1, 0, Dir::Up(0)),
//                 ]
//             }
//             From::Right(steps) => {
//                 vec![
//                     (0, 1, From::Right(steps + 1)),
//                     // (0, -1, Dir::Left(0)),
//                     (1, 0, From::Above(1)),
//                     (-1, 0, From::Below(1)),
//                 ]
//             }
//             From::Left(steps) => {
//                 vec![
//                     // (0, 1, Dir::Right(0)),
//                     (0, -1, From::Left(steps + 1)),
//                     (1, 0, From::Above(1)),
//                     (-1, 0, From::Below(1)),
//                 ]
//             }
//         };

//         for (pos_y, pos_x, next_from) in next_positions {
//             let new_y = y + pos_y;
//             let new_x = x + pos_x;
//             let dir_val = match next_from {
//                 From::Below(val) => val,
//                 From::Above(val) => val,
//                 From::Left(val) => val,
//                 From::Right(val) => val,
//             };
//             if new_x < 0
//                 || new_y < 0
//                 || new_x >= width as i32
//                 || new_y >= height as i32
//                 || dir_val > 2
//                 || path_heat[y as usize][x as usize] + heatloss[new_y as usize][new_x as usize]
//                     > path_heat[new_y as usize][new_x as usize]
//             {
//                 continue;
//             }
//             path_heat[new_y as usize][new_x as usize] =
//                 path_heat[y as usize][x as usize] + heatloss[new_y as usize][new_x as usize];
//             next_path.push_back((new_y, new_x, next_from));
//         }
//     }

//     println!("PATHS: ");
//     for p in &path_heat {
//         println!("{:?}", p);
//     }
//     path_heat[height - 1][width - 1] - heatloss[0][0]
// }

fn part1(reader: BufReader<File>) -> i32 {
    let heatloss: Vec<Vec<u8>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let height = heatloss.len() as i32;
    let width = heatloss[0].len() as i32;

    let mut next_position: BinaryHeap<(i32, (i32, i32, (i8, i8)))> = BinaryHeap::new();
    let mut costs: HashMap<(i32, i32, (i8, i8)), i32> = HashMap::new();

    next_position.push((0, (0, 0, (0, 0))));

    while let Some((cost, (y, x, dir))) = next_position.pop() {
        if y == height - 1 && x == width - 1 {
            return -cost;
        }
        if let Some(existing_cost) = costs.get(&(y, x, dir)) {
            if -existing_cost < -cost {
                continue;
            }
        }
        for (move_y, move_x) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (move_y, move_x) == dir || (-move_y, -move_x) == dir {
                continue;
            }
            let mut new_cost = cost;
            for i in 1..=3 {
                let new_y = y + (move_y as i32 * i);
                let new_x = x + (move_x as i32 * i);

                if new_x < 0 || new_y < 0 || new_x >= width || new_y >= height {
                    break;
                }
                new_cost -= (heatloss[new_y as usize][new_x as usize]) as i32;
                let key = (new_y, new_x, (move_y, move_x));
                if new_cost > *costs.get(&key).unwrap_or(&-99999999) {
                    costs.insert(key, new_cost);
                    next_position.push((new_cost, key))
                }
            }
        }
    }
    panic!("should not get here")
}

fn part2(reader: BufReader<File>) -> i32 {
    let heatloss: Vec<Vec<u8>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let height = heatloss.len() as i32;
    let width = heatloss[0].len() as i32;

    let mut next_position: BinaryHeap<(i32, (i32, i32, (i8, i8)))> = BinaryHeap::new();
    let mut costs: HashMap<(i32, i32, (i8, i8)), i32> = HashMap::new();

    next_position.push((0, (0, 0, (0, 0))));

    while let Some((cost, (y, x, dir))) = next_position.pop() {
        if y == height - 1 && x == width - 1 {
            return -cost;
        }
        if let Some(existing_cost) = costs.get(&(y, x, dir)) {
            if -existing_cost < -cost {
                continue;
            }
        }
        for (move_y, move_x) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (move_y, move_x) == dir || (-move_y, -move_x) == dir {
                continue;
            }
            let mut new_cost = cost;
            for i in 1..=10 {
                let new_y = y + (move_y as i32 * i);
                let new_x = x + (move_x as i32 * i);
                if new_x < 0 || new_y < 0 || new_x >= width || new_y >= height {
                    break;
                }
                new_cost -= (heatloss[new_y as usize][new_x as usize]) as i32;

                if i < 4 {
                    continue;
                } 

                let key = (new_y, new_x, (move_y, move_x));
                if new_cost > *costs.get(&key).unwrap_or(&-99999999) {
                    costs.insert(key, new_cost);
                    next_position.push((new_cost, key))
                }
            }
        }
    }
    panic!("should not get here")
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day17.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
