use std::cmp::{max, PartialEq};
use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io;
use utils::{direction, onLine, on_grid_line, Position, Segment};

#[derive(Debug)]
struct Calculation {
    target:i64,
    numbers:Vec<i64>
}

pub(crate) fn run(segmentMap: HashMap<char, Vec<Segment>>, xMax: i32, yMax: i32) {

    let mut result = 0;
    let mut antiNodes :HashSet<Position> = HashSet::new();

    on_grid_line(Position { x: 2, y: 8 }, &Segment{first:Position{x:5, y:9}, second:Position{x:8, y:10}});

    for x in 0..xMax{
        for y in 0..yMax{
            println!("Testing {},{}", x, y);
            for segmentVec in segmentMap.values(){
                for segment in segmentVec {
                    if onLine(Position { x: x, y: y }, &segment) {
                        println!("Online ({},{}) is on {:?}",x,y,segment );
                        antiNodes.insert(Position{x,y});
                        result +=1;
                    }
                }
            }
        }
    }
    println!("Result: {}", result);
    println!("Result: {}", antiNodes.len());
    println!("Antinodes: {:?}", antiNodes);

}