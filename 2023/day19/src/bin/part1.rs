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
struct Part {
    x: usize,
    m: usize,
    a:usize,
    s: usize
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
    
    let mut graph: HashMap<String, (Vec<Condition>, String)> = HashMap::new();
    
    let groups = group_text("sample.txt");
    let rules  = &groups[0];
    let parts  = &groups[1];

    let re = Regex::new(r"(.*)\{(.*),(.*)\}").unwrap();
    for rule in rules.iter() {
        let caps = re.captures(&rule).unwrap();
        let workflow_id = caps.get(1).unwrap().as_str();
        let workflow_conditions = caps.get(2).unwrap().as_str();
        let default = caps.get(3).unwrap().as_str();
        
        let workflow_conditions_v: Vec<&str> = workflow_conditions.split(',').collect();
        
        let mut adj_list: Vec<Condition> = Vec::new();
        let re = Regex::new(r"(.*)([<>])(.*):(.*)").unwrap();
        for condition in workflow_conditions_v.iter(){
            let caps = re.captures(&condition).unwrap();
            let field:char = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
            let op:char = caps.get(2).unwrap().as_str().chars().nth(0).unwrap();
            let value:usize = caps.get(3).unwrap().as_str().parse().unwrap();
            let next:String = caps.get(4).unwrap().as_str().to_string();
            adj_list.push(Condition{field: field, op: op, value: value, next: next});
        }
        graph.insert(workflow_id.to_string(), (adj_list, default.to_string()));
    }
    println!("{:?}", graph);

    let mut parts_v:Vec<Part> = Vec::new();
    for part in parts.iter(){
        let re = Regex::new(r"\{x=(.*),m=(.*),a=(.*),s=(.*)\}").unwrap();
        let caps = re.captures(&part).unwrap();
        let x:usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let m:usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let a:usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let s:usize = caps.get(4).unwrap().as_str().parse().unwrap();
        parts_v.push(Part{x:x, m:m, a:a, s:s});
    }

    let mut result = 0;
    for p in parts_v.iter() {
        result += get_score(&graph, &p);
    }


    println!("total = {}", result);
    Ok(())
}

fn get_score(graph:&HashMap<String,(Vec<Condition>, String)>, p: &Part) -> usize {


    let mut current = "in";
    while ! (current == "R" || current =="A" )
    {
       let (v, d) =  graph.get(&current.to_string()).unwrap();
       println!("checking  p={:?} current={}",p, current);
       current = d;
       for r in v.iter() {
           println!("conditions{:?} default={}", v, d);
           match (r.field, r.op) {
               ('x', '<') => if p.x < r.value {current = &r.next; break;} ,
               ('x', '>') => if p.x > r.value {current = &r.next; break;} ,
               ('s', '<') => if p.s < r.value {current = &r.next; break;},
               ('s', '>') => if p.s > r.value {current = &r.next; break;} ,
               ('m', '<') => if p.m < r.value {current = &r.next; break;} ,
               ('m', '>') => if p.m > r.value {current = &r.next; break;} ,
               ('a', '<') => if p.a < r.value {current = &r.next; break;} ,
               ('a', '>') => if p.a > r.value {current = &r.next; break;} ,
               _ => {}
           };
       }


    }

    match current {
        "R" => 0,
        "A" => {
            println!("{:?}",p );
            p.x + p.m + p.s + p.a
        },
        _ => 0
    }
    
}

