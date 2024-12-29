use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, print_grid, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day21.txt";
const SIZE: usize = 71;


fn main() -> io::Result<()> {

    let mut max = 0;

    let groups: Vec<Vec<String>> = group_text(PATH);
    let mut codes = &groups[0];
    for code in &groups[0] {
        println!("{}", code);
    }



    //part_1::run(codes);
    part_2::run(codes);



    Ok(())
}