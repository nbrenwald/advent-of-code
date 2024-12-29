use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(locks: &HashSet<[usize; 5]>, keys: &HashSet<[usize; 5]>) {
    let mut result = 0;

    for key in keys.iter() {
        for lock in locks.iter() {
            let mut fits =true;
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    fits = false
                }
            }
            if fits {
                result+=1;
            }
        }
    }
println!("Result: {}", result);
}


