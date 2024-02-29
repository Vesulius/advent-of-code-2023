use num::Integer;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1(reader: BufReader<File>) -> u64 {
    let map: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| l.chars().collect())
        .collect();

    let width = map[0].len();
    let height = map.len();
    let mut start = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'S' {
                start = (y, x);
                break;
            }
        }
    }

    let mut weights = vec![vec![99999999; width]; height];
    let mut next_pos: VecDeque<(usize, usize)> = VecDeque::new();

    // start = (65, 65);
    println!("start {:?}", start);
    next_pos.push_back(start);
    weights[start.0][start.1] = 0;

    while let Some((y, x)) = next_pos.pop_front() {
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_y = y as i32 + dir.0;
            let new_x = x as i32 + dir.1;
            if new_y < 0
                || new_x < 0
                || new_y >= height as i32
                || new_x >= width as i32
                || map[new_y as usize][new_x as usize] == '#'
                || weights[y][x] >= 64
                || weights[new_y as usize][new_x as usize] <= weights[y][x] + 1
            {
                continue;
            }
            weights[new_y as usize][new_x as usize] = weights[y][x] + 1;
            next_pos.push_back((new_y as usize, new_x as usize));
        }
    }
    let mut sum = 0;
    for w in weights {
        for v in w {
            if v.is_even() {
                sum += 1;
            }
            if v == 99999999 {
                print!(" M");
            } else {
                print!(" {}", v);
            }
        }
        println!();
    }
    sum
}

fn part2(reader: BufReader<File>) -> u64 {
    /*
        To solve this problem, we will use little bit of knowlage of the map: we dont have any rocks (#)
        on the same column and row where S is. This means that the shortest path to the "next" map is
        allways straight direction. So the entrypoint to the other maps is always one of the points on
        this shortest path.

        The width and height of the map is 131. We can walk upto 26501365 steps. To get out of the first
        map we walk 65 steps. Now we can calculate how many maps we pass (26501365-65)/131=202300

        As we are using manhattan distance, the walkable area form a diamond shape:

           #
          ###
         #####
        #######
         #####
          ###
           #

        For the all the "edge maps" we have 131 steps to go on from the first point.

        Because the maps are odd width and height, in every other map the entrypoint will be odd and other
        even. Here is a map of maps based on if the center is even or odd step:

            E
           EOE
          EOEOE
         EOEOEOE
        EOEOEOEOE
         EOEOEOE
          EOEOE
           EOE
            E

    */

    let map: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| l.chars().collect())
        .collect();

    let width = map[0].len();
    let height = map.len();

    println!("width {}, halved {}", width, (width - 1) / 2);
    println!("height {}, halved {}", height, (height - 1) / 2);

    fn get_steps(steps: i32, start_y: usize, start_x: usize, map: &Vec<Vec<char>>) -> (u64, u64) {
        let width = map[0].len();
        let height = map.len();
        let mut weights = vec![vec![99999999; width]; height];
        let mut next_pos: VecDeque<(usize, usize)> = VecDeque::new();
        next_pos.push_back((start_y, start_x));
        weights[start_y][start_x] = 0;

        while let Some((y, x)) = next_pos.pop_front() {
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_y = y as i32 + dir.0;
                let new_x = x as i32 + dir.1;
                if new_y < 0
                    || new_x < 0
                    || new_y >= height as i32
                    || new_x >= width as i32
                    || map[new_y as usize][new_x as usize] == '#'
                    || weights[y][x] >= steps
                    || weights[new_y as usize][new_x as usize] <= weights[y][x] + 1
                {
                    continue;
                }
                weights[new_y as usize][new_x as usize] = weights[y][x] + 1;
                next_pos.push_back((new_y as usize, new_x as usize));
            }
        }
        let mut even = 0;
        let mut odd = 0;
        for w in weights {
            for v in w {
                if v != 99999999 {
                    if v.is_even() {
                        even += 1;
                    } else {
                        odd += 1;
                    }
                }
            }
        }
        (even, odd)
    }

    let (full_even_steps, full_odd_steps) = get_steps(999999, 65, 65, &map);
    println!("full even {} full odd {}", full_even_steps, full_odd_steps);

    let (top_even_steps, _) = get_steps(131, 130, 65, &map);
    let (bottom_even_steps, _) = get_steps(131, 0, 65, &map);
    let (left_even_steps, _) = get_steps(131, 65, 130, &map);
    let (right_even_steps, _) = get_steps(131, 65, 0, &map);

    let (north_east_small, _) = get_steps(65, 130, 0, &map);
    let (_, north_east_big) = get_steps(195, 130, 0, &map);

    let (south_east_small, _) = get_steps(65, 0, 0, &map);
    let (_, south_east_big) = get_steps(195, 0, 0, &map);

    let (south_west_small, _) = get_steps(65, 0, 130, &map);
    let (_, south_west_big) = get_steps(195, 0, 130, &map);

    let (north_west_small, _) = get_steps(65, 130, 130, &map);
    let (_, north_west_big) = get_steps(195, 130, 130, &map);

    let size_of_inside_squares_side_len = (26501365 - 65) / 131;

    let mut sum = 0;

    // add top, bottom, left, right maps
    sum += top_even_steps + bottom_even_steps + left_even_steps + right_even_steps;

    // corner cut maps
    sum += size_of_inside_squares_side_len
        * (north_east_small + south_east_small + north_west_small + south_west_small);
    sum += (size_of_inside_squares_side_len - 1)
        * (north_east_big + south_east_big + north_west_big + south_west_big);

    // add inside odd maps
    sum += size_of_inside_squares_side_len * size_of_inside_squares_side_len * full_even_steps;

    // add inside even maps
    sum += (size_of_inside_squares_side_len - 1)
        * (size_of_inside_squares_side_len - 1)
        * full_odd_steps;

    sum
}

pub fn run(run_part_2: bool) {
    let file = File::open("./input/day21.txt").unwrap();
    let reader = io::BufReader::new(file);
    if run_part_2 {
        println!("ANSWER: {}", part2(reader));
    } else {
        println!("ANSWER: {}", part1(reader));
    }
}
