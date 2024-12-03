use std::fs::{read_to_string, File};
use std::io;


pub(crate) fn part2(path: &str) -> io::Result<()> {



    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in read_to_string(path)?.lines() {
        let mut levels: Vec<i32> = Vec::new();
        println!("{}", line.to_string());
        let lists: Vec<_> = line.split_whitespace().collect();
        let levels = lists.iter().map(|x|  x.parse::<i32>().unwrap()).collect();
        reports.push(levels);
    }

    println!("{:?}", reports);

    let mut result = 0;

    for report in &reports {
        if is_valid(report) {
            println!("SAFE");
            result += 1;
        }
        else {
            let mut i = 0;
            while i < report.len() {
                let mut tmp = report.clone();
                tmp.remove(i);
                if is_valid(&tmp) {
                    println!("Valid with level removed");
                    result +=1;
                    break;
                }
                i = i+1;
            }
        }
    }
    println!("{}", result);
    Ok(())
}


fn is_valid(report: &Vec<i32>) -> bool {
    let is_ascending = report.get(0) <= report.get(1);

    let mut i = 1;
    let mut prev = report.get(0).unwrap();
    let mut valid = false;
    while i < report.len() {
        let diff = report.get(i).unwrap() - prev;
        if is_ascending && (diff < 1 || diff > 3){
            break;
        }
        else if !is_ascending && (diff > -1 || diff < -3) {
            break;
        }
        prev = report.get(i).unwrap();
        i += 1;
        if i == report.len() {valid = true;}
    }

    return valid;
}