use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use rayon::prelude::*;



fn main() -> io::Result<()> {

    let mut hands: Vec<(String, usize)> = Vec::new();
    for line in read_to_string("sample.txt").unwrap().lines() {
        println!("{}", line);
        let line_str:String = line.to_string();
        let words:Vec<&str> = line_str.split(" ").collect();
        let hand: &str = words[0];
        let bid:usize = words[1].parse().unwrap();
        hands.push((hand.to_string(),bid));
    }


    let mut result = classify(&hands);
    println!("{:?}", result);

    result.sort_by(|a,b| 
                   if a.0 == b.0 {b.1.cmp(&a.1)}
                   else {a.0.cmp(&b.0)}
                   );

    println!("{:?}", result);
   
    let mut total = 0;
    for (rank , score) in result.iter().enumerate() {
       println!("{} {}", rank, score.2);
       total += (rank + 1) * score.2;
    }
    println!("{}", total);
    Ok(())
}

fn classify(v: &Vec<(String, usize)>) -> Vec<(usize, String, usize)> {
    let mut result: Vec<(usize, String, usize)> = Vec::new();
    for (s,b) in v.iter() {
        let mut d:HashMap<char, usize> = HashMap::new();
        let containsJ = false;
       for c in s.chars() {
           if c == 'J' {
        
           }
          *d.entry(c).or_insert(0) += 1;
       }
       let mut score: usize = 0;
       let mut counts:Vec<&usize> = d.values().collect();
       counts.sort();
       counts.reverse();
       println!("{:?}", counts);
       if *counts[0] == 5 {
           score = 6;
       }
       else if *counts[0] == 4 {
           score = 5;
       }
       else if *counts[0] == 3 {
           if *counts[1] == 2 {
           score =4;
           } else {
               score = 3;
           }
       }
       else if *counts[0] == 2 {
           if *counts[1] == 2 { 
               score = 2;
           } else {
               score = 1;
           }
       }
       else {
           score = 0;
       }
       let mut codedS : String = s.to_string().replace('K', "B");
       codedS = codedS.replace('Q', "C");
       codedS = codedS.replace('J', "D");
       codedS = codedS.replace('T', "E");
       codedS = codedS.replace('9', "F");
       codedS = codedS.replace('8', "G");
       codedS = codedS.replace('7', "H");
       codedS = codedS.replace('6', "I");
       codedS = codedS.replace('5', "J");
       codedS = codedS.replace('4', "K");
       codedS = codedS.replace('3', "L");
       codedS = codedS.replace('2', "M");
       result.push((score, codedS, b.clone()));
    }
    result
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

fn convert(vec: &Vec<(i64,i64,i64)>, i: i64) -> i64 {

    for x in vec.iter() {
        if i>= x.0 && i< x.1 {
            return i+x.2
        } 
    }
    i
}
