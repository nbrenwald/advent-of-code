extern crate regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;

fn main() -> io::Result<()> {
   
    let RED= 12;
    let GREEN=13;
    let BLUE=14;

    let mut total = 0;
    let mut games : HashMap<usize, (usize,usize,usize)> = HashMap::new();
    let re = Regex::new(r"Game (.*):(.*)").unwrap();
    let blueRe = Regex::new(r"(\d+) blue").unwrap();
    let redRe = Regex::new(r"(\d+) red").unwrap();
    let greenRe = Regex::new(r"(\d+) green").unwrap();
    let mut score: usize = 0;
    for line in read_to_string("sample.txt").unwrap().lines() {
        let line_str = line.to_string();
        let caps = re.captures(&line_str).unwrap();
        let index: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let rounds_str = caps.get(2).unwrap().as_str();
        let rounds: Vec<&str> = rounds_str.split(';').collect();
        let mut gamePossible =  true; 
        let mut maxRed = 0;
        let mut maxGreen = 0;
        let mut maxBlue = 0;
        for round in rounds.iter() {
            let blueCaps = blueRe.captures(&round);
            let blue: usize = match blueCaps {
                None => "0", 
                Some(y) => match y.get(1) {
                    None => "0",
                    Some(x) => x.as_str()
                }
            }.parse().unwrap();


            let redCaps = redRe.captures(&round);
            let red: usize = match redCaps {
                None => "0", 
                Some(y) => match y.get(1) {
                    None => "0",
                    Some(x) => x.as_str()
                }
            }.parse().unwrap();

            let greenCaps = greenRe.captures(&round);
            let green: usize = match greenCaps {
                None => "0", 
                Some(y) => match y.get(1) {
                    None => "0",
                    Some(x) => x.as_str()
                }
            }.parse().unwrap();

            if red > maxRed {
                maxRed = red;
            }

            if green > maxGreen {
                maxGreen = green;
            }

            if blue > maxBlue {
                maxBlue = blue;
            }
        }
        let power = maxRed * maxGreen * maxBlue;

        score += power;
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

    }

    println!("score = {}", score);
    Ok(())
}
