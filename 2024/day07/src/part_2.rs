use std::cmp::{max, PartialEq};
use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io;
use utils::direction;

#[derive(Debug)]
struct Calculation {
    target:i64,
    numbers:Vec<i64>
}


pub(crate) fn parse(path: &str) -> io::Result<()> {

    let mut result = 0;
    let mut calcs: Vec<Calculation> = Vec::new();

    for (rNumber,line) in read_to_string(path).unwrap().lines().enumerate() {
        let split: Vec<&str> = line.split(':').collect();

        let numbers : Vec<i64> = split[1].split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        calcs.push(Calculation {target:split[0].parse::<i64>().unwrap(),numbers:numbers});
    }

    println!("{:?}", calcs);

    println!("result = {:?}", result);

    for calc in &calcs {
        let r =  backtrack(calc.target, calc.numbers[0], 1, &calc.numbers);
        println!("{:?}", r);
        result += r;
    }
    println!("result = {:?}", result);
    Ok(())
}

fn backtrack(target: i64, current: i64, index: usize, numbers: &Vec<i64>) -> i64 {
    println!("Checking {} {}", target, current);
    if target == current {
        println!("Match");
        return target;
    }

    if current > target {
        return 0;
    }

    if index == numbers.len() {
        return 0;
    }

    let add = backtrack(target, current + numbers[index], index+1, numbers);
    if add > 0 {return add};

    let mult = backtrack(target, current * numbers[index], index+1, numbers);
    if mult > 0 {return mult};

    let con = format!("{}{}", current, numbers[index]).parse::<i64>().unwrap();
    let concat = backtrack(target, con, index+1, numbers);
    if concat > 0 {return concat};

    0
}
