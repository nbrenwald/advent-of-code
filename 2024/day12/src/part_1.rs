use std::collections::{HashMap, HashSet};
use utils::Position;

pub(crate) fn run(numbers: &Vec<u64>){

    let mut result:u64 = 0;

    // Read the list into a map.
    let mut numbersMap : HashMap<u64, u64> = HashMap::new();
    for n in numbers {
        let mut count = 0;
        if numbersMap.contains_key(n){
            count = numbersMap[n]
        }
        numbersMap.insert(*n, count + 1);
    }



    for i in 0..75 {
        println!("Numbers Map: {:?}", numbersMap);
        let mut newMap : HashMap<u64, u64> = HashMap::new();
        for key in numbersMap.keys(){
            let count = numbersMap.get(key).unwrap();
            if *key == 0 {
                //becomes 1;
                *newMap.entry(1).or_insert(0) += count;

            }
            else if key.to_string().len() % 2 == 0 {
                // split
                let left = key.to_string()[0..key.to_string().len() / 2].parse::<u64>().unwrap();
                let right = key.to_string()[key.to_string().len() / 2 .. key.to_string().len()].parse::<u64>().unwrap();

                *newMap.entry(left).or_insert(0) += count;
                *newMap.entry(right).or_insert(0) += count;
            }
            else {
                *newMap.entry(key * 2024).or_insert(0) += count;
            }

        }
        numbersMap = newMap;

    }


    for (k,v) in numbersMap {
        result += v;
    }
    println!("Result: {}", result);

}
