use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i64;
use std::io::{self, BufRead, BufReader, Read};

fn main() -> Result<(), io::Error> {
    /*
    let count = day23_1("src/example.txt")?;
    println!("day21_1 Example: {}", count);
    println!("");
    let count = day23_1("src/input.txt")?;
    println!("day21_1 input : {}", count);
    println!("");
     */

    let count = day23_2("src/example.txt")?;
    println!("day23_2 Small Example: {}", count);
    println!("");
    let count = day23_2("src/input.txt")?;
    println!("day23_2 input : {}", count);
    println!("");
    /*
     */

    Ok(())
}

fn day23_1(filename: &str) -> Result<i64, io::Error> {
    let mut identifiers: HashSet<[char; 2]> = HashSet::new();
    let mut connections: HashSet<([char; 2], [char; 2])> = HashSet::new();
    let mut cliques: HashSet<([char; 2], [char; 2], [char; 2])> = HashSet::new();

    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let re = Regex::new(r"(\w{2})-(\w{2})").unwrap();
    for capture in re.captures_iter(&input) {
        let first_user: [char; 2] = capture[1]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let second_user: [char; 2] = capture[2]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        identifiers.insert(first_user);
        identifiers.insert(second_user);

        connections.insert((first_user, second_user));
        connections.insert((second_user, first_user));
    }

    for &identifier_1 in &identifiers {
        for &identifier_2 in &identifiers {
            if identifier_2 < identifier_1 {
                continue;
            }
            for &identifier_3 in &identifiers {
                if identifier_3 < identifier_2 {
                    continue;
                }
                if connections.contains(&(identifier_1, identifier_2))
                    && connections.contains(&(identifier_1, identifier_3))
                    && connections.contains(&(identifier_2, identifier_3))
                {
                    cliques.insert((identifier_1, identifier_2, identifier_3));
                }
            }
        }
    }

    let mut t_cliques = 0;
    for clique in &cliques {
        if clique.0[0] == 't' || clique.1[0] == 't' || clique.2[0] == 't' {
            t_cliques += 1;
        }
    }
    Ok(t_cliques)
}

fn day23_2(filename: &str) -> Result<i64, io::Error> {
    let mut identifiers: HashSet<[char; 2]> = HashSet::new();
    let mut connections: HashSet<([char; 2], [char; 2])> = HashSet::new();
    let mut cliques: HashSet<([char; 2], [char; 2], [char; 2])> = HashSet::new();

    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let re = Regex::new(r"(\w{2})-(\w{2})").unwrap();
    for capture in re.captures_iter(&input) {
        let first_user: [char; 2] = capture[1]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let second_user: [char; 2] = capture[2]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        identifiers.insert(first_user);
        identifiers.insert(second_user);

        connections.insert((first_user, second_user));
    }

    let mut adj_list: HashMap<[char; 2], HashSet<[char; 2]>> = HashMap::new();
    for &vertex in &identifiers {
        adj_list.insert(vertex, HashSet::new());
    }
    for &(v1, v2) in &connections {
        adj_list.get_mut(&v1).unwrap().insert(v2);
        adj_list.get_mut(&v2).unwrap().insert(v1);
    }

    let mut r: HashSet<[char; 2]> = HashSet::new();
    let mut p: HashSet<[char; 2]> = identifiers.clone();
    let mut x: HashSet<[char; 2]> = HashSet::new();
    let mut max_clique: HashSet<[char; 2]> = HashSet::new();

    bron_kerbosch(&adj_list, &mut r, &mut p, &mut x, &mut max_clique);

    let mut clique_vec = max_clique.iter().collect_vec();
    clique_vec.sort();

    for &member in clique_vec {
        for character in member {
            print!("{}", character)
        }
        print!(",")
    }
    println!("");

    Ok(max_clique.len() as i64)
}

fn bron_kerbosch(
    adj_list: &HashMap<[char; 2], HashSet<[char; 2]>>,
    r: &mut HashSet<[char; 2]>,
    p: &mut HashSet<[char; 2]>,
    x: &mut HashSet<[char; 2]>,
    max_clique: &mut HashSet<[char; 2]>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            max_clique.clear();
            max_clique.extend(r.iter().cloned());
        }
        return;
    }

    let pivot = choose_pivot(adj_list, p, x);
    let vertices_to_process: Vec<_> = if let Some(pivot_vertex) = pivot {
        p.iter()
            .filter(|&&v| !adj_list[&pivot_vertex].contains(&v))
            .cloned()
            .collect()
    } else {
        p.iter().cloned().collect()
    };

    for v in vertices_to_process {
        r.insert(v);

        let mut new_p = p
            .iter()
            .filter(|&&n| adj_list[&v].contains(&n))
            .cloned()
            .collect();
        let mut new_x = x
            .iter()
            .filter(|&&n| adj_list[&v].contains(&n))
            .cloned()
            .collect();

        bron_kerbosch(adj_list, r, &mut new_p, &mut new_x, max_clique);

        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}

fn choose_pivot(
    adj_list: &HashMap<[char; 2], HashSet<[char; 2]>>,
    p: &HashSet<[char; 2]>,
    x: &HashSet<[char; 2]>,
) -> Option<[char; 2]> {
    let mut union: Vec<_> = p.union(x).copied().collect();
    union.sort_unstable();

    union.into_iter().max_by_key(|&v| {
        let connections = p
            .iter()
            .filter(|&&n| adj_list.get(&v).unwrap().contains(&n))
            .count();
        (connections, v)
    })
}
