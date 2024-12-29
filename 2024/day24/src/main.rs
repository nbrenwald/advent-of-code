use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, print_grid, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day24.txt";
const SIZE: usize = 71;


fn main() -> io::Result<()> {

    let mut max = 0;

    let groups: Vec<Vec<String>> = group_text(PATH);
    //let mut codes: Vec<u64> = groups[0].iter().map(|x| x.parse::<u64>().unwrap()).collect();

    let mut wires: BTreeMap<String, usize> = BTreeMap::new();
    let mut wire_names: HashSet<String> = HashSet::new();
    for wire in &groups[0] {
        let split:Vec<&str> = wire.split(": ").collect();
        let wire: String = split[0].to_string();
        let value:usize = split[1].parse().unwrap();
        wires.insert(wire.clone(), value);
        wire_names.insert(wire.clone());
    }

    //println!("{:?}", wires);

    let mut calcs: Vec<(String,String,String,String)> = vec![];

    for calc in &groups[1]{
        let split:Vec<&str> = calc.split(" -> ").collect();
        let out = split[1].to_string();

        let split2:Vec<&str> = split[0].split(" ").collect();
        let in1 = split2[0].to_string();
        let in2 = split2[2].to_string();
        let op = split2[1].to_string();
        wire_names.insert(in1.clone());
        wire_names.insert(in2.clone());
        wire_names.insert(out.clone());

        //println!("{} {} {} {}", in1, in2,op, out);
        calcs.push((in1.clone(),in2.clone(),op,out.clone()));

    }


    //part_1::run(&mut wires, &calcs, wire_names.len());
    part_2::run(&mut wires, &calcs, wire_names.len());




    Ok(())
}