use std::fs::File;
use std::i32;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let count = day04_1("src/test.txt")?;
    println!("Day04_1 Test: {}", count);
    let count = day04_1("src/input.txt")?;
    println!("Day03_1 Input: {}", count);
    println!("");

    let count = day04_2("src/test.txt")?;
    println!("Day04_2 Test: {}", count);
    let count = day04_2("src/input.txt")?;
    println!("Day04_2 Input: {}", count);
    println!("");

    Ok(())
}

fn day04_1(filename: &str) -> Result<i32, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut string_vec: Vec<String> = Vec::new();

    let mut count = 0;

    for line in reader.lines() {
        string_vec.push(line.unwrap());
    }

    //Regular sideways
    for string in &string_vec {
        count += find(&string).unwrap()
    }

    //Topdown
    let transpose_vec = transpose(string_vec.clone());
    for string in transpose_vec.clone() {
        count += find(&string).unwrap()
    }

    //Diagonal \
    let diagonal_string_vec = shift_nth_row_by_n(string_vec.clone());
    let diagonal_string_vec_t = transpose(diagonal_string_vec);
    for string in razor(diagonal_string_vec_t.clone()) {
        count += find(&string).unwrap()
    }

    //Diagonal /
    let r_t_vec = reverse(transpose_vec);
    let diagonal_string_t_vec = shift_nth_row_by_n(r_t_vec.clone());
    let diagonal_string_t_vec_t = transpose(diagonal_string_t_vec);
    for string in razor(diagonal_string_t_vec_t.clone()) {
        count += find(&string).unwrap()
    }

    Ok(count)
}

fn razor(string_vector: Vec<String>) -> Vec<String> {
    let mut trimmed_vec: Vec<String> = Vec::new();

    for (index, string) in string_vector.iter().enumerate() {
        let width = string.len();
        let (first, second) = string.split_at(width - index);
        trimmed_vec.push(first.to_string());
        trimmed_vec.push(second.to_string());
    }

    return trimmed_vec;
}

fn transpose(string_vector: Vec<String>) -> Vec<String> {
    let width = string_vector[0].len();
    let mut result = vec![String::new(); width];

    for string in string_vector {
        for (i, c) in string.chars().enumerate() {
            result[i].push(c);
        }
    }

    result
}

fn reverse(string_vector: Vec<String>) -> Vec<String> {
    let mut result = string_vector.clone();

    result
        .iter_mut()
        .for_each(|s| *s = s.chars().rev().collect());

    return result;
}

fn shift_nth_row_by_n(string_vector: Vec<String>) -> Vec<String> {
    let mut shifted_vec: Vec<String> = Vec::new();
    for (index, string) in string_vector.iter().enumerate() {
        let (right, left) = string.split_at(index);
        shifted_vec.push(format!("{}{}", left, right));
    }
    return shifted_vec;
}

fn find(inputstring: &str) -> Result<i32, io::Error> {
    let mut count = inputstring.matches("XMAS").count();
    count += inputstring.matches("SAMX").count();
    Ok(count.try_into().unwrap())
}

fn rotate90(string_vector: Vec<String>) -> Vec<String> {
    return reverse(transpose(string_vector));
}

fn find_x_mas(string_vector: Vec<String>) -> Result<i32, io::Error> {
    let mut count = 0;
    for (vec_index, string) in string_vector.iter().enumerate() {
        if vec_index + 2 < string_vector.len() {
            for (str_index, char) in string.chars().enumerate() {
                if str_index + 2 < string.len() {
                    if char == 'M'
                        && string.chars().nth(str_index + 2).unwrap() == 'M'
                        && string_vector[vec_index + 1]
                            .chars()
                            .nth(str_index + 1)
                            .unwrap()
                            == 'A'
                        && string_vector[vec_index + 2].chars().nth(str_index).unwrap() == 'S'
                        && string_vector[vec_index + 2]
                            .chars()
                            .nth(str_index + 2)
                            .unwrap()
                            == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count.try_into().unwrap())
}

fn day04_2(filename: &str) -> Result<i32, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut string_vector: Vec<String> = Vec::new();

    let mut count = 0;

    for line in reader.lines() {
        string_vector.push(line.unwrap());
    }

    count += find_x_mas(string_vector.clone()).unwrap();
    count += find_x_mas(rotate90(string_vector.clone())).unwrap();
    count += find_x_mas(rotate90(rotate90(string_vector.clone()))).unwrap();
    count += find_x_mas(rotate90(rotate90(rotate90(string_vector)))).unwrap();

    return Ok(count);
}
