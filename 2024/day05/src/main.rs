use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::i32;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let count = day05_1("src/example.txt")?;
    println!("Day05_1 Test: {}", count);
    let count = day05_1("src/input.txt")?;
    println!("Day05_1 Input: {}", count);
    //println!("");

    let count = day05_2("src/example.txt")?;
    println!("Day05_2 Test: {}", count);
    let count = day05_2("src/input.txt")?;
    println!("Day05_2 Input: {}", count);
    println!("");

    Ok(())
}

fn day05_1(filename: &str) -> Result<i32, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\d+)\|(\d+)").unwrap();

    let mut rules: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut done_parsing_rules: bool = false;

    for line in reader.lines() {
        if let Ok(line_string) = line {
            if !done_parsing_rules {
                let mut has_cap = false;
                for cap in re.captures_iter(line_string.as_str()) {
                    if cap.len() >= 3 {
                        if let (Ok(n1), Ok(n2)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                            if let Some(set) = rules.get_mut(&n1) {
                                set.insert(n2);
                            } else {
                                let mut new_set: BTreeSet<i32> = BTreeSet::new();
                                new_set.insert(n2);
                                rules.insert(n1, new_set);
                            }
                        }
                        has_cap = true;
                    }
                }
                if !has_cap {
                    done_parsing_rules = true;
                }
            } else {
                let update: Vec<i32> = line_string
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                updates.push(update)
            }
        }
    }

    let mut result_sum = 0;

    for update in updates {
        let mut previous_pages: BTreeSet<i32> = BTreeSet::new();
        let mut alive = true;
        let mid_index = update.len() / 2;
        let mid_value = update[mid_index];
        for page in update {
            if let Some(page_rules) = rules.get(&page) {
                for rule in page_rules {
                    if previous_pages.contains(rule) {
                        alive = false;
                    }
                }
            }
            previous_pages.insert(page);
        }
        if alive {
            result_sum += mid_value;
        }
    }

    Ok(result_sum)
}

fn day05_2(filename: &str) -> Result<i32, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\d+)\|(\d+)").unwrap();

    let mut rules: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut done_parsing_rules: bool = false;

    for line in reader.lines() {
        if let Ok(line_string) = line {
            if !done_parsing_rules {
                let mut has_cap = false;
                for cap in re.captures_iter(line_string.as_str()) {
                    if cap.len() >= 3 {
                        if let (Ok(n1), Ok(n2)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                            if let Some(set) = rules.get_mut(&n1) {
                                set.insert(n2);
                            } else {
                                let mut new_set: BTreeSet<i32> = BTreeSet::new();
                                new_set.insert(n2);
                                rules.insert(n1, new_set);
                            }
                        }
                        has_cap = true;
                    }
                }
                if !has_cap {
                    done_parsing_rules = true;
                }
            } else {
                let update: Vec<i32> = line_string
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                updates.push(update)
            }
        }
    }

    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        let mut previous_pages: BTreeSet<i32> = BTreeSet::new();
        let mut is_invalid = false;

        for page in &update {
            if let Some(page_rules) = rules.get(page) {
                for rule in page_rules {
                    if previous_pages.contains(rule) {
                        is_invalid = true;
                        break;
                    }
                }
            }
            if is_invalid {
                break;
            }
            previous_pages.insert(*page);
        }

        if is_invalid {
            invalid_updates.push(update);
        }
    }

    let mut result_sum = 0;

    for update in invalid_updates {
        let mid_index = update.len() / 2;
        result_sum += update[mid_index];
    }

    Ok(result_sum)
}
