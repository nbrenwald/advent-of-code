use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use regex::Regex;
use utils::{group_text, GridPosition, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day13.txt";

//will use grid position to Represent start, current and offsets and target
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Equation {
    pub button_a: GridPosition,
    pub button_b: GridPosition,
    pub target: GridPosition
}

fn main() -> io::Result<()> {
    let text: Vec<Vec<String>> = group_text(PATH);

    let mut equations: Vec<Equation> = Vec::new();

    for section in text.iter() {
        let button_a = &section[0];
        println!("{:?}", button_a);
        let re = Regex::new(r".*X\+(.*), Y\+(.*)").unwrap();
        let caps = re.captures(button_a).unwrap();
        println!("Captured {} {}", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());

        let button_a = GridPosition {row:caps.get(2).unwrap().as_str().parse::<usize>().unwrap(), col:caps.get(1).unwrap().as_str().parse::<usize>().unwrap()};

        let button_b = &section[1];
        println!("{:?}", button_b);
        let caps = re.captures(button_b).unwrap();
        println!("Captured {} {}", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());

        let button_b = GridPosition {row:caps.get(2).unwrap().as_str().parse::<usize>().unwrap(), col:caps.get(1).unwrap().as_str().parse::<usize>().unwrap()};


        let target = &section[2];
        println!("{:?}", target);
        let re = Regex::new(r".*X\=(.*), Y\=(.*)").unwrap();
        let caps = re.captures(target).unwrap();
        println!("Captured {} {}", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
        let target = GridPosition {row:caps.get(2).unwrap().as_str().parse::<usize>().unwrap(), col:caps.get(1).unwrap().as_str().parse::<usize>().unwrap()};

        let eq = Equation{button_a, button_b, target};
        equations.push(eq);
        println!("{:?}", eq);
    }


    //part_1::run(&equations);
    part_2::run(&equations);

    Ok(())
}