use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, print_grid, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day23.txt";
const SIZE: usize = 71;


fn main() -> io::Result<()> {

    let mut max = 0;

    let groups: Vec<Vec<String>> = group_text(PATH);
    //let mut codes: Vec<u64> = groups[0].iter().map(|x| x.parse::<u64>().unwrap()).collect();

    let mut nodes :HashSet<String> = HashSet::new();
    let mut graph : HashMap<String, Vec<String>> = HashMap::new();
    for edge in &groups[0] {
        let vertices:Vec<&str> = edge.split('-').collect();
        nodes.insert(vertices[0].to_string());
        nodes.insert(vertices[1].to_string());

        graph.entry(vertices[0].to_string()).or_insert(Vec::new()).push(vertices[1].to_string());
        graph.entry(vertices[1].to_string()).or_insert(Vec::new()).push(vertices[0].to_string());

    }
    println!("{:?}", graph);
    println!("{:?}", nodes);



    let trios: HashSet<Vec<String>> = part_1::run(&graph, &nodes);
    part_2::run(&graph, &nodes, &trios);

    println!("{:?}", nodes.len());



    Ok(())
}