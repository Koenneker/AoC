use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    /*
    let count = day11_1("src/example.txt", 6)?;
    println!("day10_1 Test 6: {}", count);
     */
    let count = day11_1("src/input_manu.txt", 25)?;
    println!("day11_1 Test 25: {}", count);
    println!("");

    /*
    let count = day11_2("src/example.txt", 25)?;
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

fn day11_1(filename: &str, number_of_blinks: usize) -> Result<usize, io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let input: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut stones = input;
    for i in 1..=number_of_blinks {
        println!("{}", i);
        //println!("{:?}", stones);
        stones = blink(&stones);
    }
    //println!("{:?}", stones);

    Ok(stones.len())
}

fn day11_2(filename: &str, number_of_blinks: i32) -> Result<i64, io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let input: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let stone_count = input
        .par_iter()
        .map(|stone| blink_n(*stone, number_of_blinks))
        .sum();

    Ok(stone_count)
}

fn day11_3(filename: &str, number_of_blinks: i32) -> Result<i64, io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let input: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let stone_count = input
        .iter()
        .map(|stone| blink_n(*stone, number_of_blinks))
        .sum();

    Ok(stone_count)
}

thread_local! {
    static CACHE: RefCell<HashMap<(i64, i32), i64>> = RefCell::new(HashMap::new());
}

fn blink_n(stone: i64, n: i32) -> i64 {
    CACHE.with(|cache| {
        if let Some(&result) = cache.borrow().get(&(stone, n)) {
            return result;
        }

        let result = if n == 0 {
            1
        } else if stone == 0 {
            blink_n(1, n - 1)
        } else if stone.to_string().len() % 2 == 0 {
            let stone_string = stone.to_string();
            let (first_stone, second_stone) = stone_string.split_at(stone_string.len() / 2);
            blink_n(first_stone.parse().unwrap(), n - 1)
                + blink_n(second_stone.parse().unwrap(), n - 1)
        } else {
            blink_n(stone * 2024, n - 1)
        };

        cache.borrow_mut().insert((stone, n), result);
        result
    })
}

fn blink(input: &Vec<i64>) -> Vec<i64> {
    input
        .par_iter()
        .flat_map(|stone| {
            let mut local_output = Vec::new();
            if *stone == 0 {
                local_output.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                let (first_stone, second_stone) = stone_string.split_at(stone_string.len() / 2);
                local_output.push(first_stone.parse().unwrap());
                local_output.push(second_stone.parse().unwrap());
            } else {
                local_output.push(*stone * 2024);
            }
            local_output
        })
        .collect()
}
