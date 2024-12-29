use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(codes: Vec<u64> ) {
    let mut result = 0;
    let mut global_costs: HashMap<String, u64> = HashMap::new();
    for code in codes {
        println!("Code {}", code);
        let mut local_costs: HashMap<String, u64> = HashMap::new();
        let mut ones:Vec<u64> = Vec::new();
        let mut  secret = code;
        ones.push(secret %10);
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
            //println!("new {}:", secret);
            //println!("new {}:", secret%10);
            ones.push(secret%10);
        }


        let mut i = 1;
        let mut j = 2;
        let mut k = 3;
        let mut l = 4;
        while l < ones.len() {
            let i_diff:i32 = ones[i] as i32 - ones[i-1] as i32;
            let j_diff:i32 = ones[j] as i32 - ones[j-1] as i32;
            let k_diff:i32 = ones[k] as i32 - ones[k-1] as i32;
            let l_diff:i32 = ones[l] as i32 - ones[l-1] as i32;
            //println!("{},{},{},{} = {}", i_diff, j_diff, k_diff, l_diff, ones[l]);

            let key = format!("{},{},{},{}", i_diff, j_diff, k_diff, l_diff);
            let mut x = 0;

            if !local_costs.contains_key(&key) {
                local_costs.insert(key, ones[l]);
            }
            i += 1;
            j += 1;
            k += 1;
            l += 1;
        }
        for (k, v) in local_costs.iter() {
            let mut x = 0;
            if global_costs.contains_key(k) {
                x = global_costs[k];
            }
            global_costs.insert(k.clone(), x + v);
        }
    }

    for key in global_costs.keys() {
        println!("{},{},{}", key, global_costs[key], global_costs[key]);
        result = result.max(global_costs[key]);
    }
    println!("result {}:", result);
}
