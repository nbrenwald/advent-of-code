use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use rayon::prelude::*;



fn main() -> io::Result<()> {

    let mut hands: Vec<(String, usize)> = Vec::new();
    for line in read_to_string("sample.txt").unwrap().lines() {
        //println!("{}", line);
        let line_str:String = line.to_string();
        let words:Vec<&str> = line_str.split(" ").collect();
        let hand: &str = words[0];
        let bid:usize = words[1].parse().unwrap();
        hands.push((hand.to_string(),bid));
    }


    let mut result = classify(&hands);
    //println!("{:?}", result);

    result.sort_by(|a,b| 
                   if a.0 == b.0 {b.1.cmp(&a.1)}
                   else {a.0.cmp(&b.0)}
                   );

    //println!("{:?}", result);
   
    let mut total = 0;
    for (rank , score) in result.iter().enumerate() {
       //println!("{} {}", rank, score.2);
       println!("{}", score.2);
       total += (rank + 1) * score.2;
    }
    println!("{}", total);
    Ok(())
}

fn classify(v: &Vec<(String, usize)>) -> Vec<(usize, String, usize)> {
    let mut result: Vec<(usize, String, usize)> = Vec::new();
    for (s,b) in v.iter() {
        let mut d:HashMap<char, usize> = HashMap::new();
        let mut jCount = 0;
       for c in s.chars() {
           if c == 'J' {
        jCount += 1;
           }
           else {
          *d.entry(c).or_insert(0) += 1;
           }
       }
       let mut score: usize = 0;
       let mut counts:Vec<&usize> = d.values().collect();
       counts.sort();
       counts.reverse();
       //println!("{:?} jCount={}", counts, jCount);
       
       if counts.len() == 0 {
           score == 6; // only jokers - can make 5 of a kind
       }
      else if *counts[0] == 5 { //5 of a kind
           score = 6;
       }
       else if *counts[0] == 4 {// 4 of a kinf
           score = 5;
       }
       else if *counts[0] == 3 { //3 of a kind
           if counts.len() >1 && *counts[1] == 2 { //full house
           score =4;
           } else {
               score = 3; //3 of a kind normal
           }
       }
       else if *counts[0] == 2 { //2 of a kind
           if counts.len() >1 && *counts[1] == 2 {  //2 pairs
               score = 2;
           } else {
               score = 1; //1 pair
           }
       }
       else {
           score = 0; //nothing mtch
       }

       while jCount > 0 {
 
           if score == 5 { //4 of a kind goes to 5 of a kind
               score = 6;
           }
           if score == 4 { 
               score = 5//full house foes to 4 of a kind
           }
           if score == 3 {
               score = 5;//3 of a kind goes to 4 of a kind
           }
           if score == 2 {
              score = 4; //2 pairs goes to afull house
           }
           if score == 1 { //1 pair goes to a 3 of a kind
               score = 3;
           }
           if score == 0 { //no pairs goes to a pair
               score = 1;
           }
           jCount -= 1;
       }
       let mut codedS : String = s.to_string().replace('K', "B");
       codedS = codedS.replace('Q', "C");
       codedS = codedS.replace('J', "O");
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

