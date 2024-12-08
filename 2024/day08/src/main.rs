use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use utils::{Position, Segment};

mod part_2;
mod part_1;

const PATH: &str = "/Users/nbrenwald/private_src/advent-of-code/2024/data/day08.txt";

fn main() -> io::Result<()> {

    let maxY = read_to_string(PATH).unwrap().lines().collect::<Vec<_>>().len();
    let maxX = read_to_string(PATH).unwrap().lines().collect::<Vec<_>>()[0].len();
    let map = parse();
    //part_1::run(map, maxY as i32, maxX as i32 );
    part_2::run(map, maxY as i32, maxX as i32 );

    Ok(())
}

fn parse () -> HashMap<char, Vec<Segment>> {
    let mut positions: HashMap<char, Vec<Position>> = HashMap::new();
    for (rNumber,line) in read_to_string(PATH).unwrap().lines().rev().enumerate() {
        for (cNumber, c) in line.chars().enumerate() {
            if c != '.' {
                let p = Position { x: cNumber as i32, y: rNumber as i32 };
                positions.entry(c).or_insert(Vec::new()).push(p);
            }
        }
    }
    println!("{:?}", positions);

    let mut segmentMap: HashMap<char, Vec<Segment>> = HashMap::new();
    for antenna in positions.keys(){
        let mut segments : Vec<Segment> = Vec::new();
        let antennaPositions = positions.get(antenna).unwrap();
        let mut i = 0;
        while i < antennaPositions.len() -1 {
            let mut j = i + 1;
            while j < antennaPositions.len() {
                let s:Segment = Segment { first: antennaPositions[i], second: antennaPositions[j] };
                segments.push(s);
                j +=1;
            }
            i+=1;
        }
        segmentMap.insert(*antenna, segments);
    }

    println!("{:?}", segmentMap);
    segmentMap
}



