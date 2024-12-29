use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{group_text, GridPosition, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day15.txt";
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Robot {
    pos: GridPosition,
    rowVelocity: i32,
    colVelocity: i32
}


fn main() -> io::Result<()> {

    let groups: Vec<Vec<String>> = group_text(PATH);


    let mut map : Vec<Vec<char>> = Vec::new();
    let mut start: GridPosition = GridPosition{row: 0, col: 0};
    for (i, row) in groups[0].iter().enumerate(){
       if row.contains('@') {
           let j = row.find('@').unwrap();
           start = GridPosition{row:i, col:j};
       }
       map.push(row.chars().collect());
    }

    let mut ins: Vec<char> = Vec::new();
    for row in &groups[1]{
        row.chars().for_each(|c| ins.push(c));

    }
    println!("{:?}", map);
    println!("{:?}", ins);
    //part_1::run(&mut map, start, &ins);
    part_2::run(&mut map, start, &ins);
    Ok(())
}