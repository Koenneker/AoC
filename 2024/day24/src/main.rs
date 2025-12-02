use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::{self, Read};
use strum_macros::EnumString;

fn main() -> Result<(), io::Error> {
    /*
    let count = day24_1("src/example.txt")?;
    println!("day24_1 Example: {}", count);
    println!("");
    let count = day24_1("src/example_2.txt")?;
    println!("day24_1 Example: {}", count);
    println!("");
    let count = day24_1("src/input.txt")?;
    println!("day24_1 input : {}", count);
    println!("");
     */

    let count = day24_2("src/input.txt")?;
    println!("day24_2 input : {}", count);
    println!("");
    /*
     */

    Ok(())
}

fn day24_2(filename: &str) -> Result<i64, io::Error> {
    let mut wires: HashMap<[char; 3], bool> = HashMap::new();
    let mut operations: HashMap<[char; 3], ([char; 3], Operation, [char; 3])> = HashMap::new();

    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let re = Regex::new(r"(\w{3}) (AND|XOR|OR) (\w{3}) -> (\w{3})").unwrap();
    for capture in re.captures_iter(&input) {
        let operand_1: [char; 3] = capture[1]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        let operation: Operation = capture[2].try_into().unwrap();

        let operand_2: [char; 3] = capture[3]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        let result: [char; 3] = capture[4]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        //println!("{:?} {:?} {:?} -> {:?}",operand_1, operation, operand_2, result);

        operations.insert(result, (operand_1, operation, operand_2));
    }

    for i in 0..45 {
        let x_wire = ['x', (b'0' + (i / 10)) as char, (b'0' + (i % 10)) as char];
        let x_value = ((x >> i) & 1) == 1;
        wires.insert(x_wire, x_value);

        let y_wire = ['y', (b'0' + (i / 10)) as char, (b'0' + (i % 10)) as char];
        let y_value = ((y >> i) & 1) == 1;
        wires.insert(y_wire, y_value);
    }
    let mut z_index: u8 = 0;
    let mut result: i64 = 0;
    loop {
        let z = [
            'z',
            (b'0' + (z_index / 10)) as char,
            (b'0' + (z_index % 10)) as char,
        ];
        //println!("{:?}", z);
        if !wires.contains_key(&z) && !operations.contains_key(&z) {
            break;
        }

        if evaluate_identifier(&z, &mut wires, &operations) {
            result += (2 as i64).pow(z_index as u32);
        }

        z_index += 1;
    }

    Ok(result.into())
}

fn day24_1(filename: &str) -> Result<i64, io::Error> {
    let mut wires: HashMap<[char; 3], bool> = HashMap::new();
    let mut operations: HashMap<[char; 3], ([char; 3], Operation, [char; 3])> = HashMap::new();

    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let re = Regex::new(r"(\w{3})\:\ (\d)").unwrap();
    for capture in re.captures_iter(&input) {
        let identifier: [char; 3] = capture[1]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        let bool_value = match capture[2].chars().collect::<Vec<char>>()[0] {
            '1' => true,
            '0' => false,
            _ => panic!("Invalid boolean value"),
        };

        wires.insert(identifier, bool_value);
    }

    let re = Regex::new(r"(\w{3}) (AND|XOR|OR) (\w{3}) -> (\w{3})").unwrap();
    for capture in re.captures_iter(&input) {
        let operand_1: [char; 3] = capture[1]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        let operation: Operation = capture[2].try_into().unwrap();

        let operand_2: [char; 3] = capture[3]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        let result: [char; 3] = capture[4]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        //println!("{:?} {:?} {:?} -> {:?}",operand_1, operation, operand_2, result);

        operations.insert(result, (operand_1, operation, operand_2));
    }

    let mut z_index: u8 = 0;
    let mut result: i64 = 0;
    loop {
        let z = [
            'z',
            (b'0' + (z_index / 10)) as char,
            (b'0' + (z_index % 10)) as char,
        ];
        //println!("{:?}", z);
        if !wires.contains_key(&z) && !operations.contains_key(&z) {
            break;
        }

        if evaluate_identifier(&z, &mut wires, &operations) {
            result += (2 as i64).pow(z_index as u32);
        }

        z_index += 1;
    }

    Ok(result.into())
}

fn evaluate_identifier(
    identifier: &[char; 3],
    wires: &mut HashMap<[char; 3], bool>,
    operations: &HashMap<[char; 3], ([char; 3], Operation, [char; 3])>,
) -> bool {
    if wires.contains_key(identifier) {
        return *(wires.get(identifier).unwrap());
    }
    if operations.contains_key(identifier) {
        if let Some(operation) = operations.get_key_value(identifier) {
            let operand_1 = evaluate_identifier(&operation.1 .0, wires, operations);
            let operand_2 = evaluate_identifier(&operation.1 .2, wires, operations);

            return match operation.1 .1 {
                Operation::AND => operand_1 & operand_2,
                Operation::OR => operand_1 | operand_2,
                Operation::XOR => operand_1 ^ operand_2,
            };
        }
    }
    panic!("Unknown Identifier: {:?}", identifier);
}

#[derive(Debug, EnumString)]
enum Operation {
    #[strum(serialize = "AND")]
    AND,
    #[strum(serialize = "OR")]
    OR,
    #[strum(serialize = "XOR")]
    XOR,
}
