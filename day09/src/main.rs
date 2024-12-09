use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() -> Result<(), io::Error> {
    /*
    let count = day09_1("src/12345.txt")?;
    println!("Day09_1 12345: {}", count);
    let count = day09_1("src/example.txt")?;
    println!("Day09_1 Test: {}", count);
    let count = day09_1("src/input.txt")?;
    println!("Day09_1 Input: {}", count);
    println!("");
     */

    let count = day09_2("src/example.txt")?;
    println!("Day09_2 Test: {}", count);
    let count = day09_2("src/input.txt")?;
    println!("Day09_2 Input: {}", count);
    println!("");
    /*
     */

    Ok(())
}

fn day09_1(filename: &str) -> Result<usize, io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let input: Vec<i32> = buffer
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let parsed_input = create_array_from_example(&input);

    let moved_blocks = move_blocks_forward(&parsed_input);

    let checksum = calculate_checksum(&moved_blocks);

    /*
    println!("{:?}", input);
    println!("{:?}", parsed_input);
    println!("{:?}", moved_blocks);
    println!("{}", checksum);
     */

    Ok(checksum)
}

fn day09_2(filename: &str) -> Result<usize, io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let input: Vec<i32> = buffer
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let parsed_input = create_array_from_example(&input);

    let blocks = vec_2_blocks(&parsed_input);

    let result_blocks = move_acutal_blocks_forward_if_fit(&blocks);

    let result_vec = blocks_2_vec(&result_blocks);

    let checksum = calculate_checksum(&result_vec);

    /*
    println!("{:?}", input);
    println!("{:?}", parsed_input);
    println!("{:?}", blocks);
    println!("{:?}", result_blocks);
    println!("{:?}", result_vec);
    println!("{}", checksum);
     */

    Ok(checksum)
}

fn calculate_checksum(input_vec: &Vec<i32>) -> usize {
    let mut sum = 0;
    for (i, value) in input_vec.iter().enumerate() {
        if *value < 0 {
            continue;
        } else {
            sum += i * *value as usize
        }
    }
    sum
}

fn move_acutal_blocks_forward_if_fit(input_vec: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut output_vec: Vec<(i32, i32)> = input_vec.clone();

    for (insert_value, insert_len) in input_vec.into_iter().rev() {
        if *insert_value == -1 {
            continue;
        }
        let mut new_vec: Vec<(i32, i32)> = Vec::new();
        let mut placed = false;
        for (space_value, space_len) in output_vec.into_iter() {
            if placed {
                if space_value != *insert_value {
                    new_vec.push((space_value, space_len));
                } else {
                    new_vec.push((-1, *insert_len));
                }
            } else if space_value == *insert_value {
                new_vec.push((*insert_value, *insert_len));
                placed = true;
            } else if space_value == -1 && space_len > *insert_len {
                new_vec.push((*insert_value, *insert_len));
                new_vec.push((space_value, space_len - insert_len));
                placed = true;
            } else if space_value == -1 && space_len == *insert_len {
                new_vec.push((*insert_value, *insert_len));
                placed = true;
            } else {
                new_vec.push((space_value, space_len));
            }
        }
        output_vec = new_vec;
    }

    output_vec
}

fn vec_2_blocks(input_vec: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut last_value = -2;
    let mut output_vec: Vec<(i32, i32)> = Vec::new();

    for value in input_vec {
        if *value == last_value {
            if let Some(last_output) = output_vec.last_mut() {
                last_output.1 += 1;
            } else {
                println!("Fucky wucky")
            }
        } else {
            last_value = *value;
            output_vec.push((*value, 1));
        }
    }

    output_vec
}

fn blocks_2_vec(input_blocks: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut output_vec: Vec<i32> = Vec::new();

    for (value, length) in input_blocks {
        output_vec.append(&mut vec![*value; *length as usize]);
    }

    output_vec
}

fn get_last_block_length_id(input_vec: &Vec<i32>) -> (i32, i32) {
    let mut index = input_vec.len() - 1;
    let mut len = 0;
    let id = input_vec[index];

    while input_vec[index] == id {
        len += 1;
        index -= 1;
    }

    (id, len)
}

fn move_blocks_forward(input_vec: &Vec<i32>) -> Vec<i32> {
    let mut output_vec: Vec<i32> = Vec::new();

    let mut front_index = 0;
    let mut back_index = input_vec.len() - 1;

    while front_index <= back_index {
        let value = input_vec[front_index];
        if value != -1 {
            output_vec.push(value);
            front_index += 1;
        } else if input_vec[back_index] != -1 {
            output_vec.push(input_vec[back_index]);
            back_index -= 1;
            front_index += 1;
        } else {
            back_index -= 1;
        }
    }

    output_vec
}

fn create_array_from_example(input_vec: &Vec<i32>) -> Vec<i32> {
    let mut output_vec: Vec<i32> = Vec::new();

    let mut block_size = true;
    let mut block_index = 0;
    for value in input_vec {
        if block_size {
            let append_vec = vec![block_index as i32; *value as usize];
            output_vec = [output_vec, append_vec].concat();
            block_index += 1;
            block_size = false;
        } else {
            let append_vec = vec![-1; *value as usize];
            output_vec = [output_vec, append_vec].concat();
            block_size = true;
        }
    }

    output_vec
}
