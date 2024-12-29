use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use utils::{direction, group_text, GridPosition, GridPositionAndDirection, Position};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day17.txt";


fn main() -> io::Result<()> {

    let groups: Vec<Vec<String>> = group_text(PATH);


        let re = Regex::new(r"Register.*: (.*)").unwrap();





    let mut a: u64 = re.captures(groups[0][0].as_str()).unwrap().get(1).unwrap().as_str().parse().unwrap();
    let mut b: u64 = re.captures(groups[0][1].as_str()).unwrap().get(1).unwrap().as_str().parse().unwrap();
    let mut c: u64 = re.captures(groups[0][2].as_str()).unwrap().get(1).unwrap().as_str().parse().unwrap();

    let program:Vec<u64> = groups[1][0].chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap() as u64 ).collect::<Vec<u64>>();
    println!("Register A : {:?}", a);
    println!("Register B : {:?}", b);
    println!("Register C : {:?}", c);
    println!("Program : {:?}", program);





    //part_1::run(&mut a, &mut b,&mut c,program);
    part_2::run(&mut a, &mut b,&mut c,program);



    Ok(())
}