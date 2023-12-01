use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fs::read_to_string;

fn main() -> io::Result<()> {
    let file = File::open("sample.txt")?;
    let reader = BufReader::new(file);
    
    let mut total = 0;
    for line in read_to_string("sample.txt").unwrap().lines() {
        let mut number = 0;
        let line_str = line.to_string();
        for i in line_str.chars() {
            if i.is_numeric() {
                number += i.to_digit(10).unwrap_or(0) * 10;
                break;
            }
        }


        for i in line_str.chars().rev() {
            if i.is_numeric() {
                number += i.to_digit(10).unwrap_or(0);
                break;
            }
        }

        total += number;
        println!("{} {}", number,  line_str);

    }
    println!("total={}", total);

    Ok(())
}
