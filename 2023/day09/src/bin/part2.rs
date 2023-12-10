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
    for line in read_to_string("sample.txt").unwrap().lines() {

        let mut next: Vec<i64> = Vec::new();
        let line_string = line.to_string();
        let mut v: Vec<i64> = Vec::new();
        for value_str in line_string.split(" "){
          let value:i64 = value_str.parse().unwrap();
          v.push(value);
        }
        println!("v = {:?}", v);

        let mut nv:Vec<i64> = Vec::new();
        while  ! all_zero(&v) {
          nv = Vec::new();
          next.push(v[0]);
          let mut i = 1;
          while i < v.len() {
              let diff = v[i] - v[i-1];
              nv.push(diff);
              i += 1;
          }
          println!("{:?}", nv);
          v = nv;
        }
        println!("next {:?}", next);
        let mut result = 0;
        let mut x = next.len();
        while x > 0 {
            x -= 1;
            println!("next[x]={} result={} x={}", next[x], result, x);
           result = next[x] - result;
        }
        println!("result {}", result);
        total += result;
    }
    println!("total {}", total);
    Ok(())
}

fn all_zero(v:&Vec<i64>) -> bool {
    for x in v.iter() {
        if *x != 0i64 {
            return false;
        }
    }
    true
}

 
