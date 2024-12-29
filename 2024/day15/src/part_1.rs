use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use utils::{GridPosition, Position};
use crate::Robot;

pub(crate) fn run(map: &mut Vec<Vec<char>>, start: GridPosition, ins: &Vec<char>) -> i32{
    let mut result:i32 = 0;
    println!("map is {:?}", map);
    println!("start is {:?}", start);
    println!("ins is {:?}", start);
    let mut sub = start.clone();

    for c in ins {
        match c{
            '^'=>{
                sub = row_shift(map, sub, -1, 0);

            },
            '<'=> {
                sub = row_shift(map, sub, 0, -1);
            },
            '>'=> {
                sub = row_shift(map, sub, 0, 1);

            },
            'v'=>{
                sub = row_shift(map, sub, 1, 0);
            },
            _ => {}
        }

    }

    for (i,r) in map.iter().enumerate() {
        for (j,c) in r.iter().enumerate() {
            print!("{}",c);
            if *c == 'O' {
                result += ((100*i) +j) as i32;
            }
        }
        println!();
    }


    println!("Result is {}", result);
    return result;
}

fn row_shift(map: &mut Vec<Vec<char>>, mut start: GridPosition, row_offset: i32, col_offset: i32) -> GridPosition {
    let mut ptr_row = start.row as i32 + row_offset;
    let mut ptr_col = start.col as i32 + col_offset;
    let mut pos = start.clone();
    if  ptr_row > 0 && ptr_col > 0 && ptr_row  < map.len() as i32 && ptr_col < map[0].len() as i32 && map[ptr_row as usize][ptr_col as usize] == '.'{
        map[ptr_row as usize][ptr_col as usize] = '@';
        map[pos.row][pos.col] = '.';
        return GridPosition{row: ptr_row as usize, col:ptr_col as usize};
    }
    while ptr_row > 0 && ptr_col > 0 && ptr_row < map.len() as i32 && ptr_col < map[0].len() as i32 {
        if map[ptr_row as usize ][ptr_col as usize] == '#'{
            break;
        }
        if map[ptr_row as usize][ptr_col as usize] == '.'{
            map[ptr_row as usize][ptr_col as usize] = 'O';
            map[start.row][start.col] = '.';
            map[(start.row as i32 + row_offset) as usize][(start.col as i32 + col_offset) as usize] = '@';
            pos = GridPosition{row: (start.row as i32 + row_offset) as usize, col: (start.col as i32 + col_offset) as usize};
            break;
        }
        ptr_row = ptr_row + row_offset;
        ptr_col = ptr_col + col_offset;
    }

    pos
}
