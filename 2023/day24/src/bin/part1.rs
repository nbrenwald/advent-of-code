use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use regex::Regex;


#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug, PartialEq, Clone)]
struct Segment {
    start: Point,
    end: Point
}

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


fn main() -> io::Result<()> {
    let mut result = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    
    let groups = group_text("sample.txt");
    let re = Regex::new(r"(.*)~(.*)").unwrap();
    let mut id = 0;
    let mut hail_stones:Vec<Segment> = Vec::new();
    for row in groups[0].iter() {
        let point_and_velocity:Vec<&str> = row.split("@").collect();
        println!("{:?}", point_and_velocity);

        let coord:Vec<&str> = point_and_velocity[0].split(",").collect();
        let velocity:Vec<&str> = point_and_velocity[1].split(",").collect();
        let startPoint = Point {x:coord[0].trim().parse().unwrap(), y:coord[1].trim().parse().unwrap(), z:coord[2].trim().parse().unwrap() };
        let velocityPoint = Point {x:velocity[0].trim().parse().unwrap(), y:velocity[1].trim().parse().unwrap(), z:velocity[2].trim().parse().unwrap() };
        let endPoint = Point{x: startPoint.x+velocityPoint.x, y: startPoint.y+velocityPoint.y, z: startPoint.z+velocityPoint.z};
        let segment = Segment{start:startPoint, end:endPoint};
        hail_stones.push(segment);
    }

    println!("{:?}", hail_stones);

    let mut x = 0;
    while x < hail_stones.len() -1 {
        let mut y = x + 1;
        while y < hail_stones.len() {
            if compare(&hail_stones[x], &hail_stones[y], 200000000000000f64, 400000000000000f64) {
                result += 1;
            }
            y += 1;
        }
        x+=1
    }
    println!("result {}", result);

    Ok(())
}

fn compare(s1:&Segment, s2:&Segment, min:f64, max:f64)  -> bool{
    /*double det = A1 * B2 - A2 * B1
if (det == 0) {
  //Lines are parallel
} else {
  double x = (B2 * C1 - B1 * C2) / det
  double y = (A1 * C2 - A2 * C1) / det
}*/

/*
 * A = y2-y1
B = x1-x2
C = Ax1+By1
*/
    println!("Coparing {:?} {:?}", s1, s2);
    let A1 = s1.end.y - s1.start.y;
    let A2 = s2.end.y - s2.start.y;
    let B1 = s1.start.x - s1.end.x;
    let B2 = s2.start.x - s2.end.x;
    let C1 = A1*s1.start.x + B1*s1.start.y;
    let C2 = A2*s2.start.x + B2*s2.start.y;
    println!("{} {} {}", A1, B1, C1);
    println!("{} {} {}", A2, B2, C2);

    let det = A1 * B2 - A2 * B1;
    println!("{}", det);

    if det == 0f64 {
        println!("Parallel");
        return false;
    }
    let x = (B2 * C1 - B1 * C2) / det;
    let y = (A1 * C2 - A2 * C1) / det;
    println!("Crosses at x={} y={}", x, y);

    if x < min || x > max || y < min || y> max {
        return false;
        println!("out of bounds");
    }
    if A1 > 0f64 && y < s1.end.y {
        return false;
        println!("Crosses in past");
    }
    if A1 < 0f64 && y > s1.end.y {
        return false;
        println!("Crosses in past");
    }
    if A2 > 0f64 && y < s2.end.y {
        return false;
        println!("Crosses in past");
    }
    if A2 < 0f64 && y > s2.end.y {
        return false;
        println!("Crosses in past");
    }
    true

}


