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

       let mut i = 0;
       while i < map[0].len() {
           result += calculate(&map, i);
           i += 1;
       }

      let mut x = 0;
      let mut deltas : Vec<Vec<(usize, usize)>> = Vec::new();
      let mut scores: HashMap<String, (String, usize)> = HashMap::new();
      let mut score = 0;
      let mut delta_key = "".to_string();
      while x < 1000000000 {
          if x % 100000000 == 0 {
              println!("done another 100K");
          }

       if !scores.contains_key(&delta_key) { 
       
       let mut nm = north(&mut map);
       nm = rotate(&mut nm);
       nm = north(&mut nm);
       nm = rotate(&mut nm);
       nm = north(&mut nm);
       nm = rotate(&mut nm);
       nm = north(&mut nm);
       nm = rotate(&mut nm);

       score = calculate_score(&nm);
       let new_key = same(&map, &nm);
       scores.insert(delta_key, (new_key.clone(), score));
       delta_key = new_key;
       map = nm;}
       else {
           let (new_key, new_score) = scores.get(&delta_key).unwrap();
           delta_key = new_key.to_string();
           score = *new_score;
           //println!("{}", new_score);
       }
       x+= 1;
       }
      println!("final Score = {}", score);

       //println!("{:?}", scores);
      
    }
        println!("result = {}", result);
    Ok(())
}

fn calculate_score(map: &Vec<Vec<char>>) -> usize {

let mut result = 0;
    let mut i = 0;
    while i < map.len() {
        let mut j = 0 ;
        while j < map[0].len() {
            if map[i][j] == 'O' {
                result += map.len() - i;
            }
            j +=1;
        }
        i+= 1;
    }
   result 

}

fn same(orig:&Vec<Vec<char>>, new: &Vec<Vec<char>>) -> String {
       /*println!("", );
       for v in orig.iter() {
       println!("{:?}", v);
       }
       println!("", );
       for v in new.iter() {
       println!("{:?}", v);
       }*/
    let mut result: String = String::new();
    let mut i = 0;
    while i < orig.len() {
    let mut j = 0;
       while j < orig[0].len(){
            if orig[i][j] != new[i][j] {
                println!("not match at i={}, j={} {} != {}", i, j, orig[i][j], new[i][j]);
                result = format!("{}({},{})", result, i,j);
            } 
           j +=1;
       }
    i += 1;
    }
    result
}

fn north (map: &Vec<Vec<char>>) -> Vec<Vec<char>> {

    let  mut result: Vec<Vec<char>> = Vec::new();
    for r in map.iter() {
        result.push(Vec::new());
    }
    
    let mut col = 0;
    while col < map[0].len() {
        let mut row = 0;
        let mut dotCounter = 0;
        let mut writePtr = 0;
        while row < map.len() {
            
          if map[row][col] == 'O' {
              result[writePtr].push('O');
              writePtr += 1;
              //map[row].get_mut(writePtr).unwrap() = 'O';
          } 
          else if map[row][col] == '.' {
              dotCounter += 1;
          }
          else if map[row][col] == '#' {
             while dotCounter > 0 {
                 result[writePtr].push('.');
                 writePtr += 1;
                 dotCounter -= 1;
             }

             result[writePtr].push('#');
             writePtr += 1;
          }
          row += 1;
        }

             while dotCounter > 0 {
                 result[writePtr].push('.');
                 writePtr += 1;
                 dotCounter -= 1;
             }

      col += 1;
    }
       result
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

fn rotate(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  // col 0 to become first row
    let  mut result: Vec<Vec<char>> = Vec::new();

    let mut col = 0;
    while col < map[0].len() {
        let mut newRow:Vec<char> = Vec::new();
        let mut row = map.len();
        while row > 0 {

            row -= 1;
            newRow.push(map[row][col]);
        }
        result.push(newRow);
        col +=1 ;
    }
    result
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

 
