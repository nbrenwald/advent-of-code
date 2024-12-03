use std::fs::{read_to_string, File};
use std::io;
use regex::Regex;

pub(crate) fn part2(path: &str) -> io::Result<()> {

    let mut result:i64 = 0;
    let mut input: String = String::new();
    for line in read_to_string(path)?.lines() {
        println!("Searching {}", line);
        input = format!("{}{}", input, line);
    }


        let re = Regex::new(r"do\(\)").unwrap();
        //let dos: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        let does: Vec<_> = input.split("do()").collect();
        println!("{:?}", does);

        for d in does {

            let re = Regex::new(r".*^[don't\(\)]").unwrap();
            //let donts: Vec<_> = re.find_iter(d).map(|m| m.as_str()).collect();
            let donts: Vec<_> = d.split("don't()").collect();
            println!("{:?}", donts);

            result += calc(donts[0]);

        }

    println!("Final result: {}", result);
    //188534791 too high
    //79967531 too high
    Ok(())
}


fn calc(input: &str) -> i64 {
    let re = Regex::new(r"mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    println!("{:?}", matches);

    let mut result:i64 = 0;

    for capture in matches {
        let re = Regex::new(r"[0-9]+").unwrap();
        let matches: Vec<_> = re.find_iter(capture).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
        println!("{:?}", matches);

        result += matches[0] * matches[1];

    }
    result
}