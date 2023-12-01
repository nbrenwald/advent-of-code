use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fs::read_to_string;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("sample.txt")?;
    let reader = BufReader::new(file);
    let mut numbersAsStrings = HashMap::new();
    numbersAsStrings.insert("one","1e");
    numbersAsStrings.insert("two","2o");
    numbersAsStrings.insert("three","3e");
    numbersAsStrings.insert("four","4r");
    numbersAsStrings.insert("five","5e");
    numbersAsStrings.insert("six","6x");
    numbersAsStrings.insert("seven","7n");
    numbersAsStrings.insert("eight","8t");
    numbersAsStrings.insert("nine","9e");

    let mut total = 0;
    for line in read_to_string("sample.txt").unwrap().lines() {
        let mut number = 0;
        let mut line_str = line.to_string();

        for (string, digit) in &numbersAsStrings {
            let c = format!("{string}{digit}");
            line_str = line_str.replace(string, &c)
}


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
        println!("{} {} {}", number,  line.to_string(), line_str);

    }
    println!("total={}", total);

    Ok(())
}
