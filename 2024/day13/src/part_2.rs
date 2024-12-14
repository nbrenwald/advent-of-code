use std::collections::{HashMap, HashSet, VecDeque};
use utils::{GridPosition, Position};
use crate::Equation;

fn solve(x: i64, y: i64, ax: i64, ay: i64, bx: i64, by: i64) -> Option<i64> {
    let ma = [[ax, bx], [ay, by]];
    let det = ma[0][0] * ma[1][1] - ma[0][1] * ma[1][0];
    let inv = [[ma[1][1], -ma[0][1]], [-ma[1][0], ma[0][0]]];
    let numa = inv[0][0] * x + inv[0][1] * y;
    let numb = inv[1][0] * x + inv[1][1] * y;
    if numa % det == 0 && numb % det == 0 {
        Some(3 * numa / det + numb / det)
    } else {
        None
    }
}
pub(crate) fn run(equations: &Vec<Equation>) -> i64{
    let mut result:i64 = 0;

    for e in equations {


        let mut cost:u64 = u64::MAX;

        let r = solve(e.target.col as i64+ 10000000000000 as i64, e.target.row as i64+ 10000000000000 as i64, e.button_a.col as i64, e.button_a.row as i64, e.button_b.col as i64, e.button_b.row as i64);

        if r.is_some() {
            result += r.unwrap();
        }
        /*for i in 0 .. 10000000000000 {
            for j in 0 .. 10000000000000 {
                if (i*e.button_a.col as u64 + j*e.button_b.col as u64 > e.target.col as u64+ 10000000000000 as u64) ||
                    (i*e.button_a.row as u64+ j*e.button_b.row  as u64 > e.target.row as u64+ 10000000000000 as u64) {

                    break;
                }
                else if (i*e.button_a.col as u64 + j*e.button_b.col as u64 == e.target.col as u64+ 10000000000000 as u64) &&
                    (i*e.button_a.row as u64+ j*e.button_b.row  as u64 == e.target.row as u64+ 10000000000000 as u64) {

                    println!("Solved at x={} y={}", i, j);
                    cost = u64::min (cost, 3*i as u64 + j as u64);
                    //println!("cost={}", cost);
                }
            }
        }*/


    }


    println!("Result is {}", result);
    return result;
}
