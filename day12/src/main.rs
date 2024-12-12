use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    /*
    let count = day12_1("src/smallEx.txt")?;
    println!("day10_1 Test 6: {}", count);
    let count = day12_1("src/example.txt")?;
    println!("day11_1 Test 25: {}", count);
    println!("");
    let count = day12_1("src/input.txt")?;
    println!("day12_1 input : {}", count);
    println!("");
     */

    let count = day12_2("src/smallEx.txt")?;
    println!("day10_2 Test: {}", count);
    /*
    let count = day12_2("src/example.txt")?;
    println!("day10_2 Test: {}", count);
    let count = day11_2("src/input.txt", 1000)?;
    println!("day10_2 Input: {}", count);
    println!("");
    let count = day11_2("src/example.txt", 25)?;
    println!("day10_2 Test: {}", count);
    let count = day11_3("src/input.txt", 1000)?;
    println!("day10_2 Input: {}", count);
     */
    /*
    println!("");
     */

    Ok(())
}

fn day12_1(filename: &str) -> Result<usize, io::Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    //print_grid(&grid);

    let mut unclaimed_map: HashMap<(usize, usize), char> = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (plot_index, plot) in row.iter().enumerate() {
            unclaimed_map.insert((row_index, plot_index), *plot);
        }
    }

    let mut claims: Vec<(char, HashSet<(usize, usize)>)> = Vec::new();

    while unclaimed_map.len() != 0 {
        claims.push(get_region(&mut unclaimed_map));
    }

    //println!("{:?}", claims);

    let mut cost = 0;

    for claim in claims {
        cost += get_region_cost(claim);
    }

    Ok(cost)
}

fn day12_2(filename: &str) -> Result<usize, io::Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    //print_grid(&grid);

    let mut unclaimed_map: HashMap<(usize, usize), char> = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (plot_index, plot) in row.iter().enumerate() {
            unclaimed_map.insert((row_index, plot_index), *plot);
        }
    }

    let mut claims: Vec<(char, HashSet<(usize, usize)>)> = Vec::new();

    while unclaimed_map.len() != 0 {
        claims.push(get_region(&mut unclaimed_map));
    }

    //println!("{:?}", claims);

    let mut cost = 0;

    for claim in claims {
        cost += get_region_cost_with_discount(claim);
    }

    Ok(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut parcels = HashSet::new();
        parcels.insert((0, 0));
        parcels.insert((0, 1));
        parcels.insert((0, 2));
        parcels.insert((0, 3));

        assert_eq!(count_sides(parcels), 4);
    }

    #[test]
    fn test_b() {
        let mut parcels = HashSet::new();
        parcels.insert((0, 0));
        parcels.insert((0, 1));
        parcels.insert((1, 0));
        parcels.insert((1, 1));

        assert_eq!(count_sides(parcels), 4);
    }

    #[test]
    fn test_c() {
        let mut parcels = HashSet::new();
        parcels.insert((0, 0));
        parcels.insert((0, 1));
        parcels.insert((1, 1));
        parcels.insert((1, 2));

        assert_eq!(count_sides(parcels), 8);
    }

    #[test]
    fn test_d() {
        let mut parcels = HashSet::new();
        parcels.insert((0, 0));

        assert_eq!(count_sides(parcels), 4);
    }

    #[test]
    fn test_e() {
        let mut parcels = HashSet::new();
        parcels.insert((0, 0));
        parcels.insert((1, 0));
        parcels.insert((2, 0));

        assert_eq!(count_sides(parcels), 4);
    }
}

fn get_region_cost_with_discount(claim: (char, HashSet<(usize, usize)>)) -> usize {
    let number_of_plots = claim.1.len();

    let sides = count_sides(claim.1);

    println!(
        "Char: {}, Capacity: {}, Sides: {}",
        claim.0, number_of_plots, sides,
    );

    number_of_plots * sides
}

fn get_region_cost(claim: (char, HashSet<(usize, usize)>)) -> usize {
    let number_of_plots = claim.1.len();

    let mut edges = number_of_plots * 4;

    for (x, y) in claim.1.iter() {
        if *x != 0 && claim.1.contains(&(*x - 1, *y)) {
            edges -= 1;
        }
        if claim.1.contains(&(*x + 1, *y)) {
            edges -= 1;
        }
        if *y != 0 && claim.1.contains(&(*x, *y - 1)) {
            edges -= 1;
        }
        if claim.1.contains(&(*x, *y + 1)) {
            edges -= 1;
        }
    }

    number_of_plots * edges
}

fn get_region(
    unclaimed_map: &mut HashMap<(usize, usize), char>,
) -> (char, HashSet<(usize, usize)>) {
    let (&start_pos, &symbol) = unclaimed_map.iter().next().unwrap();
    let mut region = HashSet::new();
    let mut to_check = vec![start_pos];

    while let Some(pos) = to_check.pop() {
        if !region.contains(&pos) && unclaimed_map.get(&pos) == Some(&symbol) {
            region.insert(pos);
            unclaimed_map.remove(&pos);
            let (x, y) = pos;
            let neighbors = [
                (x.wrapping_sub(1), y), // left
                (x + 1, y),             // right
                (x, y.wrapping_sub(1)), // up
                (x, y + 1),             // down
            ];

            for neighbor in neighbors {
                to_check.push(neighbor);
            }
        }
    }

    (symbol, region)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for num in row {
            print!("{} ", num)
        }
        println!("");
    }
    println!("");
}
