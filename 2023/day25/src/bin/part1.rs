use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use regex::Regex;


#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug, PartialEq, Clone)]
struct Segment {
    start: Point,
    end: Point
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
    let mut result = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    
    let groups = group_text("sample.txt");
    let re = Regex::new(r"(.*)~(.*)").unwrap();
    let mut id = 0;
    let mut wires:HashSet<(String, String)> = HashSet::new();
    for row in groups[0].iter() {
        let adj_list:Vec<&str> = row.split(": ").collect();
        //println!("{:?}", adj_list);

        let start:String = adj_list[0].to_string();
        let ends:Vec<&str> = adj_list[1].split(" ").collect();
        for e in ends.iter() {
            wires.insert((start.clone(), e.to_string()));
        }
    }
    //print out nodes for graphviz
    for w in wires.iter() {
        println!("{} -- {} [label=\"{}-{}\"]", w.0, w.1, w.0, w.1);
    }

    //println!("{:?}", wires);
    let mut x = 0;
    //manually expected graphviz to get the 3 wires to cut, it was obvious
    // dot -Tsvg -Kneato graph.out > out.svg
    wires.remove(&("ffj".to_string(), "lkm".to_string()));
    wires.remove(&("vgs".to_string(), "xjb".to_string()));
    wires.remove(&("ljl".to_string(), "xhg".to_string()));
        let graph = build_graph(&wires);
        let mut visited:Vec<String> = Vec::new();
        let start = "ljl".to_string();
        dfs(&graph, &mut visited, &start);

        if visited.len() != graph.keys().len() {
           println!("Combination {:?}", wires);
           println!("{:?}", visited);
           println!("{:?}", visited.len());
           println!("{:?}", graph.keys().len()- visited.len());
           println!("Disconnected");
        }
    //nprintln!("{}",x);
    Ok(())
}

fn build_graph(wires: &HashSet<(String,String)>) -> HashMap<String, Vec<String>> {
   let mut result:HashMap<String, Vec<String>> = HashMap::new();

   for wire in wires.iter() {
       result.entry(wire.0.clone()).or_insert(vec![]).push(wire.1.clone());
       result.entry(wire.1.clone()).or_insert(vec![]).push(wire.0.clone());
   }
   result
}

fn dfs(graph: &HashMap<String, Vec<String>>, visited:&mut Vec<String>, node: &String) -> bool {
    let neighbours = graph.get(node).unwrap();
    visited.push(node.to_string());
    for n in neighbours.iter(){
        if !visited.contains(n) {
            dfs(graph, visited, &n);
        }
    }
    true
}

