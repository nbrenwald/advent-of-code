use std::fs::{read_to_string, File};
use std::io;
use std::io::BufReader;

fn main() -> io::Result<()> {
    //let file = File::open("sample.txt")?;
    //let reader = BufReader::new(file);

    for line in read_to_string("/Users/nbrenwald/private_src/advent-of-code/2024/data/day01-sample.txt").unwrap().lines() {
        println!("{}", line.to_string());
    }
    Ok(())
}