use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use utils::Position;

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day10.txt";

fn main() -> io::Result<()> {


    let mut map: Vec<Vec<char>> = Vec::new();
    let mut trail_heads: Vec<Position> = Vec::new();
    for (y,line) in read_to_string(PATH).unwrap().lines().enumerate() {
        let mut row:Vec<char> = Vec::new();
        for (x,ch) in line.chars().enumerate() {
            if ch == '0' {
                trail_heads.push(Position{x: x as i32, y: y as i32});
            }
            row.push(ch);
        }
        map.push(row);
    }

    println!("{:?}", map);
    println!("{:?}", trail_heads);
    //part_1::run(&map, &trail_heads);

    part_2::run(&map, &trail_heads);

    Ok(())
}