use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, print_grid, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day25.txt";


fn main() -> io::Result<()> {

    let mut max = 0;

    let groups: Vec<Vec<String>> = group_text(PATH);
    let mut locks : HashSet<[usize; 5]> = HashSet::new();
    let mut keys : HashSet<[usize; 5]> = HashSet::new();

    for g in groups {
        let is_lock = g[0].chars().collect::<Vec<char>>()[0] == '#';
        let mut pattern: [usize; 5] = [0; 5];
        for i in 1..g.len() - 1 {
           let row: Vec<char> = g[i].chars().collect::<Vec<char>>();
            for (j,c) in row.iter().enumerate() {
                if *c == '#' {pattern[j] = pattern[j] + 1;}
            }
        }
        if is_lock {locks.insert(pattern);} else {
            keys.insert(pattern);
        }

    }
    println!("{:?}", locks);
    println!("{:?}", keys);

    part_1::run(&locks, &keys);
    //part_2::run(&mut wires, &calcs, wire_names.len());




    Ok(())
}