use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;

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


fn main() -> io::Result<()> {
    let mut total = 0;
    let mut instructions: Vec<(char, usize)> = Vec::new();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut dug_squares: HashSet<(i32, i32)> = HashSet::new();    
    for group in group_text("sample.txt").iter(){

        for line in group.iter() {
            let words:Vec<&str> = line.split(' ').collect();
            let c:char = words[0].parse().unwrap();
            let x:usize = words[1].parse().unwrap();
            println!("{} {}", c, x);
            instructions.push((c,x));
        }
    }

    let mut i:i32 = 0;
    let mut j:i32 = 0;
    for instruction in instructions.iter() {
        let c = instruction.0;
        let x = instruction.1;
        println!("{} {} {} {}", i, j, c, x);
        let mut z = 0;
        while z < x {
            if c == 'R' {
                j += 1;
            }
            if c == 'L' {
                j -= 1;
            }
            if c == 'U' {
                i -= 1;
            }
            if c == 'D' {
                i += 1;
            }
            dug_squares.insert( (i,j) );
            z += 1;
        }

    }

    let mut visit: Vec<(i32, i32)> = Vec::new();
    visit.push((1,1));

    while !visit.is_empty() {
        let pos = visit.pop().unwrap();
        println!("checking {:?}", pos);

        if dug_squares.contains(&pos) {
            continue;
        }

        dug_squares.insert(pos);
        visit.push( (pos.0 - 1, pos.1) );
        visit.push( (pos.0 + 1, pos.1) );
        visit.push( (pos.0, pos.1 - 1) );
        visit.push( (pos.0, pos.1 + 1) );
    }


    println!("{:?}", instructions);
    for v in map.iter() {
      println!("{:?}", v);
    }

    println!("{:?}" , dug_squares);
    println!("{}" , dug_squares.len());
    Ok(())
}

