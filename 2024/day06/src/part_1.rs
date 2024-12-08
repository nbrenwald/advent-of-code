use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io;
use utils::direction;


pub(crate) fn part1(path: &str) -> io::Result<()> {

    let mut grid : Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    let mut visited:HashSet<(usize,usize)> = HashSet::new();
    let mut direction= direction::NORTH;


    for (rNumber,line) in read_to_string(path).unwrap().lines().enumerate() {
        let mut row : Vec<char> = Vec::new();
        for (cNumber,c) in line.chars().enumerate() {
            if c == '^' {
                i = rNumber;
                j = cNumber;
            }
            row.push(c);
        }
        grid.push(row);
    }

    println!("{:?}",grid);
    println!("Starting at {} {}",i, j);

    while i > 0 && i < grid.len() && j>0 && j < grid[0].len() {

        let mut nextI = i;
        let mut nextJ = j;

        match direction {
            direction::NORTH => { nextI -= 1;}
            direction::SOUTH => { nextI += 1;}
            direction::EAST => {nextJ += 1;}
            direction::WEST => {nextJ -= 1;}
        }

        if nextI >= 0 && nextI < grid.len() && nextJ>=0 && nextJ < grid[0].len() && grid[nextI][nextJ] == '#' {
            match direction {
                direction::NORTH => { direction = direction::EAST; }
                direction::SOUTH => { direction = direction::WEST}
                direction::EAST => {direction = direction::SOUTH}
                direction::WEST => {direction = direction::NORTH}
            }
        } else {
            visited.insert((i,j));
            i = nextI;
            j = nextJ;
        }
    }





    let mut result = visited.len();


    println!("result = {:?}", result);

    Ok(())
}
