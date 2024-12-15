use std::fs::File;
use std::io::{self, BufRead, BufReader};

use regex::Error;

fn main() -> Result<(), io::Error> {
    /*
    let count = day15_1("src/smallEx.txt")?;
    println!("day15_1 Example: {}", count);
    println!("");
    let count = day15_1("src/example.txt")?;
    println!("day15_1 Example: {}", count);
    println!("");
    let count = day15_1("src/input.txt")?;
    println!("day15_1 input : {}", count);
    println!("");
     */

    /*
    let count = day15_2("src/smallEx_2.txt")?;
    println!("day15_2 Small Example: {}", count);
    println!("");
    let count = day15_2("src/example.txt")?;
    println!("day15_2 Example: {}", count);
    println!("");
    */
    let count = day15_2("src/input.txt")?;
    println!("day15_2 input : {}", count);
    println!("");
    /*
     */

    Ok(())
}

fn day15_1(filename: &str) -> Result<usize, io::Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut field = true;

    for line in reader.lines() {
        if field {
            let line = line?;
            if line == "" {
                field = false;
            }
            let string: Vec<char> = line.chars().collect();
            grid.push(string)
        } else {
            let line = line?;
            let mut string: Vec<char> = line.chars().collect();
            operations.append(&mut string);
        }
    }

    //print_grid(&grid);
    //println!("{:?}", operations);
    let mut robot_position: (usize, usize) = (0, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, char) in row.iter().enumerate() {
            if *char == '@' {
                robot_position = (row_index, column_index);
            }
        }
    }

    let mut grid_pointer = &mut grid;
    for operation in operations {
        (robot_position, grid_pointer) = apply_move(grid_pointer, robot_position, operation);
        //print_grid(&*grid_pointer);
    }

    let score = calculate_gps(&*grid_pointer);
    print_grid(&*grid_pointer);
    Ok(score)
}

fn day15_2(filename: &str) -> Result<usize, io::Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut field = true;

    for line in reader.lines() {
        if field {
            let line = line?;
            if line == "" {
                field = false;
            }
            let string: Vec<char> = line.chars().collect();
            grid.push(string)
        } else {
            let line = line?;
            let mut string: Vec<char> = line.chars().collect();
            operations.append(&mut string);
        }
    }

    grid = convert_map_for_second_star(&grid);

    let mut robot_position: (usize, usize) = (0, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, char) in row.iter().enumerate() {
            if *char == '@' {
                robot_position = (column_index, row_index);
            }
        }
    }

    let mut grid_pointer = &mut grid;
    for operation in operations {
        println!("Next move: {}", operation);
        print_grid(&*grid_pointer);
        (robot_position, grid_pointer) = apply_large_move(grid_pointer, robot_position, operation);
    }

    let score = calculate_gps(&*grid_pointer);
    print_grid(&*grid_pointer);
    Ok(score)
}

fn apply_large_move(
    grid: &mut Vec<Vec<char>>,
    robot_position: (usize, usize),
    operation: char,
) -> ((usize, usize), &mut Vec<Vec<char>>) {
    let direction: (i32, i32);
    match operation {
        '^' => direction = (0, -1),
        '>' => direction = (1, 0),
        'v' => direction = (0, 1),
        '<' => direction = (-1, 0),
        _ => {
            panic!("Robot not there");
        }
    }

    //Check if the robot is at the field
    if grid[robot_position.1][robot_position.0] != '@' {
        print_grid(grid);
        print!("{}", grid[robot_position.1][robot_position.0]);
        panic!("Robot not there");
    }

    let mut field_to_move_into = (
        (robot_position.0 as i32 + direction.0) as usize,
        (robot_position.1 as i32 + direction.1) as usize,
    );

    if grid[field_to_move_into.1][field_to_move_into.0] == '['
        && check_if_large_box_is_moveable(grid, direction, field_to_move_into)
    {
        move_large_box(grid, direction, field_to_move_into);
    } else if grid[field_to_move_into.1][field_to_move_into.0] == ']'
        && check_if_large_box_is_moveable(
            grid,
            direction,
            (field_to_move_into.0 - 1, field_to_move_into.1),
        )
    {
        move_large_box(
            grid,
            direction,
            (field_to_move_into.0 - 1, field_to_move_into.1),
        );
    }

    match grid[field_to_move_into.1][field_to_move_into.0] {
        '.' => {
            grid[field_to_move_into.1][field_to_move_into.0] = '@';
            grid[robot_position.1][robot_position.0] = '.';
        }
        '#' | ']' | '[' => field_to_move_into = robot_position,
        _ => {
            print_grid(grid);
            panic!(
                "Unexpected Char: '{}' in Grid",
                grid[field_to_move_into.1][field_to_move_into.0]
            )
        }
    }

    (field_to_move_into, grid)
}

