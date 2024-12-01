use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;


pub(crate) fn part2(path: &str) -> io::Result<()> {

    let mut v: Vec<i32> = Vec::new();
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for line in read_to_string(path)?.lines() {
        println!("{}", line.to_string());

        let lists: Vec<_> = line.split_whitespace().collect();
        v.push(lists[0].parse::<i32>().unwrap());

        let y = lists[1].parse::<i32>().unwrap();

        if counts.contains_key(&y) {
            let y_count = counts.get(&y).unwrap();
            counts.insert(y, y_count + 1);
        }
        else {
            counts.insert(y, 1);
        }
    }

    let mut result = 0;
    for i in v.iter() {

       if counts.contains_key(&i){
           result += counts.get(&i).unwrap() * i;
       }
    }

    println!("Result: {}", result);
    Ok(())
}