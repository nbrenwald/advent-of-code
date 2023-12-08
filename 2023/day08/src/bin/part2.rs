use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use regex::Regex;

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
    let input:Vec<Vec<String>> = group_text("sample.txt");

    let directions:Vec<char> = input[0][0].chars().collect();
    let mut graph:HashMap<String, (String, String)> = HashMap::new();
    let mut startNodes:Vec<String> = Vec::new();
    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    for line in input[1].iter() {
        let caps = re.captures(&line).unwrap();
        let id = caps.get(1).unwrap().as_str().to_string();
        if id.chars().nth(2).unwrap() == 'A' {
            println!("{}", id);
            startNodes.push(id.clone());
        }
        let left = caps.get(2).unwrap().as_str().to_string();
        let right = caps.get(3).unwrap().as_str().to_string();
        println!("{} {} {} {}", line, id, left, right);

        graph.insert(id, (left, right));
        let line_str:String = line.to_string();
        let words:Vec<&str> = line_str.split(" ").collect();
    }

    let mut current_place:String = "AAA".to_string();
    let mut steps = 0;
    let mut direction =0;

    let mut result: Vec<usize> = Vec::new();
    for node in startNodes.iter(){
        let x = get_path(&node, &graph, &directions);
        result.push(x);
    }
    
    print!("{:?}, {}", result, steps);

    let mut l :i64 = result[0].try_into().unwrap();
    let mut i = 1;
    while i < result.len() {
        l = lcm(l, result[i].try_into().unwrap());
        i += 1;
    }
println!("{}", l);
    Ok(())
}

 
fn get_path(node : &String, graph: &HashMap<String, (String, String)>, directions: &Vec<char>) -> usize {
    let mut current_place:String = node.clone();
    let mut steps = 0;
    let mut direction =0;
    while current_place.chars().nth(2).unwrap() != 'Z' {
       let (left, right) = graph.get(&current_place).unwrap();
       if directions[direction] == 'L' {
           current_place = left.to_string();
       }
       else {
           current_place = right.to_string();
       }
       steps += 1;
       if direction == directions.len() -1 {
           direction = 0;
       }
       else {direction += 1;}
    }
    print!("{}", steps);
    steps

}

fn next(graph: &HashMap<String, (String, String)>, start: Vec<String>, d: char) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for node in start.iter() {
        let (left, right) = graph.get(node).unwrap();
        //println!("Checking {} for direction {} with rule ({},{})", node, d, left, right);
        if d == 'L' {
            //println!("adding {}", left);
            result.push(left.to_string());}
        else {

            //println!("adding {}", right);
            result.push(right.to_string());}
        let ten_millis = time::Duration::from_millis(1000);

    }
    result
}

fn all_end_with(v: &Vec<String>, c: char) -> bool {
    for s in v.iter(){

        if s.chars().nth(2).unwrap() != c {
            return false;
        }
    }
    true
}

fn lcm(n1:i64, n2:i64) -> i64 
{
    let mut rem:i64= 0;
    let mut lcm:i64= 0;
    let mut x:i64  = 0;
    let mut y:i64  = 0;
    
    if (n1 > n2) {
        x = n1;
        y = n2;
    }
    else {
        x = n2;
        y = n1;
    }

    rem = x % y;

    while (rem != 0) {
        x = y;
        y = rem;
        rem = x % y;
    }

    lcm = n1 * n2 / y;
    lcm
}
