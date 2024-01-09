use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use regex::Regex;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Part {
    x: usize,
    m: usize,
    a:usize,
    s: usize
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Condition {
    field: char,
    op: char,
    value:usize,
    next: String
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


fn main() -> io::Result<()> {
    let mut total = 0;
    let mut result = 0;
    let mut map: Vec<Vec<bool>> = Vec::new();
    
    let groups = group_text("sample.txt");

    let mut start:(usize, usize) = (0,0);
    for row in groups[0].iter().enumerate() {
        let mut map_row:Vec<bool> = Vec::new(); 
        for col in row.1.chars().enumerate() {
            if col.1 == 'S' {
                start = (row.0, col.0);
            }
            if col.1 == '#' {
                map_row.push(false);
            }
            else {
                map_row.push(true);
            }
        }
        map.push(map_row);
    }

    println!("Start {:?}", start);
    println!("{:?}", map);

    result = bfs(&map, start, 64);

    println!("total = {}", result);
    Ok(())
}

fn bfs(map: &Vec<Vec<bool>>, start: (usize, usize), steps: usize) -> usize {

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    set.insert(start);
    let mut x = 0;
    while x < steps {
        
        let mut new_set:HashSet<(usize,usize)> = HashSet::new();
        for e in set.iter() {
          for x in next_steps(map, e).iter() {
              new_set.insert(*x);
          }  
        }
        set = new_set;
        x += 1;
    }
    set.len()
}

fn next_steps(map: &Vec<Vec<bool>>, p: &(usize, usize)) -> Vec<(usize, usize)> {
 let mut result: Vec<(usize, usize)> = Vec::new();
   if p.0 > 0 && map[p.0-1][p.1] {
     result.push((p.0-1, p.1));
   }
   if p.0 < map.len() -1 && map[p.0+1][p.1] {
       result.push((p.0+1, p.1));
   }
   if p.1 >0 && map[p.0][p.1 -1] {
       result.push((p.0, p.1-1));
   }
   if p.1 < map[p.0].len() -1 && map[p.0][p.1+1] {
       result.push((p.0, p.1+1));
   }

 result

}

