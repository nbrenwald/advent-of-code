use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use regex::Regex;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
    z:usize
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
struct Brick {
    start: Coord,
    end: Coord,
    id:usize
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
    let mut result = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    
    let groups = group_text("sample.txt");
    let re = Regex::new(r"(.*)~(.*)").unwrap();
    let mut bricks: Vec<Brick> = Vec::new();
    let mut id = 0;
    for row in groups[0].iter() {
       map.push(row.chars().collect());
    }
    println!("{:?}", map);
    let mut visited:HashSet<(usize,usize)> = HashSet::new();
    result = backtrack(&map, &mut visited, (0,1), 0);

    println!("{}",result);

    Ok(())
}


fn backtrack(map:&Vec<Vec<char>>, visited:&mut HashSet<(usize,usize)>, pos:(usize,usize), x: usize) -> usize {
    println!("Checking {:?}", pos);
    let mut result = 0;
    if pos.0 == map.len() - 1 {
        println!("Found a path of length {}", x);
        return x;
    }

    if map[pos.0][pos.1] == '#' {
        return 0
    }

    visited.insert(pos);
    let c = map[pos.0][pos.1];
    if pos.0 < map.len() -1 && !visited.contains(&(pos.0+1, pos.1)) && c != '^' && c != '<' && c != '>'{
        result = cmp::max(result,backtrack(map, visited, (pos.0+1, pos.1), x+1));
    }
    if pos.0 > 0 && !visited.contains(&(pos.0-1, pos.1)) && c != 'v' && c != '<' && c != '>'{
        result = cmp::max(result,backtrack(map, visited, (pos.0-1, pos.1), x+1));
    }
    if pos.1 < map[0].len() -1 && !visited.contains(&(pos.0, pos.1+1))  && c != '^' && c != '<' && c != 'v'{
        result = cmp::max(result,backtrack(map, visited, (pos.0, pos.1+1), x+1));
    }
    if pos.1 >0 && !visited.contains(&(pos.0, pos.1-1)) && c != '^' && c != 'v' && c != '>'{
        result = cmp::max(result,backtrack(map, visited, (pos.0, pos.1-1), x+1));
    }

    visited.remove(&pos);

result
}

