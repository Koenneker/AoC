use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::{self, BufRead, BufReader, Read};

fn main() -> Result<(), io::Error> {
    let count = day21_1("src/example.txt")?;
    println!("day21_1 Example: {}", count);
    println!("");
    /*
    let count = day21_1("src/input.txt")?;
    println!("day21_1 input : {}", count);
    println!("");
     */

    /*
    let count = day21_2("src/example_2.txt")?;
    println!("day21_2 Small Example: {}", count);
    println!("");
    let count = day21_2("src/input.txt")?;
    println!("day21_2 input : {}", count);
    println!("");
     */

    Ok(())
}

fn day21_1(filename: &str) -> Result<i64, io::Error> {
    let mut char_vec_vec: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        char_vec_vec.push(line.as_str().chars().collect_vec());
    }

    println!("{:?}", char_vec_vec);

    Ok(0)
}

/*
+-0-+-1-+-2-+
0 7 | 8 | 9 |
+---+---+---+
1 4 | 5 | 6 |
+---+---+---+
2 1 | 2 | 3 |
+---+---+---+
3   | 0 | A |
    +---+---+ */
fn code_to_keypad_presses(code: Vec<char>) -> Vec<Vec<char>> {
    let mut pos: (i8, i8) = (3, 2);
    let presses = Vec::new();
    for char in code {
        let target = match char {
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '0' => (2, 1),
            'A' => (3, 2),
            _ => panic!("Unknown Char in sequence"),
        };

        let delta_y = pos.0 - target.0;
        let delta_x = pos.1 - target.1;

        pos = target;
    }
    presses
}

/*0 +-1-+-2-+
0   | ^ | A |
+---+---+---+
1 < | v | > |
+---+---+---+*/
fn keypad_presses_to_keypad_presses(code: Vec<char>) -> Vec<Vec<char>> {
    let mut pos: (i8, i8) = (0, 2);
    let presses = Vec::new();
    for char in code {
        let target = match char {
            '^' => (0, 1),
            'A' => (0, 2),
            '<' => (1, 0),
            'v' => (1, 1),
            '>' => (1, 2),
            _ => panic!("Unknown Char {} in sequence", char),
        };

        let delta_y = pos.0 - target.0;
        let delta_x = pos.1 - target.1;

        pos = target;
    }
    presses
}
