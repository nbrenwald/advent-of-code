use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use utils::Position;

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day12-sample.txt";

fn main() -> io::Result<()> {


    let mut map : Vec<Vec<char>> = Vec::new();
    let binding = read_to_string(PATH).unwrap();
    for line in binding.lines() {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    println!("{:?}", map);

    //part_1::run(&numbers);

    //part_2::run(&numbers);

    Ok(())
}