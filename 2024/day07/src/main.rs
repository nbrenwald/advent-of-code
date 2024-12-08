use std::fs::{read_to_string, File};
use std::io;
mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day07.txt";

fn main() -> io::Result<()> {
    //part_1::parse(PATH);
    part_2::parse(PATH);

    Ok(())
}


