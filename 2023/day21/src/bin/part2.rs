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

    let grown_map = grow_map(&map, 101);

    println!("Start {:?}", start);
    //println!("{:?}", grown_map);

    //println!("{}", grown_map.len()/2);
    //println!("{}", grown_map[6615].len()/2);
    result = bfs(&grown_map, (6615, 6615), 65);
    //result = bfs(&grown_map, (6615, 6615), 66);
   // result = bfs(&grown_map, (6615, 6615), 196);
    //result = bfs(&grown_map, (13165, 13165), 327);
    //result = bfs(&grown_map, (6615, 6615), 589);
    //result = bfs(&grown_map, (6615, 6615), 458);
    //result = bfs(&grown_map, (555, 555), 100);

    let first:u64 = ( 14494u64 * (66u64 * 66u64) ) / 17161u64;
    let second:u64 = ( 38467u64 * 66u64 ) / 17161u64;
    let third:u64 = 187220u64 / 17161u64;
    let r:u64 = first + second + third;
    println!("{} {} {} {}", first, second, third, r);

    //from wolfram quadratic equation calculator
    let first:f64 = ( 14494f64 * (26501365f64 * 26501365f64) ) / 17161f64;
    let second:f64= ( 38467f64 * 26501365f64 ) / 17161f64;
    let third:f64 = 187220f64 / 17161f64;
    let r:f64 = first + second + third;
    println!("{} {} {} {}", first, second, third, r);
    //from wolfram quadratic equation calculator
    let first:f64 =  0.844589 * (26501365f64 * 26501365f64);
    let second:f64 = 2.24154 * 26501365f64;
    let third:f64 = 10.9096;
    let r:f64 = first + second + third;
    println!("{} {} {} {}", first, second, third, r);
    println!("total = {}", result);
    Ok(())
}

fn grow_map(map: &Vec<Vec<bool>>, factor: usize) -> Vec<Vec<bool>> {
  let mut tmp_result: Vec<Vec<bool>> = Vec::new();
  let mut result: Vec<Vec<bool>> = Vec::new();

  for r in map.iter() {

  let mut x = 0;
   let mut row: Vec<bool> = Vec::new();
  
    while x < factor {
        row.extend(r.iter());
        x += 1;
    }
    tmp_result.push(row);
  }

  let mut x = 0;
  while x< factor {
     for r in tmp_result.iter() {
         result.push(r.clone());
     }
     x += 1;
  }
  result
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