fn apply_move(
    grid: &mut Vec<Vec<char>>,
    robot_position: (usize, usize),
    operation: char,
) -> ((usize, usize), &mut Vec<Vec<char>>) {
    let direction: (i32, i32);
    match operation {
        '^' => direction = (0, -1),
        '>' => direction = (1, 0),
        'v' => direction = (0, 1),
        '<' => direction = (-1, 0),
        _ => {
            panic!("Robot not there");
        }
    }

    //Check if the robot is at the field
    if grid[robot_position.1][robot_position.0] != '@' {
        panic!("Robot not there");
    }

    let mut field_to_move_into = (
        (robot_position.0 as i32 + direction.0) as usize,
        (robot_position.1 as i32 + direction.1) as usize,
    );

    match grid[field_to_move_into.1][field_to_move_into.0] {
        '.' => {
            grid[field_to_move_into.1][field_to_move_into.0] = '@';
            grid[robot_position.1][robot_position.0] = '.';
        }
        'O' => {
            let mut o_index = (field_to_move_into.0, field_to_move_into.1);
            while grid[o_index.1][o_index.0] == 'O' {
                o_index = (
                    (o_index.0 as i32 + direction.0) as usize,
                    (o_index.1 as i32 + direction.1) as usize,
                );
            }
            if grid[o_index.1][o_index.0] == '.' {
                grid[o_index.1][o_index.0] = 'O';
                grid[field_to_move_into.1][field_to_move_into.0] = '@';
                grid[robot_position.1][robot_position.0] = '.';
            } else if grid[o_index.1][o_index.0] == '#' {
                field_to_move_into = robot_position;
            } else {
                println!("Unexpected Char: '{}' in Grid", grid[o_index.1][o_index.0]);
            }
        }
        '#' => field_to_move_into = robot_position,
        _ => panic!(
            "Unexpected Char: '{}' in Grid",
            grid[field_to_move_into.1][field_to_move_into.0],
        ),
    }

    (field_to_move_into, grid)
}

fn check_if_large_box_is_moveable(
    grid: &Vec<Vec<char>>,
    direction: (i32, i32),
    box_coords: (usize, usize),
) -> bool {
    match direction {
        (0, 1) | (0, -1) => check_if_large_box_is_moveable_vertical(grid, direction, box_coords),
        (1, 0) | (-1, 0) => check_if_large_box_is_moveable_lateral(grid, direction, box_coords),
        _ => panic!("Unknown direction: {:?}", direction),
    }
}

fn check_if_large_box_is_moveable_lateral(
    grid: &Vec<Vec<char>>,
    direction: (i32, i32),
    box_coords: (usize, usize),
) -> bool {
    let neighbor_to_move: (usize, usize);

    match direction {
        (1, 0) => neighbor_to_move = (box_coords.0 + 2, box_coords.1),
        (-1, 0) => neighbor_to_move = (box_coords.0 - 1, box_coords.1),
        _ => panic!("Unknown move: {:?} in lateral", direction),
    }

    let moveable: bool;
    match grid[neighbor_to_move.1][neighbor_to_move.0] {
        '.' => moveable = true,
        '#' => moveable = false,
        ']' => {
            moveable = check_if_large_box_is_moveable_lateral(
                grid,
                direction,
                (neighbor_to_move.0 - 1, neighbor_to_move.1),
            )
        }
        '[' => moveable = check_if_large_box_is_moveable_lateral(grid, direction, neighbor_to_move),
        _ => panic!(
            "Unknown char: {:?} when trying to move lateral",
            grid[neighbor_to_move.1][neighbor_to_move.0]
        ),
    }

    moveable
}

fn check_if_large_box_is_moveable_vertical(
    grid: &Vec<Vec<char>>,
    direction: (i32, i32),
    box_coords: (usize, usize),
) -> bool {
    println!(
        "Called with direction: {:?}, box_coords:{:?}",
        direction, box_coords
    );
    if grid[box_coords.1][box_coords.0] != '[' {
        panic!("Box not present at {:?}", (box_coords.1, box_coords.0));
    }

    let box_left = box_coords;
    let box_right = (box_coords.0 + 1, box_coords.1);

    let above_left = grid[(box_left.1 as i32 + direction.1) as usize]
        [(box_left.0 as i32 + direction.0) as usize];
    let above_right = grid[(box_right.1 as i32 + direction.1) as usize]
        [(box_right.0 as i32 + direction.0) as usize];

    let left_movable: bool;
    let right_movable: bool;

    match above_left {
        '.' => left_movable = true,
        '[' => {
            left_movable = check_if_large_box_is_moveable(
                grid,
                direction,
                (
                    ((box_left.0 as i32 + direction.0) as usize),
                    ((box_left.1 as i32 + direction.1) as usize),
                ),
            )
        }
        ']' => {
            left_movable = check_if_large_box_is_moveable(
                grid,
                direction,
                (
                    ((box_left.0 as i32 + direction.0) as usize - 1),
                    ((box_left.1 as i32 + direction.1) as usize),
                ),
            )
        }
        _ => left_movable = false,
    }

    match above_right {
        '.' => right_movable = true,
        '[' => {
            right_movable = check_if_large_box_is_moveable(
                grid,
                direction,
                (
                    ((box_right.0 as i32 + direction.0) as usize),
                    ((box_right.1 as i32 + direction.1) as usize),
                ),
            )
        }
        ']' => right_movable = true,
        _ => right_movable = false,
    }

    left_movable && right_movable
}

