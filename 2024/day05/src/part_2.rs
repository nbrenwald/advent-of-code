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


pub(crate) fn part2(path: &str) -> io::Result<()> {

    let mut result:i32 = 0;

    let vec = group_text(path);

    let rules = vec.get(0).unwrap();
    println!("{:?}", rules);

    let pages = vec.get(1).unwrap();
    println!("{:?}", pages);

    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut map2: HashMap<i32, HashSet<i32>> = HashMap::new();

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
        let mut pageVec = page.as_str().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        for  pageNumber in &pageVec {

            println!("{}", pageNumber);
            if map.contains_key(&pageNumber) {
                println!("Checking rules for {}", pageNumber);
                if !set.is_disjoint(map.get(&pageNumber).unwrap()) {
                    valid = false;
                    repair(&mut pageVec, &map);
                    break;
                }
            }
            set.insert(*pageNumber);
        }
        if !valid {
            let i : usize = pageVec.len()/2;
            result = result +  pageVec[i]
        }
    }

    println!("result = {:?}", result);

    Ok(())
}

fn repair(p0: &mut Vec<i32>, map: &HashMap<i32, HashSet<i32>>) {

    let mut i = 0;
    while i < p0.len() {
        println!("checking {} in {:?}", p0[i], p0);

        let mut j = i+1;
        while j < p0.len() {
            println!("checking {} against {}", p0[i], p0[j]);
            if map.contains_key(&p0[j]) {
                if map.get(&p0[j]).unwrap().contains(&p0[i]) {
                    println!("Found a swap {} {}", p0[i], p0[j]);
                    let tmp = p0[i];
                    p0[i] = p0[j];
                    p0[j] = tmp;
                }
            }
            j += 1;
        }
        i += 1;
    }

}
