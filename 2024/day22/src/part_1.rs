use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(codes: Vec<u64> ) {
    let mut result = 0;
    for code in codes {
        let mut  secret = code;
        for i in 0 .. 2000 {
            let mut a = secret *64;
            secret = a ^ secret;
            secret=  secret % 16777216;
            let mut b = secret / 32;;
            secret =  b ^ secret;
            secret = secret % 16777216;
            let mut c = secret * 2048;
            secret = c ^ secret;
            secret=  secret % 16777216;
        }
        result += secret;
        println!("new {}:", secret);
    }
    println!("result {}:", result);
}
