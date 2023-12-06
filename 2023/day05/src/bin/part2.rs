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

let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
//let seeds: Vec<i64> = vec![79, 14, 55, 13]; <Down>
let mut seeds: Vec<(i64, i64)> = Vec::new();
//vec![(202517468, 131640971), (1553776977, 241828580), (1435322022, 100369067), (2019100043, 153706556), (460203450, 84630899), (3766866638, 114261107), (1809826083, 153144153), (2797169753, 177517156), (2494032210, 235157184), (856311572, 542740109)]; 

let groups: Vec<Vec<String>> = group_text("sample.txt");

let mut i = 1;
println!("{:?}", groups[0]);
let s:Vec<&str> = groups[0][0].split(" ").collect();
while i < s.len() {
    let start = s[i].parse().unwrap();
    let end = s[i+1].parse().unwrap();
    seeds.push((start, end));
    i+= 2;
}
println!("{:?}", seeds);

let mut i = 1;
while i < groups.len() {
    let mut map: Vec<(i64, i64, i64)> = Vec::new(); 
    println!("{:?}", groups[i]);
    for line_str in groups[i].iter(){
        let words: Vec<&str> = line_str.split(" ").collect();

        if words.len() == 3 {
            let start: i64 = words[1].parse().unwrap();
            let range:i64 = words[2].parse().unwrap();
            let end:i64 = start + range;
            let destination:i64 = words[0].parse().unwrap(); 
            let offset:i64 = destination - start;

            map.push((start,end,offset));
        }
    }
    maps.push(map);

    i += 1;
}

    seeds.par_iter().for_each(|seed| {
            let mut min_location = std::i64::MAX;

      let mut range = 0;
      println!("Checking seed{:?}", seed);
      while range < seed.1 {
      let mut s = seed.0+range;
      //println!("Checking seed {}", s);
      
      for map in maps.iter() {
          s = convert(&map, s);
      }

      min_location = cmp::min(min_location, s);
      range += 1;
      }



    println!("min location {}", min_location);
    });
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
