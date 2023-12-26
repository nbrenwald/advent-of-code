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
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize)
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

    let mut result = 0;
        result += get_score(&graph);


    println!("total = {}", result);
    Ok(())
}

fn apply(r:&Condition, p:&Part) -> (Part, Part, String) {
    let mut new_part = p.clone();
    let mut unmatch = p.clone();
               match (r.field, r.op) {
               ('x', '<') => {
                   new_part.x.1 = r.value -1;
                   unmatch.x.0 = r.value;
               },
               ('x', '>') => {
                   new_part.x.0 = r.value+1;
                   unmatch.x.1 = r.value;
               },
               ('s', '<') => {
                   new_part.s.1 = r.value -1;
                   unmatch.s.0 = r.value;
               },
               ('s', '>') => {
                   new_part.s.0 = r.value +1;
                   unmatch.s.1 = r.value;
               },
               ('m', '<') => {
                   new_part.m.1 = r.value -1;
                   unmatch.m.0 = r.value;
               },
               ('m', '>') => {
                   new_part.m.0 = r.value+1;
                   unmatch.m.1 = r.value;
               },
               ('a', '<') => {
                   new_part.a.1 = r.value -1;
                   unmatch.a.0 = r.value;
               }
               ('a', '>') => {
                   new_part.a.0 = r.value+1;
                   unmatch.a.1 = r.value;
               },
               _ => {}
           };
    (new_part, unmatch, r.next.clone())        

}

fn get_next(graph:&HashMap<String,(Vec<Condition>, String)>, current:&String, p: Part) -> Vec<(Part, String)> { 
    let mut result : Vec<(Part, String)> = Vec::new();

    let (v, d) =  graph.get(&current.to_string()).unwrap();
    let mut my_p : Part = p;
    for r in v.iter() {
        let new_set = apply(&r, &my_p);
        my_p = new_set.1;
        result.push((new_set.0, new_set.2));
    }
    result.push((my_p, d.to_string()));


    result
}
fn get_score(graph:&HashMap<String,(Vec<Condition>, String)>) -> i64 {


    let mut current = "in".to_string();
    let mut visit:Vec<(Part, String)> = Vec::new();
    visit.push( (Part{x:(1,4000), s:(1,4000), m:(1,4000), a:(1,4000)}, current.clone()) );


    let mut score:Vec<Part> = Vec::new();
    while !visit.is_empty() {
        let (p, s) = visit.pop().unwrap();
        println!("visit part={:?}, workflow={}", p,s );
        let v = get_next(graph, &s, p);
        for x in v.iter() {
            if x.1 == "A" {
                score.push(x.0.clone());
            }
            else if x.1 != "R"{
              visit.push((x.0.clone(), x.1.clone()));
        }}
    }


    let mut total:i64  = 0;
    for s in score.iter() {
       println!("{:?}", s);
       let x =  s.x.1 - s.x.0 + 1;
       let m =  s.m.1 - s.m.0 + 1;
       let a =  s.a.1 - s.a.0 + 1;
       let s =  s.s.1 - s.s.0  + 1;
       let x:i64 = i64::try_from(x).unwrap();
       let m:i64 = i64::try_from(m).unwrap();
       let a:i64 = i64::try_from(a).unwrap();
       let s:i64 = i64::try_from(s).unwrap();

       total += x*m*a*s;
    }

    /*while ! (current == "R" || current =="A" )
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
    }*/
    total
    
}

