use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(codes: &Vec<String>) {
    let mut result = 0;

    for code in codes {
        let numeric_code = &code[0..3].parse::<usize>().unwrap();
        let mut start:char =  'A';
        let mut paths:Vec<Vec<char>> = Vec::new();
        for digit in code.chars() {
            //how to go from A to digit
            //println!("How to go from {:?} to {:?}" ,start, digit);
            paths = get_directions(start, digit, paths, &get_numeric_keypad());
            //println!("paths is now {:?}",paths);
            start = digit;
        }
        println!("{:?}", paths);
        println!("{:?}", paths.len());


        let mut code_min:u64 = u64::MAX;
        let mut cache :HashMap<(Vec<char>,char,char,usize),u64> = HashMap::new();
        for path in &paths {
            let r = recursive(path, 'A', 0, 24, &mut cache);
            code_min = min(r, code_min);
            println!("r={}", r);
        }
        result += code_min as u64  * *numeric_code as u64;
        println!("Shortest code path = {:?}",code_min);



    }
    println!("{}", result);
}


fn recursive(p:&Vec<char>, prev:char, index:usize, depth: usize, cache: &mut HashMap<(Vec<char>,char,char,usize),u64>) -> u64 {
    //println!("recursing on start = {}, end={} at index ={} d={}", prev, p[index], index, depth);
    if cache.contains_key(&(p.clone(), prev, p[index], depth)) {
        return *cache.get(&(p.clone(),prev, p[index], depth)).unwrap();
    }

    let mut result = 0;

    // Get the different paths to go from prev to the next char
    let start_position = get_position(prev, &get_direction_keypad());
    let end_position = get_position(p[index], &get_direction_keypad());
    let shortest_paths: Vec<Vec<char>> = get_paths(start_position,end_position, &get_direction_keypad(), &Vec::new() );

    // Get shortest path for the rest of this sequence
    if index < p.len() -1 {
        result += recursive(p, p[index], index + 1, depth, cache);
    }
    let mut sp = u64::MAX;
    for path in &shortest_paths {
        if depth > 0 {
            sp  = min(sp, recursive(path, 'A',0, depth - 1, cache));
        }
        else {
            sp = min(sp, path.len() as u64);
        }
    }
    result += sp;
    cache.insert((p.clone(), prev, p[index], depth),result);
    result
}


fn get_directions(start: char, end: char, paths:Vec<Vec<char>>,keypad: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    //println!("Finding path from {:?} to {:?} with {:?}", start, end, paths);

    let s = get_position(start, keypad);
    let e =  get_position(end, keypad);
    //println!("{:?}",get_paths(s,e, keypad, &paths));
    let result = get_paths(s,e, keypad, &paths);
    result
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