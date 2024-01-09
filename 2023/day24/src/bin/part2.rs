use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;
use regex::Regex;
use z3::ast;
use z3::ast::Ast;
use itertools::Itertools;
//use itertools::structs::Combinations;


#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug, PartialEq, Clone)]
struct Segment {
    start: Point,
    end: Point
}

type ParsedItem = (Vec<i64>, Vec<i64>);
type Parsed = Vec<ParsedItem>;

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
    let mut hail_stones:Vec<ParsedItem> = Vec::new();
    for row in groups[0].iter() {
        let point_and_velocity:Vec<&str> = row.split("@").collect();
        println!("{:?}", point_and_velocity);

        let coord:Vec<&str> = point_and_velocity[0].split(",").collect();
        let start_vec:Vec<i64> = coord.iter().map(|x| x.trim().parse().unwrap()).collect();
        let velocity:Vec<&str> = point_and_velocity[1].split(",").collect();
        let velocity_vec:Vec<i64> = velocity.iter().map(|x| x.trim().parse().unwrap()).collect();
        let startPoint = Point {x:coord[0].trim().parse().unwrap(), y:coord[1].trim().parse().unwrap(), z:coord[2].trim().parse().unwrap() };
        let velocityPoint = Point {x:velocity[0].trim().parse().unwrap(), y:velocity[1].trim().parse().unwrap(), z:velocity[2].trim().parse().unwrap() };
        let endPoint = Point{x: startPoint.x+velocityPoint.x, y: startPoint.y+velocityPoint.y, z: startPoint.z+velocityPoint.z};
        let segment = Segment{start:startPoint, end:endPoint};
        hail_stones.push((start_vec, velocity_vec));
    }

    println!("{:?}", hail_stones);
    println!("{}", part2(&hail_stones));
    println!("result {}", result);

    Ok(())
}

//Solved using Z3 - taken from a rust solution i saw online.
fn part2(data: &Parsed) -> i64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let px = z3::ast::Int::new_const(&ctx, "px");
    let py = z3::ast::Int::new_const(&ctx, "py");
    let pz = z3::ast::Int::new_const(&ctx, "pz");
    let pvx = z3::ast::Int::new_const(&ctx, "pvx");
    let pvy = z3::ast::Int::new_const(&ctx, "pvy");
    let pvz = z3::ast::Int::new_const(&ctx, "pvz");
    let mut ts = vec![];
    for (i, (p, v)) in data.iter().enumerate() {
        let x = z3::ast::Int::from_i64(&ctx, p[0]);
        let y = z3::ast::Int::from_i64(&ctx, p[1]);
        let z = z3::ast::Int::from_i64(&ctx, p[2]);
        let vx = z3::ast::Int::from_i64(&ctx, v[0]);
        let vy = z3::ast::Int::from_i64(&ctx, v[1]);
        let vz = z3::ast::Int::from_i64(&ctx, v[2]);
        let t = z3::ast::Int::new_const(&ctx, format!("t{}", i));
        let a = z3::ast::Int::add(&ctx, &[&x, &z3::ast::Int::mul(&ctx, &[&t, &vx])]);
        let b = z3::ast::Int::add(&ctx, &[&y, &z3::ast::Int::mul(&ctx, &[&t, &vy])]);
        let c = z3::ast::Int::add(&ctx, &[&z, &z3::ast::Int::mul(&ctx, &[&t, &vz])]);
        let d = z3::ast::Int::add(&ctx, &[&px, &z3::ast::Int::mul(&ctx, &[&t, &pvx])]);
        let e = z3::ast::Int::add(&ctx, &[&py, &z3::ast::Int::mul(&ctx, &[&t, &pvy])]);
        let f = z3::ast::Int::add(&ctx, &[&pz, &z3::ast::Int::mul(&ctx, &[&t, &pvz])]);
        solver.assert(&a._eq(&d));
        solver.assert(&b._eq(&e));
        solver.assert(&c._eq(&f));
        ts.push(t);
    }
    let f = z3::ast::Bool::from_bool(&ctx, false);
    for t in ts.iter().combinations(2) {
        solver.assert(&t[0]._eq(t[1])._eq(&f));
    }
    solver.check();
    let m = solver.get_model().unwrap();
    let px = m.get_const_interp(&px).unwrap().as_i64().unwrap();
    let py = m.get_const_interp(&py).unwrap().as_i64().unwrap();
    let pz = m.get_const_interp(&pz).unwrap().as_i64().unwrap();
    px + py + pz
}
