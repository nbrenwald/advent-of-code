use std::cmp::{max, PartialEq};
use std::collections::{BTreeMap, HashMap, HashSet};

pub(crate) fn run(diskMap: &Vec<char>) {

    let mut result:u64 = 0;

    //println!("Disk Map: {:?}", diskMap);


    let mut uncompressed: Vec<String> = Vec::new();
    let mut fileLocations : BTreeMap<u32, (u32, u32)> = BTreeMap::new(); // fileId, start index, length
    let mut freeLocations : BTreeMap<u32, u32> = BTreeMap::new(); //start index, length
    let mut memLocation = 0;
    for (index, c) in  diskMap.iter().enumerate() {

        let mut cNum = c.to_digit(10).unwrap();


        if index %2 == 0 {
            //file
            let fileId = index as u32 /2;
            fileLocations.insert(fileId, (memLocation, cNum));
            while cNum > 0 {
                uncompressed.push(fileId.to_string());
                cNum -= 1;
                memLocation += 1 ;
            }
        }
        else {
            //free space
            freeLocations.insert(memLocation, cNum);
            while cNum > 0 {
                uncompressed.push(".".to_string());
                cNum -= 1;
                memLocation += 1 ;
            }
        }

    }

    //println!("Uncompressed: {:?}", uncompressed);
    //println!("File Locations: {:?}", fileLocations);
   // println!("Free Locations: {:?}", freeLocations);

    let keys:Vec<&u32> = fileLocations.keys().rev().collect();
    for fileId in &keys {
        let length = fileLocations.get(&fileId).unwrap().1;
        let fileIndex = fileLocations.get(&fileId).unwrap().0;

        //Try and find an empty slot
        let mut newIndex = fileIndex;
        for freeIndex in  freeLocations.keys().collect::<Vec<&u32>>() {
            let freeLength = freeLocations.get(&freeIndex).unwrap();
            if freeLength >= &length {
                newIndex = *freeIndex;
                break;
            }
        }

        if newIndex < fileIndex {

            let freeLength = freeLocations.get(&newIndex).unwrap().clone();
            //Move
            let mut i: usize = 0;
            while i < length as usize {
                uncompressed[newIndex as usize + i] = fileId.to_string();
                uncompressed[fileIndex as usize + i] = ".".to_string();
                i += 1;
            }

            // Now update Free Locations
            freeLocations.remove(&newIndex);
            if freeLength > length {
                freeLocations.insert(newIndex + length, freeLength - &length);
            }

        }
    }

    //println!("Unfragmented: {:?}", uncompressed);

    for (i,c) in uncompressed.iter().enumerate() {
        if *c != "." {
            result += i as u64 * c.parse::<u64>().unwrap();
        }
    }


    println!("Result: {}", result);

}