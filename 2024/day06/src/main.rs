use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    /*let count = day06_1("src/example.txt")?;
    println!("Day05_1 Test: {}", count);
    let count = day06_1("src/input.txt")?;
    println!("Day06_1 Input: {}", count);
    println!("");

    let count = day06_2("src/example.txt")?;
    println!("Day05_2 Test: {}", count);*/
    let count = day06_2("src/input.txt")?;
    println!("Day05_2 Input: {}", count);
    println!("");

    Ok(())
}

fn day06_1(filename: &str) -> Result<usize, io::Error> {
    let mut char_vec_vec: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let string: Vec<char> = line.chars().collect();
        char_vec_vec.push(string)
    }

    while let Some((i, j)) = char_vec_vec
        .iter()
        .enumerate()
        .find_map(|(i, inner_vec)| inner_vec.iter().position(|&c| c == '^').map(|j| (i, j)))
    {
        if i == 0 {
            char_vec_vec[i][j] = 'X';
        } else {
            let char_above = char_vec_vec[i - 1][j];
            if char_above == '#' {
                char_vec_vec = rotate_270(&char_vec_vec);
            } else {
                char_vec_vec[i - 1][j] = '^';
                char_vec_vec[i][j] = 'X';
            }
        }
    }
    let count = char_vec_vec
        .iter()
        .flat_map(|inner_vec| inner_vec.iter())
        .filter(|&&c| c == 'X')
        .count();

    Ok(count)
}

fn day06_2(filename: &str) -> Result<usize, io::Error> {
    let mut char_vec_vec: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let mut first_line: Vec<char> = Vec::new();
        let mut second_line: Vec<char> = Vec::new();
        let mut third_line: Vec<char> = Vec::new();

        for char in line.chars() {
            first_line = vec![first_line, vec!['0', '0', '0']].concat();
            second_line = vec![second_line, vec!['0', char, '0']].concat();
            third_line = vec![third_line, vec!['0', '0', '0']].concat();
        }

        char_vec_vec.push(first_line);
        char_vec_vec.push(second_line);
        char_vec_vec.push(third_line);
    }

    let plain_char_vec_vec = char_vec_vec.clone();
    let mut rotations = 0;

    while let Some((i, j)) = char_vec_vec
        .iter()
        .enumerate()
        .find_map(|(i, inner_vec)| inner_vec.iter().position(|c| *c == '^').map(|j| (i, j)))
    {
        if i == 1 {
            char_vec_vec[i - 1][j] = '1';
            char_vec_vec[i][j] = 'X';
        } else {
            let char_above = char_vec_vec[i - 3][j].clone();
            if char_above == '#' {
                char_vec_vec[i - 1][j] = '1';
                rotations += 1;
                char_vec_vec = rotate_270(&char_vec_vec);
            } else {
                char_vec_vec[i - 3][j] = '^';
                char_vec_vec[i - 1][j] = '1';
                char_vec_vec[i][j] = 'X';
            }
        }
    }

    let mut sum = 0;

    for _ in 0..(4 - (rotations % 4)) {
        char_vec_vec = rotate_270(&char_vec_vec);
    }

    let mut positions = Vec::new();
    for (x, row) in char_vec_vec.iter().enumerate() {
        for (y, &ch) in row.iter().enumerate() {
            if ch == 'X' {
                positions.push((y, x));
            }
        }
    }

    let sum = positions
        .par_iter()
        .filter(|(y, x)| {
            let mut test_grid = plain_char_vec_vec.clone();
            test_grid[*x][*y] = 'O';
            ends_in_loop(test_grid)
        })
        .count();

    Ok(sum)
}

fn ends_in_loop(mut char_vec_vec: Vec<Vec<char>>) -> bool {
    let mut rotations = 0;
    //print_grid_compact(&char_vec_vec);
    while let Some((i, j)) = char_vec_vec
        .iter()
        .enumerate()
        .find_map(|(i, inner_vec)| inner_vec.iter().position(|c| *c == '^').map(|j| (i, j)))
    {
        if i == 1 {
            char_vec_vec[i][j] = 'X';
            char_vec_vec[0][j] = '1';
        } else {
            let char_above = &char_vec_vec[i - 3][j];
            if char_vec_vec[i - 1][j] == '1' {
                for _ in 0..(4 - (rotations % 4)) {
                    char_vec_vec = rotate_270(&char_vec_vec);
                }
                //print_grid_compact(&char_vec_vec);

                return true;
            } else if *char_above == '#' || *char_above == 'O' {
                char_vec_vec[i - 1][j] = '1';
                char_vec_vec = rotate_270(&char_vec_vec);
                rotations += 1;
            } else {
                char_vec_vec[i - 3][j] = '^';
                char_vec_vec[i - 1][j] = '1';
                char_vec_vec[i][j] = 'X';
            }
        }
    }
    for _ in 0..(4 - (rotations % 4)) {
        char_vec_vec = rotate_270(&char_vec_vec);
    }
    //print_grid_compact(&char_vec_vec);
    return false;
}

fn rotate_270_for_vec_vec_vec_vec(
    char_vec_vec_vec_vec: &Vec<Vec<Vec<Vec<char>>>>,
) -> Vec<Vec<Vec<Vec<char>>>> {
    let rows = char_vec_vec_vec_vec.len();
    let cols = char_vec_vec_vec_vec[0].len();
    let mut rotated = vec![vec![char_vec_vec_vec_vec[0][0].clone(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[cols - 1 - j][i] = rotate_270(&char_vec_vec_vec_vec[i][j]);
        }
    }

    rotated
}

fn rotate_270(char_vec_vec: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = char_vec_vec.len();
    let cols = char_vec_vec[0].len();
    let mut rotated = vec![vec![char_vec_vec[0][0].clone(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[cols - 1 - j][i] = char_vec_vec[i][j].clone();
        }
    }

    rotated
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}

fn print_grid_compact(grid: &Vec<Vec<char>>) {
    for row in grid {
        let string = row
            .iter()
            .filter(|x| (**x != '0') && (**x != '1'))
            .collect::<String>();
        if string != "" {
            println!("{}", string);
        }
    }
    println!("");
}
