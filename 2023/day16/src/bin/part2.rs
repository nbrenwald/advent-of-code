use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Above,
    Below,
    Left,
    Right
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
    let mut map: Vec<Vec<char>> = Vec::new();
    for group in group_text("sample.txt").iter(){

        for line in group.iter() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars(){
                row.push(c);
            }
            map.push(row);
        }
    }

    let mut i = 0;
    while i < map.len() {

      let mut temp:HashSet<(usize, usize)> = HashSet::new();
      for x in bfs(&map, (Direction::Left, (i, 0))).iter(){
        temp.insert(x.1);
      }
      result = cmp::max(result, temp.len());
      i+= 1;
    }

    let mut i = 0;
    while i < map.len() {

      let mut temp:HashSet<(usize, usize)> = HashSet::new();
      for x in bfs(&map, (Direction::Right, (i, map.len()-1))).iter(){
        temp.insert(x.1);
      }
      result = cmp::max(result, temp.len());
      i += 1;
    }
    let mut i = 0;
    while i < map[0].len() {

      let mut temp:HashSet<(usize, usize)> = HashSet::new();
      for x in bfs(&map, (Direction::Above, (0, i))).iter(){
        temp.insert(x.1);
      }
      result = cmp::max(result, temp.len());
      i+= 1;
    }

    let mut i = 0;
    while i < map[0].len() {

      let mut temp:HashSet<(usize, usize)> = HashSet::new();
      for x in bfs(&map, (Direction::Below, (map[0].len()-1, i))).iter(){
        temp.insert(x.1);
      }
      result = cmp::max(result, temp.len());
      i += 1;
    }

        println!("result = {} ", result);
    Ok(())
}

fn bfs(map: &Vec<Vec<char>>, start: (Direction, (usize, usize))) -> HashSet<(Direction, (usize, usize))> {
   let mut visited: HashSet<(Direction, (usize, usize))> = HashSet::new();

   let mut visit: Vec<(Direction, (usize, usize))> = Vec::new();
   visit.push(start);

   while (!visit.is_empty()) {
       let tile = visit.pop().unwrap();
       let next = visit_next(map, &tile);
       visited.insert(tile);

       for t in next.iter() {
           if ! visited.contains(t) {
               visit.push((*t).clone());
           }
       }
   }

   visited

}

fn visit_next(map: &Vec<Vec<char>>, tile: &(Direction, (usize, usize))) -> Vec<(Direction, (usize, usize))> {
    let mut next: Vec<(Direction, (usize, usize))> = Vec::new();
    let row = tile.1.0;
    let col = tile.1.1;
    let dir = tile.0;
    let c = map[row][col];

    let up = get_up(map, (row, col));
    let down = get_down(map, (row, col));
    let left = get_left(map, (row, col));
    let right = get_right(map, (row, col));
   
    match c {
      '.' => {
          match dir {
              Direction::Above => { 
                  if down.is_some(){
                      next.push( (dir, down.unwrap() ) );
                  }
              }
              Direction::Below => {

                  if up.is_some(){
                      next.push( (dir, up.unwrap() ) );
                  }
              }
              Direction::Left => {
                  if right.is_some(){
                      next.push( (dir, right.unwrap() ) );
                  }

              }
              Direction::Right => {

                  if left.is_some(){
                      next.push( (dir, left.unwrap() ) );
                  }
              }
          }
      }
      '-' => {

          match dir {
              Direction::Above | Direction::Below => { 
                  if left.is_some(){
                      next.push( (Direction::Right, left.unwrap() ) );
                  }
                  if right.is_some(){
                      next.push( (Direction::Left, right.unwrap() ) );
                  }
              }
              Direction::Left => {
                  if right.is_some(){
                      next.push( (dir, right.unwrap() ) );
                  }

              }
              Direction::Right => {

                  if left.is_some(){
                      next.push( (dir, left.unwrap() ) );
                  }
              }
          }
      }
      '|' => {

          match dir {
              Direction::Above => { 
                  if down.is_some(){
                      next.push( (dir, down.unwrap() ) );
                  }
              }
              Direction::Below => {

                  if up.is_some(){
                      next.push( (dir, up.unwrap() ) );
                  }
              }
              Direction::Left | Direction::Right => {
                  if up.is_some(){
                      next.push( (Direction::Below, up.unwrap() ) );
                  }
                  if down.is_some(){
                      next.push( (Direction::Above, down.unwrap() ) );
                  }

              }
          }
      }
      '/' => {

          match dir {
              Direction::Above => { 
                  if left.is_some(){
                      next.push( (Direction::Right, left.unwrap() ) );
                  }
              }
              Direction::Below => {

                  if right.is_some(){
                      next.push( (Direction::Left, right.unwrap() ) );
                  }
              }
              Direction::Left => {
                  if up.is_some(){
                      next.push( (Direction::Below, up.unwrap() ) );
                  }

              }
              Direction::Right => {

                  if down.is_some(){
                      next.push( (Direction::Above, down.unwrap() ) );
                  }
              }
          }
      }
      '\\' => {

          match dir {
              Direction::Above => { 
                  if right.is_some(){
                      next.push( (Direction::Left, right.unwrap() ) );
                  }
              }
              Direction::Below => {

                  if left.is_some(){
                      next.push( (Direction::Right, left.unwrap() ) );
                  }
              }
              Direction::Left => {
                  if down.is_some(){
                      next.push( (Direction::Above, down.unwrap() ) );
                  }

              }
              Direction::Right => {

                  if up.is_some(){
                      next.push( (Direction::Below, up.unwrap() ) );
                  }
              }
          }
      }
      _ => {}
    }
    next
}

fn get_up(map: &Vec<Vec<char>>, current: (usize, usize)) -> Option<(usize, usize)> {
    if current.0 == 0 {
        return None;
    }
    Some( (current.0 - 1, current.1))
}

fn get_down(map: &Vec<Vec<char>>, current: (usize, usize)) -> Option<(usize, usize)> {
    if current.0 == map.len() -1 {
        return None;
    }
    Some( (current.0 + 1, current.1))
}
fn get_left(map: &Vec<Vec<char>>, current: (usize, usize)) -> Option<(usize, usize)> {
    if current.1 == 0 {
        return None;
    }
    Some( (current.0, current.1 - 1))
}
fn get_right(map: &Vec<Vec<char>>, current: (usize, usize)) -> Option<(usize, usize)> {
    if current.1 == map[0].len()-1 {
        return None;
    }
    Some( (current.0, current.1 + 1))
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

 
