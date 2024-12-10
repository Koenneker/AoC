use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let count = day10_1("src/smallEx.txt")?;
    println!("day10_1 smallEx: {}", count);
    let count = day10_1("src/example.txt")?;
    println!("day10_1 Test: {}", count);
    let count = day10_1("src/input.txt")?;
    println!("day10_1 Input: {}", count);
    println!("");
    /*
     */

    let count = day10_2("src/example.txt")?;
    println!("day10_2 Test: {}", count);
    let count = day10_2("src/input.txt")?;
    println!("day10_2 Input: {}", count);
    println!("");
    /*
     */

    Ok(())
}

fn day10_1(filename: &str) -> Result<usize, io::Error> {
    let mut grid: Vec<Vec<usize>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(numbers);
    }

    //print_grid(&grid);

    let mut trailhead_set = HashSet::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 0 {
                trailhead_set.insert((x, y));
            }
        }
    }
    let mut result = 0;
    for trailhead in trailhead_set {
        result += simulate_open_trails(&grid, trailhead);
    }

    Ok(result)
}

fn day10_2(filename: &str) -> Result<usize, io::Error> {
    let mut grid: Vec<Vec<usize>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(numbers);
    }

    //print_grid(&grid);

    let mut trailhead_set = HashSet::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 0 {
                trailhead_set.insert((x, y));
            }
        }
    }
    let mut result = 0;
    for trailhead in trailhead_set {
        result += simulate_open_trails_array(&grid, trailhead);
    }

    Ok(result)
}

fn simulate_open_trails_array(grid: &Vec<Vec<usize>>, trailhead: (usize, usize)) -> usize {
    let mut open_array = Vec::new();
    open_array.push(trailhead);
    //println!("Startet with: {}", open_set.len());

    let mut sum_of_trails = 0;

    let grid_max_x = grid.len() - 1;
    let grid_max_y = grid[0].len() - 1;

    while open_array.len() != 0 {
        let working_array = open_array;
        open_array = Vec::new();

        for (x, y) in working_array {
            let value = grid[x][y];
            if value == 9 {
                sum_of_trails += 1;
                continue;
            } else {
                if x != 0 && grid[x - 1][y] == value + 1 {
                    open_array.push((x - 1, y));
                }
                if y != 0 && grid[x][y - 1] == value + 1 {
                    open_array.push((x, y - 1));
                }
                if x < grid_max_x && grid[x + 1][y] == value + 1 {
                    open_array.push((x + 1, y));
                }
                if y < grid_max_y && grid[x][y + 1] == value + 1 {
                    open_array.push((x, y + 1));
                }
            }
        }

        //println!("{:?}", open_set);
    }

    sum_of_trails
}

fn simulate_open_trails(grid: &Vec<Vec<usize>>, trailhead: (usize, usize)) -> usize {
    let mut open_set = HashSet::new();
    open_set.insert(trailhead);
    //println!("Startet with: {}", open_set.len());

    let mut sum_of_trails = 0;

    let grid_max_x = grid.len() - 1;
    let grid_max_y = grid[0].len() - 1;

    while open_set.len() != 0 {
        let working_set = open_set;
        open_set = HashSet::new();

        for (x, y) in working_set {
            let value = grid[x][y];
            if value == 9 {
                sum_of_trails += 1;
                continue;
            } else {
                if x != 0 && grid[x - 1][y] == value + 1 {
                    open_set.insert((x - 1, y));
                }
                if y != 0 && grid[x][y - 1] == value + 1 {
                    open_set.insert((x, y - 1));
                }
                if x < grid_max_x && grid[x + 1][y] == value + 1 {
                    open_set.insert((x + 1, y));
                }
                if y < grid_max_y && grid[x][y + 1] == value + 1 {
                    open_set.insert((x, y + 1));
                }
            }
        }

        //println!("{:?}", open_set);
    }

    sum_of_trails
}
/*
fn print_grid(grid: &Vec<Vec<usize>>) {
    for row in grid {
        for num in row {
            print!("{} ", num)
        }
        println!("");
    }
    println!("");
}
*/
