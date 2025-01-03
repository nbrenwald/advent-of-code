use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(codes: &Vec<String>) {
    let mut result = 0;

    for code in codes {
        let numeric_code = &code[0..3].parse::<usize>().unwrap();
        let mut start:char =  'A';
        let mut directions: Vec<char> = Vec::new();
        let mut directions2: Vec<char> = Vec::new();
        let mut paths:Vec<Vec<char>> = Vec::new();
        for digit in code.chars() {
            //how to go from A to digit
            //println!("How to go from {:?} to {:?}" ,start, digit);
            paths = get_directions(start, digit, paths, &get_numeric_keypad());
            println!("paths is now {:?}",paths);
            start = digit;
        }
        println!("{:?}", paths);
        println!("{:?}", paths.len());
        if directions.len() != directions2.len() {println!("Diff");}

        paths = translate(&paths);
        paths = translate(&paths);


        //println!("{:?}", translation);
        let mut t_min = u64::MAX;
        for t in &paths {
            t_min = min(t_min,t.len() as u64);
        }
        println!("{:?}", paths.len());
        println!("{:?}", t_min);
        result += t_min * *numeric_code as u64;
    }
println!("{}", result);
    //107430 too high
    //107430 too high
}

fn translate(paths:&Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut start:char =  'A';
    let mut translation_results:Vec<Vec<char>> = Vec::new();
    for path in paths {
        let mut translations:Vec<Vec<char>> = Vec::new();
        //println!("translating {:?}",path);
        for digit in path {
            //how to go from A to digit
            //println!("How to go from {:?} to {:?}" ,start, digit);
            translations = get_directions(start, *digit, translations, &get_direction_keypad());
            //println!("translation is now {:?}", translation);
            start = *digit;
        }
        for t in &translations {
            translation_results.push(t.clone());
        }

    }
    translation_results
}


fn get_directions(start: char, end: char, paths:Vec<Vec<char>>,keypad: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    //println!("Finding path from {:?} to {:?} with {:?}", start, end, paths);

    let s = get_position(start, keypad);
    let e =  get_position(end, keypad);
    //println!("{:?}",get_paths(s,e, keypad, &paths));
    let result = get_paths(s,e, keypad, &paths);
    return result;
}

//BFS each solution can add an ^ <


fn get_paths(start: GridPosition, end: GridPosition, keypad: &Vec<Vec<char>>, paths:&Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    let going_up:bool = end.row < start.row;
    let going_right:bool = start.col < end.col;
    //each item on the queue is a path of directions and a position
    let mut queue:VecDeque<(GridPosition,Vec<char>)> = VecDeque::new();
    if paths.is_empty(){
        //println!("Paths is empty");
        queue.push_back((start, Vec::new()));
    }
    else {
        for p in paths {
            queue.push_back((start, p.clone()));
        }
    }


    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.0 == end {
            let mut nextPath = current.1.clone();
            nextPath.push('A');
            result.push(nextPath);
            continue;
        }

        if keypad[current.0.row][current.0.col] == 'X' {
            continue;
        }

        if current.0.col > 0 && !going_right {
            let nextGridPosition = GridPosition{col:current.0.col-1, row:current.0.row};
            let mut nextPath = current.1.clone();
            nextPath.push('<');
            queue.push_back((nextGridPosition, nextPath));
        }
        if current.0.row > 0 && going_up {
            let nextGridPosition = GridPosition{col:current.0.col, row:current.0.row-1};
            let mut nextPath = current.1.clone();
            nextPath.push('^');
            queue.push_back((nextGridPosition, nextPath));
        }
        if current.0.row < keypad.len() -1 && !going_up {
            let nextGridPosition = GridPosition{col:current.0.col, row:current.0.row+1};
            let mut nextPath = current.1.clone();
            nextPath.push('v');
            queue.push_back((nextGridPosition, nextPath));
        }
        if current.0.col < keypad[0].len() -1 && going_right{
            let nextGridPosition = GridPosition{col:current.0.col+1, row:current.0.row};
            let mut nextPath = current.1.clone();
            nextPath.push('>');
            queue.push_back((nextGridPosition, nextPath));
        }

    }
    result
}


fn get_position(key:char, keypad:&Vec<Vec<char>>) -> GridPosition {
    let mut result:GridPosition = GridPosition{col:0, row:0};
    for (i,row) in keypad.iter().enumerate(){
        for (j,c) in row.iter().enumerate() {
            if keypad[i][j] == key {
                result.row=i;
                result.col=j;
            }
        }
    }
    result
}

fn get_numeric_keypad() -> Vec<Vec<char>> {
    let mut result:Vec<Vec<char>> = vec![vec!['X';3]; 4];
    result[0][0] = '7';
    result[0][1] = '8';
    result[0][2] = '9';
    result[1][0] = '4';
    result[1][1] = '5';
    result[1][2] = '6';
    result[2][0] = '1';
    result[2][1] = '2';
    result[2][2] = '3';
    result[3][0] = 'X';
    result[3][1] = '0';
    result[3][2] = 'A';
    result
}

fn get_direction_keypad() -> Vec<Vec<char>> {
    let mut result:Vec<Vec<char>> = vec![vec!['X';3]; 2];
    result[0][0] = 'X';
    result[0][1] = '^';
    result[0][2] = 'A';
    result[1][0] = '<';
    result[1][1] = 'v';
    result[1][2] = '>';
    result
}