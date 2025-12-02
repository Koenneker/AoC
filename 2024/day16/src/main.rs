use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::{self, BufRead, BufReader};

use regex::Error;

fn main() -> Result<(), io::Error> {
    /*
     */
    let count = day16_1("src/example.txt")?;
    println!("day16_1 Example: {}", count);
    println!("");
    let count = day16_1("src/example_2.txt")?;
    println!("day16_1 Example: {}", count);
    println!("");
    let count = day16_1("src/input.txt")?;
    println!("day16_1 input : {}", count);
    println!("");

    /*
     */
    let count = day16_2("src/example.txt")?;
    println!("day16_2 Small Example: {}", count);
    println!("");
    let count = day16_2("src/example_2.txt")?;
    println!("day16_2 Example: {}", count);
    println!("");
    let count = day16_2("src/input.txt")?;
    println!("day16_2 input : {}", count);
    println!("");
    /*
     */

    Ok(())
}

fn day16_1(filename: &str) -> Result<usize, io::Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let string: Vec<char> = line.chars().collect();
        grid.push(string)
    }

    let mut reindeer_position: (usize, usize) = (0, 0);
    let mut exit_position: (usize, usize) = (0, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, char) in row.iter().enumerate() {
            if *char == 'S' {
                reindeer_position = (column_index, row_index);
            }
            if *char == 'E' {
                exit_position = (column_index, row_index);
            }
        }
    }

    print_grid(&grid);

    let (score, path) = find_a_star_path(reindeer_position, exit_position, &grid);
    let visual_grid = visualize_path(&grid, &path);

    println!("Path visualization:");
    print_grid(&visual_grid);

    Ok(score as usize)
}

fn day16_2(filename: &str) -> Result<usize, io::Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let string: Vec<char> = line.chars().collect();
        grid.push(string)
    }

    let mut reindeer_position: (usize, usize) = (0, 0);
    let mut exit_position: (usize, usize) = (0, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, char) in row.iter().enumerate() {
            if *char == 'S' {
                reindeer_position = (column_index, row_index);
            }
            if *char == 'E' {
                exit_position = (column_index, row_index);
            }
        }
    }

    print_grid(&grid);

    let (_, paths) = find_all_best_paths(reindeer_position, exit_position, &grid);

    let (score, filled_grid) = visualize_all_paths(&grid, &paths);
    print_grid(&filled_grid);

    Ok(score as usize)
}

fn visualize_all_paths(
    grid: &Vec<Vec<char>>,
    paths: &Vec<Vec<((usize, usize), (i32, i32))>>,
) -> (usize, Vec<Vec<char>>) {
    let mut visual_grid = grid.clone();
    let all_positions: HashSet<(usize, usize)> = paths
        .iter()
        .flat_map(|path| path.iter().map(|&(pos, _)| pos))
        .collect();

    let score = all_positions.len();

    for pos in all_positions {
        visual_grid[pos.1][pos.0] = 'O';
    }

    (score, visual_grid)
}

fn find_all_best_paths(
    start: (usize, usize),
    goal: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> (i32, Vec<Vec<((usize, usize), (i32, i32))>>) {
    let mut open_set: HashSet<((usize, usize), (i32, i32))> = HashSet::new();
    open_set.insert((start, (1, 0)));

    let mut came_from: HashMap<((usize, usize), (i32, i32)), Vec<((usize, usize), (i32, i32))>> =
        HashMap::new();

    let mut g_score: HashMap<((usize, usize), (i32, i32)), i32> = HashMap::new();
    g_score.insert((start, (1, 0)), 0);

    let mut f_score: HashMap<((usize, usize), (i32, i32)), i32> = HashMap::new();
    f_score.insert((start, (1, 0)), heuristic(start, goal));

    let mut best_paths: Vec<Vec<((usize, usize), (i32, i32))>> = Vec::new();
    let mut best_score = i32::MAX;

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by_key(|pos| f_score.get(pos).unwrap_or(&i32::MAX))
            .unwrap();

        let current_score = *g_score.get(&current).unwrap();

        if current.0 == goal {
            if current_score <= best_score {
                if current_score < best_score {
                    best_paths.clear();
                    best_score = current_score;
                }
                let paths = reconstruct_all_paths(&came_from, current);
                best_paths.extend(paths);
                open_set.remove(&current);
                continue;
            }
        }

        open_set.remove(&current);

        let current_pos = current.0;
        let current_dir = current.1;

        let possible_moves = vec![
            ((current_pos.0, current_pos.1 + 1), (0, 1)),
            ((current_pos.0, current_pos.1 - 1), (0, -1)),
            ((current_pos.0 + 1, current_pos.1), (1, 0)),
            ((current_pos.0 - 1, current_pos.1), (-1, 0)),
        ];

        for (neighbor, new_dir) in possible_moves {
            if neighbor.1 >= grid.len() || neighbor.0 >= grid[0].len() {
                continue;
            }
            if grid[neighbor.1][neighbor.0] == '#' {
                continue;
            }

            let move_cost = match current_dir {
                dir => {
                    if dir == new_dir {
                        1
                    } else if dir == (-new_dir.0, -new_dir.1) {
                        2001
                    } else {
                        1001
                    }
                }
            };

            let tentative_gscore = current_score + move_cost;
            let neighbor_state = (neighbor, new_dir);

            if tentative_gscore <= *g_score.get(&neighbor_state).unwrap_or(&i32::MAX) {
                if tentative_gscore < *g_score.get(&neighbor_state).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor_state, vec![current]);
                } else {
                    came_from.entry(neighbor_state).or_default().push(current);
                }
                g_score.insert(neighbor_state, tentative_gscore);
                f_score.insert(neighbor_state, tentative_gscore + heuristic(neighbor, goal));
                if !open_set.contains(&neighbor_state) {
                    open_set.insert(neighbor_state);
                }
            }
        }
    }

    (best_score, best_paths)
}

