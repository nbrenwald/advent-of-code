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
    let mut boxes : Vec<Vec<(String, u32)>> = Vec::new();
    let mut i = 0;
    while i < 256 {
        let mut b: Vec<(String, u32)> = Vec::new();
        boxes.push(b);
        i += 1;
    }

    for group in group_text("sample.txt").iter(){
        for line in group.iter() {
            for str in line.split(",").collect::<Vec<_>>().iter(){
                let lens = str.chars().nth(str.len()-1).unwrap();
                if lens == '-' {
                    let label = &str[..str.len()-1];
                    let label_hash = hash(label);
                    println!("remove {} from box {}", label, label_hash);
                    remove(&mut boxes, label.to_string(), label_hash.try_into().unwrap());

                }
                else { 
                    let label = &str[..str.len()-2];
                    let label_hash = hash(label);
                    println!("add {} {} to box {}", label, lens, label_hash);
                    add(&mut boxes, (label.to_string(), lens.to_digit(10).unwrap()), label_hash.try_into().unwrap());
                }
                println!("{} {}",str, lens);
                println!("{:?}", boxes);
            }
                result += score(&boxes);
        }
    }
        println!("result = {}", result);
    Ok(())
}

fn score(boxes:&Vec<Vec<(String, u32)>>) -> usize {
   let mut b = 0;
   let mut result  = 0;
   while b < boxes.len() {
       let mut l = 0;
       while l < boxes[b].len() {
           let lens:usize = boxes[b][l].1.try_into().unwrap();
           let r = (b+1) * (l+1) * lens; 
           println!("{} {} {} {} {}", b, l, lens, r , result);
           result += r; 
           l += 1;
       }
       b += 1;
   }
   println!("{}", result);
   result
}

fn add(boxes: &mut Vec<Vec<(String, u32)>>, l : (String, u32), b: usize) {
   let mut b_v: &mut Vec<(String, u32)> = boxes.get_mut(b).unwrap();
   let mut i = usize::MAX;
   for x in b_v.iter().enumerate() {
       if x.1.0 == l.0 {
           i = x.0
       }
   }
   if i == usize::MAX {
     b_v.push(l);
   }
   else {
       b_v.remove(i);
       b_v.insert(i, l);
   }
}

fn remove(boxes: &mut Vec<Vec<(String, u32)>>, label:String, b: usize) {
  
   let b_v:&mut Vec<(String, u32)> = boxes.get_mut(b).unwrap();
   let mut i = usize::MAX;
   for x in b_v.iter().enumerate(){
       if x.1.0 == label {
           i = x.0;

       }
   }
   if i != usize::MAX {
   b_v.remove(i);
   }
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

 
