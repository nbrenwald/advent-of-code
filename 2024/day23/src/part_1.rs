use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(graph: &HashMap<String, Vec<String>>, nodes:&HashSet<String> ) -> HashSet<Vec<String>>{
    let mut result = 0;
    let mut trios: HashSet<Vec<String>> = HashSet::new();

    for n in nodes.iter() {
        println!("checking {}", n);
        let nexts = graph.get(n).unwrap();
        for next in nexts {
            println!("checking {}", next);
            let next_neighbours = graph.get(next).unwrap();
            for next_neighbour in next_neighbours {
                if nexts.contains(next_neighbour) {
                    println!("Found a trio checking {} {} {}", n, next, next_neighbour);
                    let mut v: Vec<String> = vec![n.to_string(), next.to_string(), next_neighbour.to_string()];
                    let mut contains_t = false;
                    for r in &v {
                        if r.starts_with("t") {
                            contains_t = true;
                        }
                    }
                    if true {
                        v.sort();
                        let trio = v.join(",");
                        println!("Found a trio checking {}", trio);
                        trios.insert(v);
                    }
                }
            }

        }
    }

    println!("result {}:", result);
    println!("len {}:", trios.len());
    //1149: wrong
    trios
}
