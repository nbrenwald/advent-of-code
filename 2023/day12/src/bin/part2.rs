use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;



fn main() -> io::Result<()> {
    let mut total = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    let mut s = (0,0);
    let mut visited : HashSet<(usize, usize)> = HashSet::new();
let mut total = 0;
    for line in read_to_string("sample.txt").unwrap().lines().enumerate() {
       row = Vec::new();
       let line_string = line.1.to_string();
       let line_split: Vec<&str> = line_string.split(" ").collect();
       let record = line_split[0].to_string();
       let recordRepeated = format!("{v}?{v}?{v}?{v}?{v}", v=record);
       //let recordRepeated = format!("{v}", v=record);
       
       let check_string = line_split[1].to_string();
       let check_repeated = format!("{v},{v},{v},{v},{v}", v = check_string);
       //let check_repeated = format!("{v}", v = check_string);
       let check:Vec<usize> = check_repeated.split(",").map(|x| x.parse().unwrap()).collect();
//   println!("distance = {}, check={:?}", record, check); 

    let mut mem : HashMap<String, bool> = HashMap::new();
   let result = recursive(&recordRepeated, "".to_string(), 0, &check, &mut mem);
   total += result;
    println!("{}",  result);
    }
    println!("{}",  total);



    Ok(())
}

fn recursive(record: &String,  partial: String, index: usize, check: &Vec<usize>, h: &mut HashMap<String, bool>) -> usize {
    let pv = is_partial_valid(record, &partial, check, h);
    if !pv {
        return 0;
    }
    if index == record.len()
    {
  //      println!("IS VALID {}", partial);
        if is_valid(&partial, check) {
            return 1;
        }
        else {
            return 0;
        }
    }

    let c = record.chars().nth(index).unwrap();
    let mut x = 0;
    let mut y = 0;
    if c == '?' {
       let mut x_string = partial.clone();
       x_string.push('#');
       x = recursive(record, x_string, index + 1, check, h);
       let mut y_string = partial.clone();
       y_string.push('.');
       y = recursive(record, y_string, index + 1, check, h);
     }
    else {
       let mut y_string = partial.clone();
       y_string.push(c);
       y = recursive(record, y_string, index + 1, check, h);
    }

    return x + y;
}

fn is_partial_valid(record: &String, s:&String, c:&Vec<usize>, h: &mut HashMap<String, bool>) -> bool { 
    //println!("check {}", s);
if h.contains_key(s) {
    println!("Cache hit {}", s);
    return *h.get(s).unwrap();
}
   let mut i = 0;
   let mut count = 0;
   let mut result: Vec<usize> = Vec::new();
    for x in s.chars() {
        if x == '.' {
            if count > 0 {
                result.push(count);
                count = 0
            }
        }
        if x == '#' {
            count += 1;
        }
    }
    if count > 0 {
        result.push(count);
    }

    if result.len() > c.len() {
        h.insert(s.clone(), false);
        return false;
    }

    if c.iter().sum::<usize>() <  result.iter().sum::<usize>() {
        return false;
    }
    let remaining = record.len() - s.len();
    let uncounted = c.iter().sum::<usize>() - result.iter().sum::<usize>();
    if remaining < uncounted {
        return false;
    }

    if result.len() == 0 {
        h.insert(s.clone(), true);
        return true;
    }

    let mut i = 0;
    while i < result.len() -1 {
        if result[i] != c[i] {
            h.insert(s.clone(), false);
            return false;
        }
        i += 1;
    }
    if (result[i] > c[i]) {
        h.insert(s.clone(), false);
        return false
    }
        h.insert(s.clone(), true);
    true
}

fn is_valid(s:&String, c:&Vec<usize>) -> bool {
    //println!("Checking if {} is valid {:?}", s, c);
  // loop, i changes when we encounter a '.'.
  // count how many # in a row till we get to an '.' or the end.
  // Count needs to be less than the or equal to the number.
   let mut i = 0;
   let mut count = 0;
   let mut result: Vec<usize> = Vec::new();
    for x in s.chars() {
        if x == '.' {
            if count > 0 {
                result.push(count);
                count = 0
            }
        }
        if x == '#' {
            count += 1;
        }
    }
    if count > 0 {
        result.push(count);
    }

    if result.len() != c.len() {
        return false;
    }

    let mut i = 0;
    while i < result.len() {
        if result[i] != c[i] {
            return false;
        }
        i += 1;
    }

    true
}

fn col_is_empty(v: &Vec<Vec<char>>, col: usize) -> bool {
    let mut i = 0;
    while i < v.len() {
        if v[i][col] != '.' {
            return false;
        }
        i += 1;
    }
    true
}

fn row_is_empty(v: &Vec<Vec<char>>, row: usize) -> bool {
    let mut i = 0;
    while i < v[row].len() {
        if v[row][i] != '.' {
            return false;
        }
        i += 1;
    }
    true
}

fn next(curr: (usize, usize), c: char, visited: &mut HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    //println!("next called for {}, vistited={:?}", c, visited);

    visited.insert(curr);
    let mut result: Vec<(usize, usize)> = Vec::new();
    if c == '|' {
        if !visited.contains( &((curr.0 -1), curr.1) ) {
            result.push( ((curr.0 -1), curr.1) );
        }
        if !visited.contains( &(curr.0+1, curr.1 ) ) {
            result.push( (curr.0+1, curr.1) );
        }
          
    } 
    if c == '-' {
        if !visited.contains( &(curr.0, curr.1-1) ) {
            result.push( (curr.0, curr.1-1) );
        }
        if !visited.contains( &(curr.0, curr.1+1 ) ) {
            result.push( (curr.0, curr.1+1) );
        }
          
    } 
    if c == 'F'{

        if !visited.contains( &((curr.0 + 1), curr.1) ) {
            result.push( ((curr.0 +1 ), curr.1) );
        }

        if !visited.contains( &((curr.0), curr.1 + 1) ) {
            result.push( ((curr.0), curr.1 +1 ) );
        }
    }
    if c == 'L' {
        if !visited.contains( &(curr.0 -1, curr.1) ) {
            result.push( (curr.0 -1, curr.1) );
        }
        if !visited.contains( &(curr.0, curr.1+1 ) ) {
            result.push( (curr.0, curr.1+1) );
        }
    } 
    if c == 'J' {
        if !visited.contains( &(curr.0 -1, curr.1) ) {
            result.push( (curr.0 -1, curr.1) );
        }
        if !visited.contains( &(curr.0, curr.1-1 ) ) {
            result.push( (curr.0, curr.1-1) );
        }
    } 
    if c == '7' {
        if !visited.contains( &(curr.0, curr.1-1) ) {
            result.push( (curr.0, curr.1-1) );
        }
        if !visited.contains( &(curr.0+1, curr.1 ) ) {
            result.push( (curr.0+1, curr.1) );
        }
    } 
    result
}