fn reconstruct_all_paths(
    came_from: &HashMap<((usize, usize), (i32, i32)), Vec<((usize, usize), (i32, i32))>>,
    current: ((usize, usize), (i32, i32)),
) -> Vec<Vec<((usize, usize), (i32, i32))>> {
    let mut all_paths = Vec::new();

    if !came_from.contains_key(&current) {
        return vec![vec![current]];
    }

    for &prev in came_from.get(&current).unwrap() {
        let sub_paths = reconstruct_all_paths(came_from, prev);
        for mut sub_path in sub_paths {
            sub_path.push(current);
            all_paths.push(sub_path);
        }
    }

    all_paths
}

fn find_a_star_path(
    start: (usize, usize),
    goal: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> (i32, Vec<((usize, usize), (i32, i32))>) {
    let mut open_set: HashSet<((usize, usize), (i32, i32))> = HashSet::new();
    open_set.insert((start, (1, 0)));

    let mut came_from: HashMap<((usize, usize), (i32, i32)), ((usize, usize), (i32, i32))> =
        HashMap::new();

    let mut g_score: HashMap<((usize, usize), (i32, i32)), i32> = HashMap::new();
    g_score.insert((start, (1, 0)), 0);

    let mut f_score: HashMap<((usize, usize), (i32, i32)), i32> = HashMap::new();
    f_score.insert((start, (1, 0)), heuristic(start, goal));

    while !open_set.is_empty() {
        //println!("{:?}", open_set);
        let current = *open_set
            .iter()
            .min_by_key(|pos| f_score.get(pos).unwrap_or(&i32::MAX))
            .unwrap();

        if current.0 == goal {
            let mut path = Vec::new();
            let mut current_state = current;
            path.push(current_state);

            while came_from.contains_key(&current_state) {
                current_state = *came_from.get(&current_state).unwrap();
                path.push(current_state);
            }

            path.reverse();
            return (*g_score.get(&current).unwrap(), path);
        }

        open_set.remove(&current);

        let current_pos = current.0;
        let current_dir = current.1;

        let possible_moves = vec![
            ((current_pos.0, current_pos.1 + 1), (0, 1)),
            ((current_pos.0, current_pos.1 - 1), (0, -1)),
            ((current_pos.0 + 1, current_pos.1), (1, 0)),
            ((current_pos.0 - 1, current_pos.1), (-1, 0)),
        ];

        for (neighbor, new_dir) in possible_moves {
            if neighbor.1 >= grid.len() || neighbor.0 >= grid[0].len() {
                continue;
            }
            if grid[neighbor.1][neighbor.0] == '#' {
                continue;
            }

            let move_cost = match current_dir {
                dir => {
                    if dir == new_dir {
                        1
                    } else if dir == (-new_dir.0, -new_dir.1) {
                        2001
                    } else {
                        1001
                    }
                }
            };

            let tentative_gscore = g_score.get(&current).unwrap() + move_cost;
            let neighbor_state = (neighbor, new_dir);

            if tentative_gscore < *g_score.get(&neighbor_state).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor_state, current);
                g_score.insert(neighbor_state, tentative_gscore);
                f_score.insert(neighbor_state, tentative_gscore + heuristic(neighbor, goal));
                if !open_set.contains(&neighbor_state) {
                    open_set.insert(neighbor_state);
                }
            }
        }
    }
    panic!("Did not find solution");
}

fn visualize_path(
    grid: &Vec<Vec<char>>,
    path: &Vec<((usize, usize), (i32, i32))>,
) -> Vec<Vec<char>> {
    let mut visual_grid = grid.clone();

    for &(pos, dir) in path {
        let arrow = match dir {
            (0, 1) => '^',
            (0, -1) => 'v',
            (1, 0) => '>',
            (-1, 0) => '<',
            _ => '.',
        };

        if visual_grid[pos.1][pos.0] != 'S' && visual_grid[pos.1][pos.0] != 'E' {
            visual_grid[pos.1][pos.0] = arrow;
        }
    }

    visual_grid
}

fn heuristic(node: (usize, usize), goal: (usize, usize)) -> i32 {
    (node.0 as i32 - goal.0 as i32).abs() + (node.1 as i32 - goal.1 as i32).abs()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}
