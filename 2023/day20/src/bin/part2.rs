use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use regex::Regex;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    t: String,
    n: String
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Condition {
    field: char,
    op: char,
    value:usize,
    next: String
}

fn group_text(file: &str) -> Vec<Vec<String>> {
    let mut group:Vec<String> = Vec::new();
    let mut text:Vec<Vec<String>> = Vec::new();
    for line in read_to_string(file).unwrap().lines() {
      if line == "" {
         if group.len() != 0 {
            text.push(group);
            group = Vec::new();
         }   
      }
      else {
          group.push(line.to_string());
      }
    }
    if group.len() != 0 {
        text.push(group);
    }
    text
}


fn main() -> io::Result<()> {
    let mut total = 0;
    let mut result = 0;
    let mut map: Vec<Vec<usize>> = Vec::new();
    
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut flip_flops: HashMap<String, bool> = HashMap::new();
    let mut cons: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let groups = group_text("sample.txt");
    let rules  = &groups[0];

    let re = Regex::new(r"(b|%|&)(.*) -> (.*)").unwrap();
    for rule in rules.iter() {
        let caps = re.captures(&rule).unwrap();
        let node_type = caps.get(1).unwrap().as_str();
        let mut node_id = caps.get(2).unwrap().as_str();

        if node_type == "%" {
            flip_flops.insert(node_id.to_string(), false);

        }
        else if node_type == "&" {
          let mut upstream: HashMap<String, bool> = HashMap::new();
          cons.insert(node_id.to_string(), upstream);
        }
        else if node_type == "b" {
           node_id = "broadcaster";
        }

        let links = caps.get(3).unwrap().as_str();
        let links_v: Vec<&str> = links.split(", ").collect();
        let mut adj_list: Vec<String> = Vec::new();
        for l in links_v.iter(){
            adj_list.push(l.to_string());
        }
        graph.insert(node_id.to_string(), adj_list);
    }

    for (key, value) in graph.iter() {
        for x in value.iter() {
            if cons.contains_key(x) {
               cons.get_mut(x).unwrap().insert(key.to_string(), false);

            }
        }
    }


    println!("{:?}", graph);
    println!("{:?}", flip_flops);
    println!("{:?}", cons);
    let mut x = 0;
    let mut total_low = 0;
    let mut total_high = 0;
    let mut last_dh = 0;
    let mut last_mk = 0;
    let mut last_vf = 0;
    let mut last_rn = 0;
    while x < 100000000 {
      let r = bfs(&graph, &mut flip_flops, &mut cons, x);
      total_low += r.0;
      total_high += r.1;
      //println!("cycle: {}, {:?}", x+1, cons.get("jz").unwrap());

      x+= 1;
    }
    println!("{} {} {}", total_low, total_high, total_low * total_high);

    Ok(())
}

fn bfs(graph: &HashMap<String, Vec<String>>, ff: &mut HashMap<String, bool>, cons: &mut HashMap<String, HashMap<String, bool>>, x:usize) -> (usize, usize) {

    let mut last_dh = 0;
    let mut last_mk = 0;
    let mut last_vf = 0;
    let mut last_rn = 0;
    let mut low_count = 0;
    let mut high_count = 0;
    let mut q:Vec<(String, bool)> = Vec::new();

    q.push( ("broadcaster".to_string(), false ) );

    while !q.is_empty() {
              for (k,v) in cons.get("jz").unwrap() {
          if k == "dh" && *v {
              println!("dh cycle {}", x );
          }
          if k == "mk" && *v {
              println!("mk cycle {}", x );
          }
          if k == "vf" && *v {
              println!("vf cycle {}", x );
          }
          if k == "rn" && *v {
              println!("rn cycle {}", x );
          }
         //println!("cycle: {}, {:?}", x+1, cons.get("jz").unwrap());
      }

        let signal = q.remove(0);
        //println!("{} {}", signal.0, signal.1);
        let mut send = signal.1;

        if ff.contains_key(&signal.0) {

            if !signal.1 {
                let x = ff.get(&signal.0).unwrap();
                send = !x;
                ff.insert(signal.0.clone(), send);
            }
            else {
                continue;
            }
        }
        else if cons.contains_key(&signal.0) {
          let states = cons.get(&signal.0).unwrap();
          let mut x = true;
          for (k,v) in states {
            x = x && *v;
          }
          if x {
            send = false;
          }
          else {
              send = true;
          }

        }

        if graph.contains_key(&signal.0) {
          for x in  graph.get(&signal.0).unwrap().iter() {

              if cons.contains_key(x) {
                  let mut y: &mut HashMap<String, bool> = cons.get_mut(x).unwrap();
                  y.insert(signal.0.clone(), send);
              }

              q.push( (x.to_string(),send) );

          }
        }

    }
    (low_count, high_count)
}

