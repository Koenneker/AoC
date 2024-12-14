use regex::Regex;
use std::io;
use std::{fs, i64};

fn main() -> Result<(), io::Error> {
    /*
    let count = day14_1("src/example.txt", 11, 7)?;
    println!("day14_1 Example: {}", count);
    println!("");
    let count = day14_1("src/input.txt", 101, 103)?;
    println!("day14_1 input : {}", count);
    println!("");
     */

    let count = day14_2("src/input.txt", 101, 103)?;
    println!("day14_2 Input: {}", count);
    /*
    println!("");
     */

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    starting: (i64, i64),
    velocity: (i64, i64),
}

fn day14_1(filename: &str, width: i64, height: i64) -> Result<i64, io::Error> {
    let contents = fs::read_to_string(filename)?;

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots: Vec<Robot> = Vec::new();

    for capture in re.captures_iter(&contents) {
        let robot: Robot = Robot {
            starting: (
                capture[1].parse::<i64>().unwrap(),
                capture[2].parse::<i64>().unwrap(),
            ),
            velocity: (
                capture[3].parse::<i64>().unwrap(),
                capture[4].parse::<i64>().unwrap(),
            ),
        };

        println!("{:?}", robot);
        robots.push(robot);
    }

    let grid = simulate_grid_at_time(width, height, &robots, 100);
    print_grid(&grid);

    Ok(get_score_from_grid(&grid))
}

fn day14_2(filename: &str, width: i64, height: i64) -> Result<i64, io::Error> {
    let contents = fs::read_to_string(filename)?;

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots: Vec<Robot> = Vec::new();

    for capture in re.captures_iter(&contents) {
        let robot: Robot = Robot {
            starting: (
                capture[1].parse::<i64>().unwrap(),
                capture[2].parse::<i64>().unwrap(),
            ),
            velocity: (
                capture[3].parse::<i64>().unwrap(),
                capture[4].parse::<i64>().unwrap(),
            ),
        };

        robots.push(robot);
    }

    let mut longest_row = 0;
    let mut sim_id_of_longest_row = 0;

    for i in 0..10000 {
        let grid = simulate_grid_at_time(width, height, &robots, i);
        let longest_row_in_grid = detect_longest_row_in_grid(&grid);

        if longest_row_in_grid > longest_row {
            longest_row = longest_row_in_grid;
            sim_id_of_longest_row = i;
        }
    }

    let grid = simulate_grid_at_time(width, height, &robots, sim_id_of_longest_row);
    print_grid(&grid);

    Ok(sim_id_of_longest_row as i64)
}

fn detect_longest_row_in_grid(grid: &Vec<Vec<i64>>) -> i32 {
    let mut count = 0;
    let mut max_count = count;
    for row in grid {
        for value in row {
            if *value != 0 {
                count += 1;
                if count > max_count {
                    max_count = count;
                }
            } else {
                count = 0;
            }
        }
    }
    max_count
}

fn simulate_grid_at_time(
    width: i64,
    height: i64,
    robots: &Vec<Robot>,
    time: usize,
) -> Vec<Vec<i64>> {
    let mut grid: Vec<Vec<i64>> = Vec::new();
    for _ in 0..height {
        grid.push(vec![0; width as usize]);
    }

    for robot in robots {
        let position = get_robot_position_at_time(robot, time as i64);
        let actual_x = (((position.0 % width) + width) % width) as usize;
        let actual_y = (((position.1 % height) + height) % height) as usize;
        grid[actual_y][actual_x] += 1;
    }

    grid
}

fn get_robot_position_at_time(robot: &Robot, time: i64) -> (i64, i64) {
    let x: i64 = robot.starting.0 + robot.velocity.0 * time;
    let y: i64 = robot.starting.1 + robot.velocity.1 * time;

    return (x, y);
}

fn get_score_from_grid(grid: &Vec<Vec<i64>>) -> i64 {
    let mut first_quadrant = 0;
    let mut second_quadrant = 0;
    let mut third_quadrant = 0;
    let mut fourth_quadrant = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if y < grid.len() / 2 && x < row.len() / 2 {
                first_quadrant += value;
            }
            if y < grid.len() / 2 && x > row.len() / 2 {
                second_quadrant += value;
            }
            if y > grid.len() / 2 && x < row.len() / 2 {
                third_quadrant += value;
            }
            if y > grid.len() / 2 && x > row.len() / 2 {
                fourth_quadrant += value;
            }
        }
    }

    first_quadrant * second_quadrant * third_quadrant * fourth_quadrant
}

fn print_grid(grid: &Vec<Vec<i64>>) {
    for row in grid {
        for value in row {
            if *value == 0 {
                print!(".")
            } else {
                print!("{}", value.to_string());
            }
        }
        println!("");
    }
    println!("");
}
