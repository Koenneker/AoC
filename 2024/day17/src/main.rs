use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::intrinsics::mir::Len;
use std::io::{self, BufRead, BufReader, Read};

fn main() -> Result<(), io::Error> {
    /*
    let count = day17_1("src/example.txt")?;
    println!("day17_1 Example: {}", count);
    println!("");
    let count = day17_1("src/input.txt")?;
    println!("day17_1 input : {}", count);
    println!("");
     */

    /*
    let count = day17_2("src/example_2.txt")?;
    println!("day17_2 Small Example: {}", count);
    println!("");
     */
    let count = day17_2("src/input.txt")?;
    println!("day17_2 input : {}", count);
    println!("");

    Ok(())
}

fn day17_2(filename: &str) -> Result<usize, io::Error> {
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut registers: (i64, i64, i64) = (0, 0, 0);
    let mut operations: Vec<i64> = vec![];

    let re = Regex::new(
        r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ((?:\d,)*\d)",
    )
    .unwrap();

    let mut registers: (i64, i64, i64) = (0, 0, 0);
    let mut operations: Vec<i64> = Vec::new();
    if let Some(captures) = re.captures(&input.to_string()) {
        registers = (
            captures[1].parse::<i64>().unwrap(),
            captures[2].parse::<i64>().unwrap(),
            captures[3].parse::<i64>().unwrap(),
        );

        operations = captures[4]
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
    } else {
        panic!("Invalid input format");
    }

    println!("{:?}", registers);
    println!("{:?}", operations);

    let mut loop_count = 0;
    let mut correct_digits = 1;

    loop {
        loop_count += 1;
        registers = (loop_count, 0, 0);
        let mut output = Vec::new();
        let mut instruction_pointer = 0;

        while instruction_pointer < operations.len() {
            let opcode = operations[instruction_pointer];
            let operand = operations[instruction_pointer + 1];
            instruction_pointer = execute_instruction(
                opcode,
                operand,
                &mut registers,
                instruction_pointer,
                &mut output,
            );
        }
        //println!("A: {}, Output: {:?}", loop_count, output);

        for i in 0..correct_digits {
            if operations[operations.len() - i] == output[len-i];
        }

        if output == operations {
            println!("Done! {}", loop_count);
            break;
        }
    }

    Ok(0)
}


fn find_starting_value(sequence: &[i64], last_input: i64, currently_correct_digits:i64) -> i64 {
    let mut current = sequence;
    for &target in sequence.iter().rev().skip(1) {
        // Find a number that when divided by 3 gives our current number
        // and when mod 8 equals our target
        for n in 0..24 { // try multiples of 3 plus our target
            if (n % 8 == target) && (n / 3 == current) {
                current = n;
                break;
            }
        }
    }
    current
}

fn day17_1(filename: &str) -> Result<usize, io::Error> {
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut registers: (i64, i64, i64) = (0, 0, 0);
    let mut operations: Vec<i64> = vec![];

    let re = Regex::new(
        r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ((?:\d,)*\d)",
    )
    .unwrap();

    let mut registers: (i64, i64, i64) = (0, 0, 0);
    let mut operations: Vec<i64> = Vec::new();
    if let Some(captures) = re.captures(&input.to_string()) {
        registers = (
            captures[1].parse::<i64>().unwrap(),
            captures[2].parse::<i64>().unwrap(),
            captures[3].parse::<i64>().unwrap(),
        );

        operations = captures[4]
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
    } else {
        panic!("Invalid input format");
    }

    println!("{:?}", registers);
    println!("{:?}", operations);

    let mut output = Vec::new();
    let mut instruction_pointer = 0;
    while instruction_pointer < operations.len() {
        let opcode = operations[instruction_pointer];
        let operand = operations[instruction_pointer + 1];
        instruction_pointer = execute_instruction(
            opcode,
            operand,
            &mut registers,
            instruction_pointer,
            &mut output,
        );
    }
    println!("{:?}", output);

    Ok(0)
}

fn eval_combo(combo: i64, registers: &(i64, i64, i64)) -> i64 {
    match combo {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        _ => panic!("Unknown combo value: {}", combo),
    }
}

fn execute_instruction(
    opcode: i64,
    operand: i64,
    registers: &mut (i64, i64, i64),
    instruction_pointer: usize,
    output: &mut Vec<i64>,
) -> usize {
    match opcode {
        0 => adv(operand, registers, instruction_pointer),
        1 => bxl(operand, registers, instruction_pointer),
        2 => bst(operand, registers, instruction_pointer),
        3 => jnz(operand, registers, instruction_pointer),
        4 => bxc(operand, registers, instruction_pointer),
        5 => out(operand, registers, instruction_pointer, output),
        6 => bdv(operand, registers, instruction_pointer),
        7 => cdv(operand, registers, instruction_pointer),
        _ => panic!("Unknown opcode: {}", opcode),
    }
}

fn adv(combo: i64, registers: &mut (i64, i64, i64), instruction_pointer: usize) -> usize {
    let numerator = registers.0; //A register
    let denominator = (2 as i64).pow(eval_combo(combo, &registers) as u32);

    registers.0 = numerator / denominator;

    instruction_pointer + 2
}

fn bdv(combo: i64, registers: &mut (i64, i64, i64), instruction_pointer: usize) -> usize {
    let numerator = registers.0; //A register
    let denominator = (2 as i64).pow(eval_combo(combo, &registers) as u32);

    registers.1 = numerator / denominator;

    instruction_pointer + 2
}

fn cdv(combo: i64, registers: &mut (i64, i64, i64), instruction_pointer: usize) -> usize {
    let numerator = registers.0; //A register
    let denominator = (2 as i64).pow(eval_combo(combo, &registers) as u32);

    registers.2 = numerator / denominator;

    instruction_pointer + 2
}

fn bxl(literal: i64, registers: &mut (i64, i64, i64), instruction_pointer: usize) -> usize {
    registers.1 = registers.1 ^ literal;
    instruction_pointer + 2
}

fn bst(combo: i64, registers: &mut (i64, i64, i64), instruction_pointer: usize) -> usize {
    registers.1 = eval_combo(combo, registers) % 8;
    instruction_pointer + 2
}

fn jnz(literal: i64, registers: &mut (i64, i64, i64), instruction_pointer: usize) -> usize {
    if registers.0 == 0 {
        return instruction_pointer + 2;
    } else {
        return literal as usize;
    }
}
fn bxc(_: i64, registers: &mut (i64, i64, i64), instruction_pointer: usize) -> usize {
    registers.1 = registers.1 ^ registers.2;
    instruction_pointer + 2
}

fn out(
    combo: i64,
    registers: &mut (i64, i64, i64),
    instruction_pointer: usize,
    output: &mut Vec<i64>,
) -> usize {
    let num = eval_combo(combo, registers) % 8;
    output.push(num);
    instruction_pointer + 2
}
