use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use utils::{GridPosition, Position};
use crate::Robot;

pub(crate) fn run(robots: &mut Vec<Robot>, max_row: i32, max_col:i32) -> i32{
    let mut result = 0;
    let mut map: [[char; 101]; 103] =  [['.'; 101]; 103];
     //println!("{:?}", map);


        for i in 1..1000000 {
            let mut map: [[char; 101]; 103] = [['.'; 101]; 103];
            for robot in &mut *robots {
                let mut row: i32 = robot.pos.row as i32;
                let mut col: i32 = robot.pos.col as i32;
                /*for i in 0..seconds {*/
                row = row + robot.rowVelocity;
                col = col + robot.colVelocity;

                if row < 0 {
                    row = max_row + row;
                }
                if row >= max_row {
                    row = row - max_row;
                }
                if col < 0 {
                    col = max_col + col;
                }
                if col >= max_col {
                    col = col - max_col;
                }
                /* }*/
                robot.pos.row = row as usize;
                robot.pos.col = col as usize;
                map[row as usize][col as usize] = '#';
            }

            //println!("");
            for r in map.iter_mut() {
                let r_string :String  = r.iter().collect();
                if r_string.contains("########") {
                    println!("Time = {}", i );
                    for c in r.iter_mut() {

                        print!("{}", c);
                    }
                    println!("");
                }
            }
        }

    println!("Result is {}", result);
    return result;
}
