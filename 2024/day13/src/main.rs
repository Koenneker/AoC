use regex::Regex;
use std::io;
use std::{fs, i64};

fn main() -> Result<(), io::Error> {
    /*
     */
    let count = day13_1("src/example.txt")?;
    println!("day13_1 Example: {}", count);
    let count = day13_1("src/input.txt")?;
    println!("day13_1 input : {}", count);
    println!("");

    /*
     */
    let count = day13_2("src/example.txt")?;
    println!("day13_2 Test: {}", count);
    let count = day13_2("src/input.txt")?;
    println!("day13_2 Test: {}", count);

    Ok(())
}
#[derive(Debug, Clone, Copy)]
struct Button {
    x_value: i64,
    y_value: i64,
    price: i64,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: Button,
    b: Button,
    target: (i64, i64),
}

fn day13_1(filename: &str) -> Result<i64, io::Error> {
    let contents = fs::read_to_string(filename)?;

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X\=(\d+), Y\=(\d+)",
    )
    .unwrap();

    let mut machines: Vec<Machine> = Vec::new();

    for capture in re.captures_iter(&contents) {
        //println!("{:?}", capture);
        let machine: Machine = Machine {
            a: Button {
                x_value: *(&capture[1].parse::<i64>().unwrap()),
                y_value: *(&capture[2].parse::<i64>().unwrap()),
                price: 3,
            },
            b: Button {
                x_value: *(&capture[3].parse::<i64>().unwrap()),
                y_value: *(&capture[4].parse::<i64>().unwrap()),
                price: 1,
            },
            target: (
                *(&capture[5].parse::<i64>().unwrap()),
                *(&capture[6].parse::<i64>().unwrap()),
            ),
        };
        machines.push(machine);
    }

    let mut output = 0;
    for machine in machines {
        let price = find_minimum_price(&machine);
        output += price;
        //println!("{}", price)));
    }

    Ok(output)
}

fn day13_2(filename: &str) -> Result<i64, io::Error> {
    let contents = fs::read_to_string(filename)?;

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X\=(\d+), Y\=(\d+)",
    )
    .unwrap();

    let mut machines: Vec<Machine> = Vec::new();

    for capture in re.captures_iter(&contents) {
        //println!("{:?}", capture);
        let machine: Machine = Machine {
            a: Button {
                x_value: *(&capture[1].parse::<i64>().unwrap()),
                y_value: *(&capture[2].parse::<i64>().unwrap()),
                price: 3,
            },
            b: Button {
                x_value: *(&capture[3].parse::<i64>().unwrap()),
                y_value: *(&capture[4].parse::<i64>().unwrap()),
                price: 1,
            },
            target: (
                *(&capture[5].parse::<i64>().unwrap()) + 10000000000000,
                *(&capture[6].parse::<i64>().unwrap()) + 10000000000000,
            ),
        };
        machines.push(machine);
    }

    let mut output = 0;
    for machine in machines {
        let price = find_minimum_price_large(&machine);
        output += price;
        //println!("{}", price);
    }

    Ok(output)
}

fn find_minimum_price_large(machine: &Machine) -> i64 {
    let result = solve_linear_diophantine(
        machine.a.x_value,
        machine.b.x_value,
        machine.a.y_value,
        machine.b.y_value,
        machine.target.0,
        machine.target.1,
    );

    match result {
        Some((a_presses, b_presses)) => {
            if a_presses >= 0 && b_presses >= 0 && check_if_target(machine, a_presses, b_presses) {
                a_presses * machine.a.price + b_presses * machine.b.price
            } else {
                0
            }
        }
        None => 0,
    }
}

fn solve_linear_diophantine(
    a1: i64,
    b1: i64,
    a2: i64,
    b2: i64,
    c1: i64,
    c2: i64,
) -> Option<(i64, i64)> {
    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        return None;
    }

    let right_side = b2 * c1 - b1 * c2;
    if right_side % det != 0 {
        return None;
    }
    let x = right_side / det;

    if a1 == 0 {
        if b1 == 0 {
            return None;
        }
        let y = c1 / b1;
        if c1 % b1 != 0 {
            return None;
        }
        return Some((x, y));
    }

    let y = (c1 - a1 * x) / b1;
    if (c1 - a1 * x) % b1 != 0 {
        return None;
    }

    Some((x, y))
}

fn find_minimum_price(machine: &Machine) -> i64 {
    let target_x = machine.target.0 as i32;
    let target_y = machine.target.1 as i32;
    let target_max = target_x.max(target_y);

    let a = machine.a;
    let b = machine.b;

    let cost_a = machine.a.price as i32;
    let cost_b = machine.b.price as i32;

    let mut solving_traces: Vec<(i32, i32)> = Vec::new();

    for a_presses in 0..target_max {
        let new_target_x: i32 = target_x - (a_presses * a.x_value as i32);
        let new_target_y: i32 = target_y - (a_presses * a.y_value as i32);

        if new_target_x % b.x_value as i32 == 0 {
            let b_presses_to_reach_y = new_target_x / b.x_value as i32;
            if new_target_y - b_presses_to_reach_y * b.y_value as i32 == 0 {
                solving_traces.push((a_presses, b_presses_to_reach_y));
            }
        }
    }

    let mut cost = i32::MAX;

    for solving_trace in solving_traces {
        if solving_trace.0 * cost_a + solving_trace.1 * cost_b <= cost {
            cost = solving_trace.0 * cost_a + solving_trace.1 * cost_b;
        }
    }

    if cost == i32::MAX {
        return 0;
    }
    cost as i64
}

fn check_if_target(machine: &Machine, a_presses: i64, b_presses: i64) -> bool {
    let achieved_x = a_presses * machine.a.x_value + b_presses * machine.b.x_value;
    let achieved_y = a_presses * machine.a.y_value + b_presses * machine.b.y_value;

    if machine.target.0 == achieved_x && machine.target.1 == achieved_y {
        return true;
    } else {
        return false;
    }
}
