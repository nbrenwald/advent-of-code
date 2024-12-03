use std::fs::{read_to_string, File};
use std::io;
use regex::Regex;

pub(crate) fn part1(path: &str) -> io::Result<()> {

    let mut result:i64 = 0;
    for line in read_to_string(path)?.lines() {
        println!("Searching {}", line);

        let re = Regex::new(r"mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)").unwrap();
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        println!("{:?}", matches);

        for capture in matches {
            let re = Regex::new(r"[0-9]+").unwrap();
            let matches: Vec<_> = re.find_iter(capture).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
            println!("{:?}", matches);

            result += matches[0] * matches[1];

        }
    }
    println!("Final result: {}", result);
    //188534791 too high
Ok(())
}