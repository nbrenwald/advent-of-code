use std::fs::{read_to_string, File};
use std::io;
mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day06.txt";

fn main() -> io::Result<()> {
    part_1::part1(PATH);
    part_2::part2(PATH);

    Ok(())
}


// part 2 brute force.
// test adding an obstacle (not starting position,mnot existing obstacle
// run existing program. If we detect that the same node is visited (with direction) then found a loop.