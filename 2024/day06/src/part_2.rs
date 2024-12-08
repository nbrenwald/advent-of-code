use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io;
use utils::direction;
use utils::direction::NORTH;

#[derive(Eq, Hash, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
    d: direction
}

fn group_text(file: &str) -> Vec<Vec<String>> {
    let mut group:Vec<String> = Vec::new();
    let mut text:Vec<Vec<String>> = Vec::new();
    for line in read_to_string(file).unwrap().lines() {
        if line == "" {
            if group.len() != 0 {
                text.push(group);
                group = Vec::new();
            }
        }
        else {
            group.push(line.to_string());
        }
    }
    if group.len() != 0 {
        text.push(group);
    }
    text
}


pub(crate) fn part2(path: &str) -> io::Result<()> {

    let mut grid : Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    let mut j = 0;

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




    let visited = cellsVisited(&grid, i, j);
    let mut result: usize = 0;
    let total = grid[0].len() * grid.len();
    let mut checked: usize = 0;
    //while x < grid.len() {
    //    let mut y = 0;
    //    while y < grid[0].len() {
for (x,y) in visited { // Optimization from reddit, only start with the cells visited from part 1.
            //println!("Checking {} / {}", checked, total);
            if grid[x][y] == '.' {
                grid[x][y] = '#';
                if inLoop(&grid, i, j) {

                    result += 1;
                }
                grid[x][y] = '.';
            }

            //checked += 1;
     //       y+=1;
      //  }
     //   x += 1;
    }


    println!("result = {:?}", result);
    //1784

    Ok(())
}


fn inLoop(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut i = i;
    let mut j = j;
    let mut direction= NORTH;

    let mut visited:HashSet<Cell> = HashSet::new();

    while i > 0 && i < grid.len() -1 && j>0 && j < grid[0].len()-1 {

        let mut nextI = i;
        let mut nextJ = j;

        match direction {
            direction::NORTH => { nextI -= 1;}
            direction::SOUTH => { nextI += 1;}
            direction::EAST => {nextJ += 1;}
            direction::WEST => {nextJ -= 1;}
        }

        if grid[nextI][nextJ] == '#' {
            match direction {
                direction::NORTH => { direction = direction::EAST; }
                direction::SOUTH => { direction = direction::WEST}
                direction::EAST => {direction = direction::SOUTH}
                direction::WEST => {direction = direction::NORTH}
            }
        } else {
            let nextCell = Cell {x: i,y: j, d: direction};
            if visited.contains(&nextCell) { return true}
            visited.insert(nextCell);
            i = nextI;
            j = nextJ;
        }
    }

    false
}


fn cellsVisited(grid: &Vec<Vec<char>>, i: usize, j: usize) -> HashSet<(usize, usize)> {
    let mut i = i;
    let mut j = j;
    let mut direction= NORTH;

    let mut visited:HashSet<(usize,usize)> = HashSet::new();

        while i >= 0 && i < grid.len() && j>=0 && j < grid[0].len() {

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

     visited
}
