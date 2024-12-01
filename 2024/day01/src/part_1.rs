use std::fs::{read_to_string, File};
use std::io;


pub(crate) fn part1(path: &str) -> io::Result<()> {

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in read_to_string(path)?.lines() {

        let lists: Vec<_> = line.split_whitespace().collect();
        v1.push(lists[0].parse::<i32>().unwrap());
        v2.push(lists[1].parse::<i32>().unwrap());
    }

    v1.sort();
    v2.sort();

    let mut result = 0;
    let mut i = 0;
    while i < v1.len() {
        let x = v1.get(i).unwrap();
        let y = v2.get(i).unwrap();
        result += (x - y).abs();
        i += 1;
    }

    println!("Result: {}", result);
    Ok(())
}