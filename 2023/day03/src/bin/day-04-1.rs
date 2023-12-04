use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashSet;

fn main() -> io::Result<()> {

    let re = Regex::new(r"Card.*(\d+):(.*)\|(.*)").unwrap();
    let mut total:usize = 0;

    for line in read_to_string("data/day-04/sample.txt").unwrap().lines() {
        let line_str = line.to_string();
        println!("{}", line_str);
        let caps = re.captures(&line_str).unwrap();
        let gameId: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let winning_numbers = caps.get(2).unwrap().as_str();
        let mut winning_numbers_set =  HashSet::new();
        let winning_numbers_vector: Vec<&str> =  winning_numbers.split(" ").collect();
        for s in winning_numbers_vector {
            let x = s.trim();
            if x != "" {
                winning_numbers_set.insert(x);
            }
        }
        let my_numbers = caps.get(3).unwrap().as_str();
        let my_numbers_vector: Vec<&str> = my_numbers.split(' ').collect();

        let mut game_total = 0;
       for i in my_numbers_vector.iter() {
           println!("Checking {}", i);
         if winning_numbers_set.contains(i) {
             println!("found");
            if game_total == 0 {
                game_total = 1;
            }
            else {
                game_total = game_total * 2;
            }
         }


       }
       total += game_total;

        println!("Game ID:{} , total={}", gameId, game_total);
    }
    println!("Total: {}", total);
Ok(())
}
