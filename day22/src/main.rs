use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    /*
    let count = day22_1("src/example.txt")?;
    println!("day22_1 Example: {}", count);
    println!("");
    let count = day22_1("src/input.txt")?;
    println!("day22_1 input : {}", count);
    println!("");
     */

    let count = day22_2("src/example_2.txt")?;
    println!("day22_2 Small Example: {}", count);
    println!("");
    let count = day22_2("src/input.txt")?;
    println!("day22_2 input : {}", count);
    println!("");
    /*
     */

    Ok(())
}

fn day22_2(filename: &str) -> Result<i64, io::Error> {
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut monkey_numbers: Vec<i64> = Vec::new();

    let re = Regex::new(r"(\d+)").unwrap();
    for capture in re.captures_iter(&input) {
        if let Ok(number) = capture[1].parse::<i64>() {
            monkey_numbers.push(number);
        }
    }

    let mut sequence_map: HashMap<(i8, i8, i8, i8), HashMap<i64, i8>> = HashMap::new();

    monkey_numbers.iter().for_each(|starting_number| {
        let mut secret_number = *starting_number;
        let mut buffer = AllocRingBuffer::new(4);
        buffer.push((secret_number % 10) as i8);

        for _ in 0..3 {
            secret_number = next_sectret(secret_number);
            buffer.push((secret_number % 10) as i8);
        }

        let mut found_sequences = HashMap::new();

        for _ in 3..2000 {
            secret_number = next_sectret(secret_number);
            let current_price = (secret_number % 10) as i8;

            let last_values = (
                buffer[1] - buffer[0],
                buffer[2] - buffer[1],
                buffer[3] - buffer[2],
                current_price - buffer[3],
            );

            if !found_sequences.contains_key(&last_values) {
                found_sequences.insert(last_values, true);

                sequence_map
                    .entry(last_values)
                    .or_insert_with(HashMap::new)
                    .entry(*starting_number)
                    .or_insert(current_price);
            }

            buffer.push(current_price);
        }
    });

    let mut best_banana_sequence: (i8, i8, i8, i8) = (0, 0, 0, 0);
    let mut best_banana_result: i64 = 0;

    for (sequence, result_map) in sequence_map.iter() {
        let bananas_for_sequence: i64 = result_map.values().map(|&x| x as i64).sum();

        if bananas_for_sequence > best_banana_result {
            best_banana_result = bananas_for_sequence;
            best_banana_sequence = *sequence;
        }
    }

    println!("Best sequence: {:?}", best_banana_sequence);
    Ok(best_banana_result)
}

fn day22_1(filename: &str) -> Result<i64, io::Error> {
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut monkey_numbers: Vec<i64> = Vec::new();

    let re = Regex::new(r"(\d+)").unwrap();
    for capture in re.captures_iter(&input) {
        if let Ok(number) = capture[1].parse::<i64>() {
            monkey_numbers.push(number);
        }
    }

    //println!("{:?}", monkey_numbers);

    let mut new_numbers: Vec<i64> = Vec::new();
    monkey_numbers
        .par_iter()
        .map(|secret_number| {
            let mut current_number = *secret_number;
            for _ in 0..2000 {
                current_number = next_sectret(current_number);
            }
            current_number
        })
        .collect_into_vec(&mut new_numbers);

    //println!("{:?}", new_numbers);

    let sum = new_numbers.iter().fold(0, |acc, e| acc + e);

    Ok(sum)
}

fn next_sectret(current_number: i64) -> i64 {
    let first_result = prune(mix(current_number * 64, current_number));
    let second_result = prune(mix(first_result / 32, first_result));
    let third_result = prune(mix(second_result * 2048, second_result));
    third_result
}

fn mix(given_value: i64, secret_number: i64) -> i64 {
    given_value ^ secret_number
}

fn prune(secret_number: i64) -> i64 {
    secret_number % 16777216
}
