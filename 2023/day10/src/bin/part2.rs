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

    for line in read_to_string("sample.txt").unwrap().lines().enumerate() {
       row = Vec::new();
       let line_string = line.1.to_string();
       for c in line_string.chars().enumerate() {
           row.push(c.1);
           if c.1 == 'S' {
               println!("found S at {} {}", line.0, c.0);
               s = (line.0, c.0);
           }
       }
       map.push(row);
    }
    
    let start_chars:Vec<char> = vec!['J', 'F', '7', 'L', '|', '-'];
    //for c in start_chars.iter() {
        visited = HashSet::new();
    let mut nextVec: Vec<(usize, usize)> = next(s, 'J', &mut visited);

    //println!("{:?}", nextVec);
    let mut steps = 0;
    while nextVec.len() == 2 {
      steps += 1;
      //println!("{:?}", nextVec);
      let mut n:Vec<(usize, usize)> = Vec::new();
      let mut a = next(nextVec[0], map[nextVec[0].0][nextVec[0].1], &mut visited );
      //println!("a={:?}", a);
      n.append(&mut a);
      let mut b = next(nextVec[1], map[nextVec[1].0][nextVec[1].1], &mut visited );
      //println!("b={:?}",b);
      n.append(&mut b);
      nextVec = n;
    }

       println!("solution {} steps  and end vec {:?}", steps, nextVec);
       println!("{} {:?}", visited.len(), visited);

    let mut is_in_vector: Vec<Vec<bool>> = Vec::new();
    let mut i = 0;
    let mut total = 0;
    while i < map.len() {
        let mut j = 0;
        let mut is_inside = false;
        let mut row: Vec<bool> = Vec::new();
        while j < map[i].len() {
            if visited.contains(&(i,j)) && ( map[i][j] == '|' || map[i][j] == 'F' || map[i][j] == '7' ) {
              is_inside = ! is_inside;
            }
            if is_inside && !visited.contains(&(i,j)){
                total += 1;
            }
            row.push(is_inside);
            j +=1;
        }
        is_in_vector.push(row);
        i += 1;
    }
    //println!("{:?}", is_in_vector);
    println!("Total = {}", total);
    Ok(())
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