fn move_large_box(
    grid: &mut Vec<Vec<char>>,
    direction: (i32, i32),
    box_coords: (usize, usize),
) -> &mut Vec<Vec<char>> {
    match direction {
        (0, 1) | (0, -1) => move_large_box_vertically(grid, direction, box_coords),
        (1, 0) | (-1, 0) => move_large_box_laterally(grid, direction, box_coords),
        _ => panic!("Unknown direcion: {:?}", direction),
    }
}

fn move_large_box_laterally(
    grid: &mut Vec<Vec<char>>,
    direction: (i32, i32),
    box_coords: (usize, usize),
) -> &mut Vec<Vec<char>> {
    println!(
        "Called move_lateral with dir: {:?}, box: {:?}",
        direction, box_coords
    );
    if grid[box_coords.1][box_coords.0] != '[' {
        panic!("Box not present at {:?}", (box_coords.0, box_coords.1));
    }

    let new_left: (usize, usize);
    let new_right: (usize, usize);

    match direction {
        (1, 0) => {
            let currently_in_place = grid[box_coords.1][box_coords.0 + 2];
            if currently_in_place == '[' {
                move_large_box_laterally(grid, direction, (box_coords.0 + 2, box_coords.1));
            }
            new_left = (box_coords.0 + 1, box_coords.1);
            new_right = (box_coords.0 + 2, box_coords.1);
            grid[box_coords.1][box_coords.0] = '.';
            grid[new_left.1][new_left.0] = '[';
            grid[new_right.1][new_right.0] = ']';
        }
        (-1, 0) => {
            let currently_in_place = grid[box_coords.1][box_coords.0 - 1];
            println!("{}", currently_in_place);
            if currently_in_place == ']' {
                move_large_box_laterally(grid, direction, (box_coords.0 - 2, box_coords.1));
            }
            new_left = (box_coords.0 - 1, box_coords.1);
            new_right = (box_coords.0, box_coords.1);
            grid[box_coords.1][box_coords.0 + 1] = '.';
            grid[new_left.1][new_left.0] = '[';
            grid[new_right.1][new_right.0] = ']';
        }
        _ => panic!("Unknown move: {:?} in lateral move", direction),
    }
    grid
}

fn move_large_box_vertically(
    grid: &mut Vec<Vec<char>>,
    direction: (i32, i32),
    box_coords: (usize, usize),
) -> &mut Vec<Vec<char>> {
    println!(
        "Got called with dir:{:?}, box_coords:{:?}",
        direction, box_coords
    );
    if grid[box_coords.1][box_coords.0] != '[' {
        panic!("Box not present at {:?}", (box_coords.1, box_coords.0));
    }

    let box_left = box_coords;
    let box_right = (box_coords.0 + 1, box_coords.1);

    let above_left = grid[(box_left.1 as i32 + direction.1) as usize]
        [(box_left.0 as i32 + direction.0) as usize];
    let above_right = grid[(box_right.1 as i32 + direction.1) as usize]
        [(box_right.0 as i32 + direction.0) as usize];

    match above_left {
        '[' => move_large_box_vertically(
            grid,
            direction,
            (
                ((box_left.0 as i32 + direction.0) as usize),
                ((box_left.1 as i32 + direction.1) as usize),
            ),
        ),
        ']' => move_large_box_vertically(
            grid,
            direction,
            (
                ((box_left.0 as i32 + direction.0) as usize - 1),
                ((box_left.1 as i32 + direction.1) as usize),
            ),
        ),
        _ => grid,
    };

    match above_right {
        '[' => move_large_box_vertically(
            grid,
            direction,
            (
                ((box_right.0 as i32 + direction.0) as usize),
                ((box_right.1 as i32 + direction.1) as usize),
            ),
        ),
        _ => grid,
    };

    let above_left = grid[(box_left.1 as i32 + direction.1) as usize]
        [(box_left.0 as i32 + direction.0) as usize];
    let above_right = grid[(box_right.1 as i32 + direction.1) as usize]
        [(box_right.0 as i32 + direction.0) as usize];

    if above_left == '.' && above_right == '.' {
        grid[(box_left.1 as i32 + direction.1) as usize]
            [(box_left.0 as i32 + direction.0) as usize] = '[';
        grid[(box_right.1 as i32 + direction.1) as usize]
            [(box_right.0 as i32 + direction.0) as usize] = ']';
        grid[box_left.1][box_left.0] = '.';
        grid[box_right.1][box_right.0] = '.';
    }

    grid
}

fn convert_map_for_second_star(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = Vec::new();

    for row in grid {
        let mut new_row: Vec<char> = Vec::new();
        for char in row {
            match char {
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => todo!(),
            }
        }
        new_grid.push(new_row);
    }

    new_grid
}

fn calculate_gps(grid: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, char) in row.iter().enumerate() {
            if *char == 'O' {
                score += 100 * row_index + column_index;
            }
            if *char == '[' {
                score += 100 * row_index + column_index;
            }
        }
    }
    score
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}
