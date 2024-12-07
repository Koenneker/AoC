use regex::Regex;
use std::fs::File;
use std::i32;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let (count, sum) = day03_1("src/test.txt")?;
    println!("Day03_1 Test: {},{}", count, sum);
    let (count, sum) = day03_1("src/input.txt")?;
    println!("Day03_1 Input: {},{}", count, sum);
    println!("");

    let (count, sum) = day03_2("src/test_2.txt")?;
    println!("Day03_2 Test: {},{}", count, sum);
    let (count, sum) = day03_2("src/input.txt")?;
    println!("Day03_2 Input: {},{}", count, sum);
    println!("");

    Ok(())
}

fn day03_1(filename: &str) -> Result<(i32, i32), io::Error> {
    let mut inputstring = String::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        inputstring.push_str(&line.unwrap());
    }

    let (count, sum) = find(inputstring.as_str()).unwrap();

    Ok((count, sum))
}

fn find(inputstring: &str) -> Result<(i32, i32), io::Error> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut count: i32 = 0;
    let mut sum: i32 = 0;
    for cap in re.captures_iter(&inputstring) {
        if cap.len() >= 3 {
            if let (Ok(n1), Ok(n2)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                sum += n1 * n2;
                count += 1;
            }
        }
    }

    Ok((count, sum))
}

fn day03_2(filename: &str) -> Result<(i32, i32), io::Error> {
    let mut inputstring: String = String::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        inputstring.push_str(&line.unwrap());
    }

    while inputstring.contains("don't()") {
        let (safe_string, unsafe_string) = inputstring.split_once("don't()").unwrap();
        if unsafe_string.contains("do()") {
            let (_throw_away, safe_again_string) = unsafe_string.split_once("do()").unwrap();
            let mut safe_string = safe_string.to_owned();
            safe_string.push_str(safe_again_string);
            inputstring = safe_string
        } else {
            inputstring = safe_string.to_owned();
        }
    }

    let (count, sum) = find(inputstring.as_str()).unwrap();

    Ok((count, sum))
}
