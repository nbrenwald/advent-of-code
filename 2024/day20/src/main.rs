use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, print_grid, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day20.txt";
const SIZE: usize = 71;


fn main() -> io::Result<()> {

    let mut max = 0;
    let mut start: GridPosition = GridPosition{row: 0, col: 0};
    let mut end: GridPosition = GridPosition{row: 0, col: 0};

    let groups: Vec<Vec<String>> = group_text(PATH);
    let mut map: Vec<Vec<char>> = vec![vec!['#'; groups[0].len()]; groups[0][0].len()];
    for (i,row) in groups[0].iter().enumerate(){
        for (j,c) in row.chars().enumerate() {
            map[i][j] = c;
            if c == '.' {
                max += 1;
            }
            if c == 'S' {
                start = GridPosition{row: i, col: j};
            }
            if c == 'E' {
                end = GridPosition{row: i, col: j};
                max += 1 ;
            }
        }
    }

    println!("{:?}", map);
    println!("Start = {:?}", start);
    println!("End = {:?}", end);
    println!("Max = {:?}", max);






    //part_1::run(&mut map, &start, &end, max);
    part_2::run(&mut map, &start, &end, max);



    Ok(())
}