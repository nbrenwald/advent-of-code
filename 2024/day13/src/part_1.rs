use std::collections::{HashMap, HashSet, VecDeque};
use utils::{GridPosition, Position};
use crate::Equation;

pub(crate) fn run(equations: &Vec<Equation>) -> i32{
    let mut result = 0;

    for e in equations {

        let mut cost = i32::MAX;

        for i in 0 .. 100 {
            for j in 0 .. 100 {
                if (i*e.button_a.col + j*e.button_b.col == e.target.col) &&
                    (i*e.button_a.row + j*e.button_b.row == e.target.row) {

                    println!("Solved at x={} y={}", i, j);
                    cost = i32::min (cost, 3*i as i32 + j as i32);
                    //println!("cost={}", cost);
                }
            }
        }
        if cost < i32::MAX {
            println!("Solved , cost={}", cost);
            result += cost;
        }

    }


    println!("Result is {}", result);
    return result;
}
