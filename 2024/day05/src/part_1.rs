use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io;

fn group_text(file: &str) -> Vec<Vec<String>> {
    let mut group:Vec<String> = Vec::new();
    let mut text:Vec<Vec<String>> = Vec::new();
    for line in read_to_string(file).unwrap().lines() {
        if line == "" {
            if group.len() != 0 {
                text.push(group);
                group = Vec::new();
            }
        }
        else {
            group.push(line.to_string());
        }
    }
    if group.len() != 0 {
        text.push(group);
    }
    text
}



pub(crate) fn part1(path: &str) -> io::Result<()> {

    let mut result:i32 = 0;


    let vec = group_text(path);

    let rules = vec.get(0).unwrap();
    println!("{:?}", rules);

    let pages = vec.get(1).unwrap();
    println!("{:?}", pages);

    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for rule in rules {
        let split: Vec<&str> = rule.as_str().split("|").collect();
        let before: i32  = split.get(0).unwrap().parse().unwrap();
        let after: i32  = split.get(1).unwrap().parse().unwrap();

        if !map.contains_key(&before) {
            map.insert(before, HashSet::new() );
        }
        let mut v = map.get_mut(&before).unwrap();
        v.insert(after);


    }

    println!("{:?}", map);


    for page in pages {
        let mut set: HashSet<i32> = HashSet::new();
        let mut valid = true;
        let pageVec = page.as_str().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        for  pageNumber in &pageVec {

            println!("{}", pageNumber);
            if map.contains_key(&pageNumber) {
                println!("Checking rules for {}", pageNumber);
                if !set.is_disjoint(map.get(&pageNumber).unwrap()) {
                    valid = false;
                    break;
                }


            }
            set.insert(*pageNumber);

        }
        if valid {
            let i : usize = pageVec.len()/2;
            result = result +  pageVec[i]
        }

    }


    println!("result = {:?}", result);

    Ok(())
}
