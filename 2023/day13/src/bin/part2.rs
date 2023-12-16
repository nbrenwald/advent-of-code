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
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in group.iter() {
            let mut row:Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            map.push(row);
        }

        let mut l = 0;
        while l < map[0].len() - 1 {
            if is_mirror_col(l, l+1, &map, 0) {
                println!("found a mirror col {} {}", l, l+1);
                result += l+1;
                break;
            }
            l += 1;
        }

        let mut top = 0;
        while top < map.len() -1 {

            if is_mirror_row(top, top+1, &map, 0) {
              println!("found a mirror row {} {}", top, top+1);
              result += 100 * (top+1);
              break;
            }
            top += 1;
        }

    }
        println!("result = {}", result);
    Ok(())
}


fn is_mirror_row(top: usize, bottom:usize, map:&Vec<Vec<char>>,  mut smudge: usize) -> bool {
    let mut i = 0;
    while i < map[0].len() {
        if map[top][i] != map[bottom][i] {
            smudge += 1;
        }
        i += 1;
    }

    if smudge > 1 {
        return false;
    }

    if (top == 0 || bottom == map.len()-1) {
        if smudge == 1 {
            return true;
        }
        else {
            return false;
        }
    }

    is_mirror_row(top-1, bottom+1, map, smudge)
}

fn is_mirror_col(left: usize, right:usize, map:&Vec<Vec<char>>, mut smudge: usize) -> bool {
    let mut i = 0;
    while i < map.len() {
        if map[i][left] != map[i][right] {
            smudge += 1;
        }
        i += 1;
    }

    if smudge > 1 {
        return false;
    }

    if left == 0 || right == map[0].len() - 1 {
        if smudge == 1{
        return true;
        }
        else {
            return false;
        }
    }

    is_mirror_col(left-1, right+1, map, smudge)
}



fn all_zero(v:&Vec<i64>) -> bool {
    for x in v.iter() {
        if *x != 0i64 {
            return false;
        }
    }
    true
}

 
