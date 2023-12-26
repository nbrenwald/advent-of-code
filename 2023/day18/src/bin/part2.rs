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
            //let c:char = words[0].parse().unwrap();
            let x:usize = words[1].parse().unwrap();
            let y:String = words[2].chars().skip(2).take(5).collect();
            let yy = usize::from_str_radix(&y, 16).ok().unwrap();
            let c = match words[2].chars().skip(7).nth(0).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => ' '
            };
            println!("{} {} {}", c, x, yy);
            instructions.push((c,yy));
        }
    }

    let mut dug_squares: HashSet<(i64, i64)> = HashSet::new();
    let mut i:i64 = 0;
    let mut j:i64 = 0;
    for instruction in instructions.iter() {
        let c = instruction.0;
        let x = instruction.1;
        //println!("{} {} {} {}", i, j, c, x);
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

    let mut i:i64 = 0;
    let mut j:i64 = 0;
    let mut vertices:Vec<(i64, i64)> = Vec::new();
    for instruction in instructions.iter() {
        let c = instruction.0;
        let x:i64 = instruction.1.try_into().unwrap();
        //println!("{} {} {} {}", i, j, c, x);
            if c == 'R' {
                j += x;
            }
            if c == 'L' {
                j -= x;
            }
            if c == 'U' {
                i -= x;
            }
            if c == 'D' {
                i += x;
            }

            vertices.push( (i, j));

    }

  vertices.reverse();

    println!("{:?}", vertices);

    let mut i = 0;
    let mut firstSum = 0;
    while i < vertices.len() -1 {
        firstSum += vertices[i].0 * vertices[i+1].1;
        i += 1;
    }
    firstSum += vertices[i].0 * vertices[0].1;
    
    let mut i = 0;
    let mut secondSum = 0;
    while i < vertices.len() -1 {
        secondSum += vertices[i].1 * vertices[i+1].0;
        i += 1;
    }
    secondSum += vertices[i].1 * vertices[0].0;


    let result = ((secondSum-firstSum)/2).abs();
    println!("{} {} {} {}", firstSum, secondSum, result, dug_squares.len() /2);
    Ok(())
}

