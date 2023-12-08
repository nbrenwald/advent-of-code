use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use regex::Regex;

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
    let input:Vec<Vec<String>> = group_text("sample.txt");

    let directions:Vec<char> = input[0][0].chars().collect();
    let mut graph:HashMap<String, (String, String)> = HashMap::new();
    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    for line in input[1].iter() {
        let caps = re.captures(&line).unwrap();
        let id = caps.get(1).unwrap().as_str().to_string();
        let left = caps.get(2).unwrap().as_str().to_string();
        let right = caps.get(3).unwrap().as_str().to_string();
        println!("{} {} {} {}", line, id, left, right);

        graph.insert(id, (left, right));
        let line_str:String = line.to_string();
        let words:Vec<&str> = line_str.split(" ").collect();
    }

    let mut current_place:String = "AAA".to_string();
    let mut steps = 0;
    let mut direction =0;
    while current_place != "ZZZ" {
       let (left, right) = graph.get(&current_place).unwrap();
       if directions[direction] == 'L' {
           current_place = left.to_string();
       }
       else {
           current_place = right.to_string();
       }
       steps += 1;
       if direction == directions.len() -1 {
           direction = 0;
       }
       else {direction += 1;}
    }
    print!("{}", steps);


    Ok(())
}

