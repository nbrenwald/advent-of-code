use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, print_grid, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day18.txt";
const SIZE: usize = 71;


fn main() -> io::Result<()> {

    let groups: Vec<Vec<String>> = group_text(PATH);


    let re = Regex::new(r"(.*),(.*)").unwrap();


    let mut positions: Vec<GridPosition> = Vec::new();
    for mem in &groups[0] {
        let col: usize = re.captures(mem.as_str()).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let row: usize = re.captures(mem.as_str()).unwrap().get(2).unwrap().as_str().parse().unwrap();
        positions.push(GridPosition { row:row, col:col });
        if positions.len() == 2877 {break;}
    }

    let mut map:Vec<Vec<char>> = Vec::new();
    for i in 0..SIZE {
        let mut row: Vec<char> = Vec::new();
        for j in 0..SIZE {
            row.push('.');
        }
        map.push(row);
    }

    for p in &positions {
        map[p.row][p.col] = '#';
    }
     print_grid(&map);



    println!("Program : {:?}", positions);

    println!("Program : {:?}", positions.len());




    //part_1::run(&map, GridPosition{row:0,col:0}, GridPosition{row:SIZE-1, col:SIZE-1});
    //part_2::run(&mut map, start, end);



    Ok(())
}