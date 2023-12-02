extern crate regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;

fn main() -> io::Result<()> {
    
    let mut total = 0;
    let mut games = HashMap::new();
    let re = Regex::new(r"Game ():.*").unwrap();
    for line in read_to_string("test_sample.txt").unwrap().lines() {
        let line_str = line.to_string();
        let caps = re.captures(&line_str).unwrap();
        let index = caps.get(1).unwrap().as_str();
        //let mut number = 0;
        //for i in line_str.chars() {
        //    if i.is_numeric() {
        //        number += i.to_digit(10).unwrap_or(0) * 10;
        //        break;
        //    }
        //}


        //for i in line_str.chars().rev() {
        //    if i.is_numeric() {
        //        number += i.to_digit(10).unwrap_or(0);
        //        break;
        //    }
        //}

        //total += number;
        //println!("{} {}", number,  line_str);

    println!("index={}, line={}", index, line_str);
    }

    Ok(())
}
