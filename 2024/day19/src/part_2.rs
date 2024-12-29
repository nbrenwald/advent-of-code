use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

use utils::{direction, GridPosition, GridPositionAndDirection, Position};


pub(crate) fn run(towels: Vec<&str>, designs: &Vec<String>) {
    let mut cache: HashMap<String, i64> = HashMap::new();
    let mut result = 0;
    for design in designs {
        println!("Check if design {} is feasible", design);
        //println!("{}", is_possible(design, &towels));
        result +=  is_possible(design, &towels, &mut cache);

    }


    print!("result: {}", result);
}

fn is_possible(design: &String, towels: &Vec<&str>, cache: &mut HashMap<String, i64>) -> i64{

    if design == "" {return 1;}

    let mut total:i64 = 0;
    for towel in towels {

        if design.starts_with(towel) {
            let remainder = design[towel.len()..].to_string();
            if cache.contains_key(&remainder) {
                total += cache.get(&remainder).unwrap();
            }
            else {
                let count = is_possible(&remainder, towels, cache);
                cache.insert(remainder.clone(), count);
                total += count;
            }
        }
    }
    total
}

