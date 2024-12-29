use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

use utils::{direction, GridPosition, GridPositionAndDirection, Position};


pub(crate) fn run(towels: Vec<&str>, designs: &Vec<String>) {

    let mut result = 0;
    for design in designs {
        println!("Check if design {} is feasible", design);
        println!("{}", is_possible(design, &towels));
        if is_possible(design, &towels) {result += 1;}

    }
print!("result: {}", result);
}

fn is_possible(design: &String, towels: &Vec<&str>) -> bool{

    if design == "" {return true;}
    for towel in towels {

        if design.starts_with(towel) {
            let remainder = design[towel.len()..].to_string();
            if (is_possible(&remainder, towels)) {
                return true;
            }
        }
    }
     false
}

