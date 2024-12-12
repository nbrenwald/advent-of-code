use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use utils::{Position, Segment};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day09.txt";

fn main() -> io::Result<()> {


    let diskMap: Vec<char> =  read_to_string(PATH).unwrap().lines().collect::<Vec<_>>()[0].chars().collect();
    //part_1::run(&diskMap);

    // Can't use diskMap again as ownership was moved to run, then diskmap was dropped when run completed.
    part_2::run(&diskMap);

    Ok(())
}