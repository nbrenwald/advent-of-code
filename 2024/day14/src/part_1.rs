use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use utils::{GridPosition, Position};
use crate::Robot;

pub(crate) fn run(robots: &mut Vec<Robot>, max_row: i32, max_col:i32) -> i32{
    let mut result = 0;

    let mut upper_left = 0;
    let mut upper_right = 0;
    let mut lower_left = 0;
    let mut lower_right = 0;

    for robot in robots {
        let mut row:i32 = robot.pos.row as i32;
        let mut col:i32 = robot.pos.col as i32;
        for i in 0.. 100 {
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

        }
        println!("Final Position row={}, col={}", row, col);
        if row < max_row/2 && col < max_col/2 {
            upper_left = upper_left + 1;
        }
        if row < max_row/2 && col > max_col/2 {
            upper_right = upper_right + 1;
        }
        if row > max_row/2 && col < max_col/2 {
            lower_left = lower_left + 1;
        }
        if row > max_row/2 && col > max_col/2 {
            lower_right = lower_right + 1;
        }
    }
    result += upper_left * upper_right * lower_right * lower_left;

    println!("Result is {}", result);
    return result;
}
