use std::fs::File;
use std::i32;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let test_result = day01_1("src/test.txt")?;
    println!("Day01_1 Test:  {}", test_result);
    let input_result = day01_1("src/input.txt")?;
    println!("Day01_1 Input: {}", input_result);
    println!("");

    let test_result_02 = day01_2("src/test.txt")?;
    println!("Day01_2 Test:  {}", test_result_02);
    let input_result_02 = day01_2("src/input.txt")?;
    println!("Day01_2 Input: {}", input_result_02);
    Ok(())
}

fn day01_1(filename: &str) -> Result<i64, io::Error> {
    let mut number_vec_vec: Vec<Vec<i32>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let string_numbers: Vec<&str> = line.split_whitespace().collect();

        let mut numbers: Vec<i32> = Vec::new();
        for string_number in string_numbers {
            if let Ok(number) = string_number.parse::<i32>() {
                numbers.push(number);
            }
        }

        number_vec_vec.push(numbers)
    }

    let mut score = 0;

    for numbervec in number_vec_vec {
        if numbervec[0] == numbervec[1] {
            continue;
        }
        let is_ascending = numbervec[1] > numbervec[0];
        let mut is_valid = true;
        let mut last_number: &i32 = if is_ascending { &i32::MIN } else { &i32::MAX };

        for (i, number) in numbervec.iter().enumerate() {
            if i == 0 {
                last_number = number;
                continue;
            }
            if is_ascending {
                if number > last_number && (number - last_number) <= 3 {
                    last_number = number;
                    continue;
                } else {
                    is_valid = false;
                    break;
                }
            } else {
                if number < last_number && (last_number - number) <= 3 {
                    last_number = number;
                    continue;
                } else {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            score += 1
        }
    }

    Ok(score)
}

fn day01_2(filename: &str) -> Result<i64, io::Error> {
    let mut number_vec_vec: Vec<Vec<i32>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let string_numbers: Vec<&str> = line.split_whitespace().collect();

        let mut numbers: Vec<i32> = Vec::new();
        for string_number in string_numbers {
            if let Ok(number) = string_number.parse::<i32>() {
                numbers.push(number);
            }
        }

        number_vec_vec.push(numbers)
    }

    let mut score = 0;

    for mut numbervec in number_vec_vec {
        let errors = analyze(&numbervec);
        if errors <= 1 {
            score += 1
        } else {
            numbervec.reverse();
            let errors_flip = analyze(&numbervec);
            if errors_flip <= 1 {
                score += 1
            }
        }
    }

    Ok(score)
}

fn analyze(numbervec: &Vec<i32>) -> i32 {
    if numbervec[0] == numbervec[1] {
        return 2;
    }
    let is_ascending = numbervec[1] > numbervec[0];
    let mut errors = 0;
    let mut last_number: &i32 = if is_ascending { &i32::MIN } else { &i32::MAX };

    for (i, number) in numbervec.iter().enumerate() {
        if i == 0 {
            last_number = number;
            continue;
        }
        if is_ascending {
            if number > last_number && (number - last_number) <= 3 {
                last_number = number;
                continue;
            } else if errors == 0 {
                errors += 1;
                continue;
            } else {
                errors += 1;
                break;
            }
        } else {
            if number < last_number && (last_number - number) <= 3 {
                last_number = number;
                continue;
            } else if errors == 0 {
                errors += 1;
                continue;
            } else {
                errors += 1;
                break;
            }
        }
    }
    return errors;
}
