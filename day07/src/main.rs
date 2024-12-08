use rayon::prelude::*;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let count = day07_1("src/example.txt")?;
    println!("Day07_1 Test: {}", count);
    let count = day07_1("src/input.txt")?;
    println!("Day07_1 Input: {}", count);
    println!("");

    let count = day07_2("src/example.txt")?;
    println!("Day07_2 Test: {}", count);
    let count = day07_2("src/input.txt")?;
    println!("Day07_2 Input: {}", count);
    println!("");

    Ok(())
}

fn day07_2(filename: &str) -> Result<i64, io::Error> {
    let mut equation_set: HashSet<(i64, Vec<i64>)> = HashSet::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if let Some((target_string, remaining_line)) = line.split_once(':') {
            if let Ok(target) = target_string.parse::<i64>() {
                let string_numbers: Vec<&str> = remaining_line.split_whitespace().collect();

                let mut numbers: Vec<i64> = Vec::new();
                for string_number in string_numbers {
                    if let Ok(number) = string_number.parse::<i64>() {
                        numbers.push(number);
                    }
                }

                equation_set.insert((target, numbers));
            };
        }
    }

    Ok(equation_set
        .into_iter()
        .filter(|equation| is_possible_equation_recursion_with_concat(equation.0, &equation.1))
        .map(|(target, _)| target)
        .sum())
}

fn is_possible_equation_recursion_with_concat(current_target: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 2 {
        let n1 = numbers[0];
        let n2 = numbers[1];
        return n1 * n2 == current_target
            || n1 + n2 == current_target
            || concat(n1, n2) == current_target;
    }

    let (new_numbers, last_number) = numbers.split_at(numbers.len() - 1);
    let last_number = last_number[0];

    let new_add_target = current_target - last_number;
    if new_add_target >= 0
        && is_possible_equation_recursion_with_concat(new_add_target, new_numbers)
    {
        return true;
    }

    if last_number != 0 && current_target % last_number == 0 {
        let new_mul_target = current_target / last_number;
        if is_possible_equation_recursion_with_concat(new_mul_target, new_numbers) {
            return true;
        }
    }

    let last_digits = last_number.to_string().len() as i64;
    let mut divisor_string = "1".to_string();
    for _ in 0..last_digits {
        divisor_string += "0";
    }
    if let Ok(divisor) = divisor_string.parse::<i64>() {
        if current_target % divisor == last_number {
            let new_concat_target = current_target / divisor;
            if is_possible_equation_recursion_with_concat(new_concat_target, new_numbers) {
                return true;
            }
        }
    }

    false
}

fn concat(a: i64, b: i64) -> i64 {
    let b_digits = b.ilog10() + 1;
    (a * 10_i64.pow(b_digits)) + b
}

fn day07_1(filename: &str) -> Result<i64, io::Error> {
    let mut equation_set: BTreeSet<(i64, Vec<i64>)> = BTreeSet::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if let Some((target_string, remaining_line)) = line.split_once(':') {
            if let Ok(target) = target_string.parse::<i64>() {
                let string_numbers: Vec<&str> = remaining_line.split_whitespace().collect();

                let mut numbers: Vec<i64> = Vec::new();
                for string_number in string_numbers {
                    if let Ok(number) = string_number.parse::<i64>() {
                        numbers.push(number);
                    }
                }

                equation_set.insert((target, numbers));
            };
        }
    }

    Ok(equation_set
        .into_iter()
        .filter(|equation| is_possible_equation_recursion(equation.0, &equation.1))
        .map(|(target, _)| target)
        .sum())
}

fn is_possible_equation_recursion(current_target: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 2 {
        return numbers[0] * numbers[1] == current_target
            || numbers[0] + numbers[1] == current_target;
    }

    let (new_numbers, last_number) = numbers.split_at(numbers.len() - 1);
    let last_number = last_number[0];

    let new_add_target = current_target - last_number;
    if new_add_target >= 0 && is_possible_equation_recursion(new_add_target, new_numbers) {
        return true;
    }

    if last_number != 0 && current_target % last_number == 0 {
        let new_mul_target = current_target / last_number;
        if is_possible_equation_recursion(new_mul_target, new_numbers) {
            return true;
        }
    }

    false
}
