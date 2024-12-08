use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    /*
     */
    let count = day08_1("src/example.txt")?;
    println!("Day08_1 Test: {}", count);
    let count = day08_1("src/input.txt")?;
    println!("Day08_1 Input: {}", count);
    println!("");

    let count = day08_2("src/t-test.txt")?;
    println!("Day08_2 T-Test: {}", count);

    let count = day08_2("src/example.txt")?;
    println!("Day08_2 Test: {}", count);

    let count = day08_2("src/input.txt")?;
    println!("Day07_2 Input: {}", count);
    println!("");

    Ok(())
}

fn day08_1(filename: &str) -> Result<usize, io::Error> {
    let mut char_vec_vec: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let string: Vec<char> = line.chars().collect();
        char_vec_vec.push(string)
    }

    //print_grid(&char_vec_vec);

    let mut grid_vec = split_grid(&char_vec_vec);

    for grid in grid_vec.iter_mut() {
        //print_grid(grid);
        *grid = fill_grid(&grid);
        //print_grid(grid);
    }

    let mut antinode_set: HashSet<(usize, usize)> = HashSet::new();

    for grid in grid_vec {
        for (h_index, row) in grid.iter().enumerate() {
            for (w_index, char) in row.iter().enumerate() {
                if *char == '#' {
                    antinode_set.insert((h_index, w_index));
                }
            }
        }
    }

    Ok(antinode_set.len())
}

fn day08_2(filename: &str) -> Result<usize, io::Error> {
    let mut char_vec_vec: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let string: Vec<char> = line.chars().collect();
        char_vec_vec.push(string)
    }

    //print_grid(&char_vec_vec);

    let mut grid_vec = split_grid(&char_vec_vec);

    //println!("{}", grid_vec.len());

    for grid in grid_vec.iter_mut() {
        //print_grid(grid);
        *grid = fill_grid_antinodes(&grid);
        //print_grid(grid);
    }

    let mut antinode_set: HashSet<(usize, usize)> = HashSet::new();

    for grid in grid_vec {
        for (h_index, row) in grid.iter().enumerate() {
            for (w_index, char) in row.iter().enumerate() {
                if *char == '#' {
                    antinode_set.insert((h_index, w_index));
                }
            }
        }
    }

    Ok(antinode_set.len())
}

fn fill_grid_antinodes(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid_height = grid.len() as i32;
    let grid_width = grid[0].len() as i32;

    let mut coordinate_list: Vec<(i32, i32)> = Vec::new();

    for (h_index, row) in grid.iter().enumerate() {
        for (w_index, char) in row.iter().enumerate() {
            if *char != '.' && *char != '#' {
                coordinate_list.push((h_index as i32, w_index as i32));
            }
        }
    }

    let mut modified_grid = create_empty_grid(grid_width as usize, grid_height as usize);

    for (coord_1, coord_2) in coordinate_list.iter().tuple_combinations() {
        //println!("{:?},{:?}", coord_1, coord_2);

        let h_delta: i32 = coord_2.0 - coord_1.0;
        let w_delta: i32 = coord_2.1 - coord_1.1;

        let mut antinodes = vec![];
        for i in 0..(max(grid_width, grid_height)) {
            antinodes.push((coord_1.0 - (i * h_delta), coord_1.1 - (i * w_delta)));
            antinodes.push((coord_2.0 + (i * h_delta), coord_2.1 + (i * w_delta)));
        }

        //println!("Antinodes: {:#?}", antinodes);

        for (width, height) in antinodes {
            if width >= 0 && width < grid_width && height >= 0 && height < grid_height {
                modified_grid[width as usize][height as usize] = '#';
            }
        }
    }

    modified_grid
}

fn fill_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid_height = grid.len() as i32;
    let grid_width = grid[0].len() as i32;

    let mut modified_grid = grid.clone();

    let mut coordinate_list: Vec<(i32, i32)> = Vec::new();

    for (h_index, row) in grid.iter().enumerate() {
        for (w_index, char) in row.iter().enumerate() {
            if *char != '.' && *char != '#' {
                coordinate_list.push((h_index as i32, w_index as i32));
            }
        }
    }

    for (coord_1, coord_2) in coordinate_list.iter().tuple_combinations() {
        //println!("{:?},{:?}", coord_1, coord_2);

        let h_delta: i32 = coord_2.0 - coord_1.0;
        let w_delta: i32 = coord_2.1 - coord_1.1;

        let antinodes = vec![
            (coord_1.0 - h_delta, coord_1.1 - w_delta),
            (coord_2.0 + h_delta, coord_2.1 + w_delta),
        ];

        for (width, height) in antinodes {
            if width >= 0 && width < grid_width && height >= 0 && height < grid_height {
                modified_grid[width as usize][height as usize] = '#';
            }
        }
    }

    modified_grid
}

fn split_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut grid_indicies: HashMap<char, usize> = HashMap::new();
    let mut grid_vec = vec![];

    for (h_index, row) in grid.iter().enumerate() {
        for (w_index, char) in row.iter().enumerate() {
            if *char == '.' {
                continue;
            }
            let actual_index: usize;
            if let Some(set_index) = grid_indicies.get(char) {
                actual_index = *set_index;
            } else {
                actual_index = grid_indicies.len();
                grid_indicies.insert(*char, actual_index);
                grid_vec.push(create_empty_grid(grid_width, grid_height))
            }

            if let Some(isolated_grid) = grid_vec.get_mut(actual_index) {
                isolated_grid[h_index][w_index] = *char;
            } else {
                println!("Index error");
            }
        }
    }

    return grid_vec;
}

fn create_empty_grid(width: usize, height: usize) -> Vec<Vec<char>> {
    return vec![vec!['.'; width]; height];
}

fn combine_grids(grid_vec: &Vec<Vec<Vec<char>>>) -> (usize, Vec<Vec<char>>) {
    let mut actual_grid: Vec<Vec<char>> =
        create_empty_grid(grid_vec[0].len(), grid_vec[0][0].len());

    let mut corrector = 0;

    for grid in grid_vec {
        for (height_index, row) in grid.iter().enumerate() {
            for (width_index, char) in row.iter().enumerate() {
                if *char != '.'
                    && (actual_grid[height_index][width_index] == '.'
                        || actual_grid[height_index][width_index] == '#')
                {
                    if actual_grid[height_index][width_index] == '#' && *char != '#' {
                        corrector += 1;
                    }
                    actual_grid[height_index][width_index] = *char;
                }
            }
        }
    }

    return (corrector, actual_grid);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}
