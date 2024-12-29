use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{group_text, GridPosition, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day14.txt";
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Robot {
    pos: GridPosition,
    rowVelocity: i32,
    colVelocity: i32
}


fn main() -> io::Result<()> {
    let binding = read_to_string(PATH).unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    for line in binding.lines() {
        //p=2,4 v=2,-3
        let re = Regex::new(r"p=(.*),(.*) v=(.*),(.*)").unwrap();
        let caps = re.captures(line).unwrap();
        println!("Captured {} {} {} {}", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str(), caps.get(4).unwrap().as_str());
        robots.push(Robot{pos: GridPosition{row:caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            col:caps.get(1).unwrap().as_str().parse::<usize>().unwrap()},
        rowVelocity:caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        colVelocity:caps.get(3).unwrap().as_str().parse::<i32>().unwrap()});
    }

    println!("Robots {:?}", robots);


    //part_1::run(&mut robots, 103,101);
    part_2::run(&mut robots, 103,101);



    Ok(())
}