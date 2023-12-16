use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;



fn main() -> io::Result<()> {
    let mut total = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    let mut s = (0,0);
    let mut visited : HashSet<(usize, usize)> = HashSet::new();

    for line in read_to_string("test-sample.txt").unwrap().lines().enumerate() {
       row = Vec::new();
       let line_string = line.1.to_string();
       for c in line_string.chars().enumerate() {
           row.push(c.1);
       }
       map.push(row);
    }


    let mut expanded_cols: Vec<Vec<char>> = Vec::new();
    let mut expanded: Vec<Vec<char>> = Vec::new();
    let mut empty_rows: HashSet<usize> = HashSet::new();
    let mut empty_cols: HashSet<usize> = HashSet::new();
    

    let mut i = 0;
    while i < map.len() {
        if row_is_empty(&map, i) {
            empty_rows.insert(i);
            println!("found an empty row {}", i);
        }
        i += 1;
    }

    let mut i = 0;
    while i < map[0].len() {
        if col_is_empty(&map, i) {
            empty_cols.insert(i);
            println!("found an empty col {}", i);
        }
        i += 1;
    }

    let mut i = 0;
    while i < map.len() {
        let mut j = 0;
        let mut row:Vec<char> = Vec::new();
        while j< map[i].len() {
            if empty_cols.contains(&j) {
                row.push(map[i][j]);
            }
            row.push(map[i][j]);
            j += 1;
        }
        expanded_cols.push(row);
        i += 1;
    }

    let mut i = 0;
    while i < expanded_cols.len() {
        let v = expanded_cols[i].clone();

        if empty_rows.contains(&i) {
        expanded.push(expanded_cols[i].clone().clone());
        }
        expanded.push(expanded_cols[i].clone().clone());

        i += 1;
    }
    println!("{:?}", map);
    println!("{:?}", expanded_cols );
    println!("{:?}", expanded );

    let mut galaxies: Vec<(usize,usize)> = Vec::new();
    let mut i = 0;
    while i < expanded.len() {
        let mut j = 0;
        while j < expanded[i].len(){
            if expanded[i][j] == '#' {
                galaxies.push( (i,j) );
            }
            j += 1;
        }
        i+=1;
    }

    let mut i = 0;
    let mut distance = 0;
    while i < galaxies.len(){
        let mut j = 0;
        while j < galaxies.len() {
            if i != j {
                let dX = galaxies[i].0.abs_diff(galaxies[j].0);
                let dY = galaxies[i].1.abs_diff(galaxies[j].1);
                distance += dX + dY;
                println!("distance between {:?} and {:?} is {} {}", galaxies[i], galaxies[j], dX, dY);
            }
            j += 1;
        }
        i+=1;
    }
   println!("distance = {}", distance); 
    Ok(())
}

fn col_is_empty(v: &Vec<Vec<char>>, col: usize) -> bool {
    let mut i = 0;
    while i < v.len() {
        if v[i][col] != '.' {
            return false;
        }
        i += 1;
    }
    true
}

fn row_is_empty(v: &Vec<Vec<char>>, row: usize) -> bool {
    let mut i = 0;
    while i < v[row].len() {
        if v[row][i] != '.' {
            return false;
        }
        i += 1;
    }
    true
}

fn next(curr: (usize, usize), c: char, visited: &mut HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    //println!("next called for {}, vistited={:?}", c, visited);

    visited.insert(curr);
    let mut result: Vec<(usize, usize)> = Vec::new();
    if c == '|' {
        if !visited.contains( &((curr.0 -1), curr.1) ) {
            result.push( ((curr.0 -1), curr.1) );
        }
        if !visited.contains( &(curr.0+1, curr.1 ) ) {
            result.push( (curr.0+1, curr.1) );
        }
          
    } 
    if c == '-' {
        if !visited.contains( &(curr.0, curr.1-1) ) {
            result.push( (curr.0, curr.1-1) );
        }
        if !visited.contains( &(curr.0, curr.1+1 ) ) {
            result.push( (curr.0, curr.1+1) );
        }
          
    } 
    if c == 'F'{

        if !visited.contains( &((curr.0 + 1), curr.1) ) {
            result.push( ((curr.0 +1 ), curr.1) );
        }

        if !visited.contains( &((curr.0), curr.1 + 1) ) {
            result.push( ((curr.0), curr.1 +1 ) );
        }
    }
    if c == 'L' {
        if !visited.contains( &(curr.0 -1, curr.1) ) {
            result.push( (curr.0 -1, curr.1) );
        }
        if !visited.contains( &(curr.0, curr.1+1 ) ) {
            result.push( (curr.0, curr.1+1) );
        }
    } 
    if c == 'J' {
        if !visited.contains( &(curr.0 -1, curr.1) ) {
            result.push( (curr.0 -1, curr.1) );
        }
        if !visited.contains( &(curr.0, curr.1-1 ) ) {
            result.push( (curr.0, curr.1-1) );
        }
    } 
    if c == '7' {
        if !visited.contains( &(curr.0, curr.1-1) ) {
            result.push( (curr.0, curr.1-1) );
        }
        if !visited.contains( &(curr.0+1, curr.1 ) ) {
            result.push( (curr.0+1, curr.1) );
        }
    } 
    result
}
