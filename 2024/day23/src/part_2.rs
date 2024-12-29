use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(graph: &HashMap<String, Vec<String>>, nodes:&HashSet<String>, trios: &HashSet<Vec<String>> ) {
    let mut result = 0;
    let mut result_string = String::from("");

    println!("Nodes{:?}", nodes);

    for trio in trios.iter() {
        //println!("{:?}", trio);
        let mut clique: HashSet<&String> = HashSet::from_iter(trio.iter());
        for node in nodes.iter() {

            if ! clique.contains(node) {
                //println!("checking to add {:?} into {:?}",node, clique);

                let mut connected: bool = true;
                for n in &clique {
                    if !graph.get(*n).unwrap().contains(node) {
                        //println!("Not connected");
                        connected = false;
                        break;
                    }
                }

                if connected {
                    clique.insert(node);
                }
                /*  //try add node into clique
                  if trio.contains(node) {continue;}
                  let neighbours: HashSet<&String> = HashSet::from_iter(graph.get(node).unwrap().iter());
                  if clique.is_subset(&neighbours) {
                      //println!("{:?} is subset of {:?}",clique, neighbours);
                      clique.insert(node);
                  }
                  }*/
                //println!("clique: {:?}", clique.len());
                // println!("Result of Part 2: {}", result);
                if clique.len() > result {
                    println!("Found new larger clique: {:?}", clique.len());
                    let mut r: Vec<String> = Vec::new();
                    for x in clique.iter().cloned() {
                        r.push(x.to_string());
                    }
                    r.sort();
                    result_string = r.join(",");
                }
                result = result.max(clique.len());

            }
        }
}
    println!("Result of Part 2: {}", result);
    println!("Result of Part 2: {}", result_string);
}
