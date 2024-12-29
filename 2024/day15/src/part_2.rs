use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use utils::{in_grid, print_grid, GridPosition, Position};
use crate::Robot;

pub(crate) fn run(original_map: &mut Vec<Vec<char>>, start: GridPosition, ins: &Vec<char>) -> i32{

    let mut map: &mut Vec<Vec<char>> = &mut Vec::new();
    for row in original_map.iter() {
        let mut new_row = Vec::new();
        for col in row.iter() {
            match col {
                '#' => {new_row.push('#');  new_row.push('#');}
                '.' => {new_row.push('.');  new_row.push('.');}
                '@' => {new_row.push('@');  new_row.push('.');}
                'O' => {new_row.push('[');  new_row.push(']');}
                &_ => {}
            }
        }
        map.push(new_row);
    }

    let mut result:i32 = 0;
    println!("start is {:?}", start);
    println!("ins is {:?}", start);
    let mut sub = start.clone();
    sub.col = sub.col *2;

    for c in ins {
        println!("instruction is {:?}", c);
        print_grid(map);
        match c{
            '^'=>{
                if can_be_shifted_vertical(map, sub, -1, 0) {
                    shift_vertical2(map, sub, -1, 0);
                    sub.row = sub.row -1;
                }
            },
            '<'=> {
                if can_be_shifted_horizontal(map, sub, 0, -1) {
                    shift_horizontal(map, sub, 0, -1, '.');
                    sub.col = sub.col -1;
                }
            },
            '>'=> {
                if can_be_shifted_horizontal(map, sub, 0, 1) {
                    shift_horizontal(map, sub, 0, 1, '.');
                    sub.col = sub.col + 1;
                }

            },
            'v'=>{
                if can_be_shifted_vertical(map, sub, 1, 0) {
                    shift_vertical2(map, sub, 1, 0);
                    sub.row = sub.row +1;
                }
            },
            _ => {}
        }

    }

    for (i,r) in map.iter().enumerate() {
        for (j,c) in r.iter().enumerate() {
            print!("{}",c);
            if *c == '[' {
                result += ((100*i) +j) as i32;
            }
        }
        println!();
    }


    println!("Result is {}", result);
    return result;
}



fn can_be_shifted_horizontal(map: &mut Vec<Vec<char>>, mut start: GridPosition, row_offset: i32, col_offset: i32) -> bool {
    let mut ptr_row = start.row as i32 +row_offset;
    let mut ptr_col = start.col as i32 +col_offset;

    if !in_grid(ptr_row, ptr_col, map) {return false;}

    if map[ptr_row as usize][ptr_col as usize] == '.'{ return true;}

    if map[ptr_row as usize][ptr_col as usize] == '#'{ return false;}

    can_be_shifted_horizontal(map, GridPosition{row:start.row, col:(start.col as i32 + 2*col_offset) as usize}, row_offset, col_offset)
}

fn can_be_shifted_vertical(map: &mut Vec<Vec<char>>, mut start: GridPosition, row_offset: i32, col_offset: i32) -> bool {
    let mut ptr_row = start.row as i32 + row_offset;
    let mut ptr_col = start.col as i32 + col_offset;

    if !in_grid(ptr_row, ptr_col, map) { return false; }

    if map[ptr_row as usize][ptr_col as usize] == '.' { return true; }

    if map[ptr_row as usize][ptr_col as usize] == '#' { return false; }

    if map[ptr_row as usize][ptr_col as usize] == '[' {
        return crate::part_2::can_be_shifted_vertical(map, GridPosition { row: (start.row as i32 + row_offset) as usize, col: start.col }, row_offset, col_offset)
            && crate::part_2::can_be_shifted_vertical(map, GridPosition { row: (start.row as i32 + row_offset) as usize, col: start.col + 1 }, row_offset, col_offset)
    }

    return crate::part_2::can_be_shifted_vertical(map, GridPosition { row: (start.row as i32 + row_offset) as usize, col: start.col }, row_offset, col_offset)
        && crate::part_2::can_be_shifted_vertical(map, GridPosition { row: (start.row as i32 + row_offset) as usize, col: start.col -1 }, row_offset, col_offset)


}


fn shift_horizontal(map: &mut Vec<Vec<char>>, mut start: GridPosition, row_offset: i32, col_offset: i32, c:char) -> () {

    let old = map[start.row][start.col];
    map[start.row][start.col] = c;

    if old == '.' {return;}

    shift_horizontal(map, GridPosition{row:start.row, col:(start.col as i32 + col_offset) as usize}, row_offset, col_offset, old)
}


fn shift_vertical2(map: &mut Vec<Vec<char>>, mut start: GridPosition, row_offset: i32, col_offset: i32) -> () {
    let mut ptr_row = start.row as i32 + row_offset;
    let mut ptr_col = start.col as i32 + col_offset;

    let old = map[ptr_row as usize][ptr_col as usize];
    if map[ptr_row as usize][ptr_col as usize] == '.' {
        map[ptr_row as usize][ptr_col as usize] = '@';
        map[start.row][start.col] = '.';
    } else {
        //Shift boxes
        if map[ptr_row as usize][ptr_col as usize] == '[' {
            shift_boxes(map, GridPosition { row: ptr_row as usize, col: start.col }, GridPosition { row: ptr_row as usize, col: start.col+1 }, row_offset);
            map[ptr_row as usize][ptr_col as usize] = '@';
            map[start.row][start.col] = '.';
        }
        else if  map[ptr_row as usize][ptr_col as usize] == ']' {
            shift_boxes(map, GridPosition { row: ptr_row as usize, col: start.col-1 }, GridPosition { row: ptr_row as usize, col: start.col }, row_offset);
            map[ptr_row as usize][ptr_col as usize] = '@';
            map[start.row][start.col] = '.';

        }
    }
}

fn shift_boxes(map: &mut Vec<Vec<char>>, lhs: GridPosition, rhs: GridPosition, row_offset: i32) {


    if map[(lhs.row as i32 + row_offset) as usize][lhs.col] == '[' && map[(rhs.row as i32 + row_offset) as usize][rhs.col] == ']' {
        shift_boxes(map, GridPosition{row:(lhs.row as i32 + row_offset) as usize, col:lhs.col}, GridPosition{row:(rhs.row as i32 + row_offset) as usize, col:rhs.col}, row_offset )
    }

    if map[(lhs.row as i32 + row_offset) as usize][lhs.col] == ']'  {
        shift_boxes(map, GridPosition{row:(lhs.row as i32 + row_offset) as usize, col:lhs.col-1}, GridPosition{row:(lhs.row as i32 + row_offset) as usize, col:lhs.col}, row_offset )
    }
    if map[(rhs.row as i32 + row_offset) as usize][rhs.col] == '['  {
        shift_boxes(map, GridPosition{row:(rhs.row as i32 + row_offset) as usize, col:rhs.col}, GridPosition{row:(rhs.row as i32 + row_offset) as usize, col:rhs.col+1}, row_offset )
    }

    map[(lhs.row as i32 + row_offset) as usize][lhs.col] = map[lhs.row][lhs.col];
    map[(rhs.row as i32 + row_offset) as usize][rhs.col] = map[rhs.row][rhs.col];
    map[lhs.row][lhs.col] = '.';
    map[rhs.row][rhs.col] = '.';

}
