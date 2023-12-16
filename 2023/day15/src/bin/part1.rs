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
    let mut result = 0;
    for group in group_text("sample.txt").iter(){
        for line in group.iter() {
            for str in line.split(",").collect::<Vec<_>>().iter(){
                println!("{}",str);
                result += hash(str);
            }
        }
    }
        println!("result = {}", result);
    Ok(())
}

fn hash(input: &str) -> u32 {
    let mut current: u32 = 0;
    for c in input.chars() {
        current += c as u32;
        current *= 17;
        current %= 256;
    }
    current
}

fn calculate(map: &Vec<Vec<char>>, col:usize) -> usize{
    let mut base = 0;
    let mut i = 0;
    let mut total = 0; 
    while i < map.len() {
        if map[i][col] == 'O' {
            total += map.len() - base;
            base += 1;
        }   
        else if map[i][col] == '#' {
            base = i + 1;
        }
        i += 1;
    }
    total
}


fn is_mirror_row(top: usize, bottom:usize, map:&Vec<Vec<char>>) -> bool {
    let mut i = 0;
    while i < map[0].len() {
        if map[top][i] != map[bottom][i] {
            return false;
        }
        i += 1;
    }

    if top == 0 || bottom == map.len()-1 {
        return true;
    }

    is_mirror_row(top-1, bottom+1, map)
}

fn is_mirror_col(left: usize, right:usize, map:&Vec<Vec<char>>) -> bool {
    let mut i = 0;
    while i < map.len() {
        if map[i][left] != map[i][right] {
            return false;
        }
        i += 1;
    }

    if left == 0 || right == map[0].len() - 1 {
        return true;
    }

    is_mirror_col(left-1, right+1, map)
}



fn all_zero(v:&Vec<i64>) -> bool {
    for x in v.iter() {
        if *x != 0i64 {
            return false;
        }
    }
    true
}

 
