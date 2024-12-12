use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use utils::Position;

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day11.txt";

fn main() -> io::Result<()> {


    let binding = read_to_string(PATH).unwrap();
    let line: &str = binding.lines().collect::<Vec<_>>()[0];
    let numbers: Vec<u64> = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();


    println!("{:?}", numbers);

    part_1::run(&numbers);

    part_2::run(&numbers);

    Ok(())
}