use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use rayon::prelude::*;

fn main() -> io::Result<()> {
// pares each HashMap
// calculate a start, end (start plus range), offset (what we need to add on).
// for each seed, try and find in each map, or return itddlf by looking for a range it is within.
// 50 98 2 => 98,100 (start+range), -48 (start-destination)

    //let races: Vec<(usize, usize)> = vec![(71530,940200)];
    let races: Vec<(usize, usize)> = vec![(45977295, 305106211101695)];
    //naive approach - check every possibility. 0 and 7, 1 and 6, ... 7 and 0. For each
    //possibility, count if it beats the current record and multiply an accumlator.
    //
    let mut result:usize = 1;

    for race in races.iter() {

        let mut recordBeater:usize = 0;

        let time = race.0;
        let record = race.1;

        let mut hold_time = 0;
        while hold_time <= time {
            let remaining_time = time - hold_time;
            let distance = remaining_time * hold_time;
            if distance > record {
                recordBeater += 1;
            }
            hold_time += 1;
        }


        result *= recordBeater;
    }

    println!("Result {}", result);
    Ok(())
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
