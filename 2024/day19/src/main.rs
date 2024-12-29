use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, print_grid, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day19.txt";
const SIZE: usize = 71;


fn main() -> io::Result<()> {

    let groups: Vec<Vec<String>> = group_text(PATH);
    let mut towels: Vec<&str> = Vec::new();

    for towel in &groups[0][0].split(", ").collect::<Vec<&str>>() {
     towels.push(towel);
    }

    let designs = &groups[1];



    println!("{:?}", towels);
    println!("{:?}", designs);




    //part_1::run(towels, designs);
    part_2::run(towels, designs);



    Ok(())
}