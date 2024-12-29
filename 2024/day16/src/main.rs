use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day16.txt";
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Robot {
    pos: GridPosition,
    rowVelocity: i32,
    colVelocity: i32
}


fn main() -> io::Result<()> {

    let groups: Vec<Vec<String>> = group_text(PATH);


    let mut map : Vec<Vec<char>> = Vec::new();
    let mut start: GridPositionAndDirection = GridPositionAndDirection{pos:GridPosition{row: 0, col: 0}, dir:direction::EAST};
    let mut end: GridPosition = GridPosition{row: 0, col: 0};
    for (i, row) in groups[0].iter().enumerate(){
       if row.contains('S') {
           let j = row.find('S').unwrap();
           start.pos = GridPosition{row:i, col:j };
       }
        if row.contains('E') {
            let j = row.find('E').unwrap();
            end = GridPosition{row:i, col:j };
        }
       map.push(row.chars().collect());
    }


    println!("{:?}", map);
    println!("{:?}", start);
    println!("{:?}", end);



    //part_1::run(&mut map, start, end);
    part_2::run(&mut map, start, end);



    Ok(())
}