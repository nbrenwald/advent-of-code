use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::i32::MIN;
use utils::{direction, GridPosition, GridPositionAndDirection, Position};
use utils::direction::NORTH;
use crate::Robot;

pub(crate) fn run(map: &mut Vec<Vec<char>>, start: GridPositionAndDirection, end: GridPosition) -> i32{
    let mut result:i32 = i32::MAX;

    let mut queue:VecDeque<GridPositionAndDirection> = VecDeque::new();
    let mut visited: HashMap<GridPositionAndDirection, i32> = HashMap::new();

    queue.push_back(start);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let mut row_offset = 0;
        let mut col_offset = 0;
        if current.dir == direction::EAST {
            row_offset = 0;
            col_offset = 1;
        }
        else if current.dir == direction::SOUTH {
            row_offset = 1;
            col_offset = 0;
        }
        else if current.dir == direction::WEST {
            row_offset = 0;
            col_offset = -1;

        }
        else if current.dir == direction::NORTH {
            row_offset = -1;
            col_offset = 0;

        }
        let next : Option<GridPositionAndDirection> = next(current, row_offset, col_offset, &mut visited, map);
        if next.is_some() {
            queue.push_back(next.unwrap());
        }
        let rotate_anti_clockwise = rotate_anti_lockwise(current, &mut visited, map);
        if rotate_anti_clockwise.is_some() {
            queue.push_back(rotate_anti_clockwise.unwrap());
        }

        let rotate_clockwise = rotate_clockwise(current, &mut visited, map);
        if rotate_clockwise.is_some() {
            queue.push_back(rotate_clockwise.unwrap());
        }

}
    for key in visited.keys() {
        if key.pos == end {
            result = min(result, *visited.get(key).unwrap());
        }
    }
    println!("{:?}", result);

    return result;
}

fn rotate_clockwise(current:GridPositionAndDirection, visited:&mut HashMap<GridPositionAndDirection, i32>, map: &mut Vec<Vec<char>>) -> Option<GridPositionAndDirection> {
    let mut next = current.clone();
    if current.dir == direction::EAST {
        next.dir = direction::SOUTH;
    }
    if current.dir == direction::SOUTH {
        next.dir = direction::WEST;
    }
    else if current.dir == direction::WEST {
        next.dir = direction::NORTH;
    }
    else if current.dir == direction::NORTH {
        next.dir = direction::EAST;
    }

    let mut cost = 0;
    if visited.contains_key(&current) {
        cost = visited[&current];
    }
    let mut newCost = cost + 1000;

    let mut nextCost = visited.entry(next).or_insert(i32::MAX);

    if newCost < *nextCost {
        println!("Found a cheaper way to get to {:?} with cost {}", next, newCost);
        visited.insert(next, newCost);
        return Some(next);
    }

    None
}

fn rotate_anti_lockwise(current:GridPositionAndDirection, visited:&mut HashMap<GridPositionAndDirection, i32>, map: &mut Vec<Vec<char>>) -> Option<GridPositionAndDirection> {
    let mut next = current.clone();
    if current.dir == direction::EAST {
        next.dir = direction::NORTH;
    }
    if current.dir == direction::NORTH {
        next.dir = direction::WEST;
    }
    else if current.dir == direction::WEST {
        next.dir = direction::SOUTH;
    }
    else if current.dir == direction::SOUTH {
        next.dir = direction::EAST;
    }

    let mut cost = 0;
    if visited.contains_key(&current) {
        cost = visited[&current];
    }
    let mut newCost = cost + 1000;

    let mut nextCost = visited.entry(next).or_insert(i32::MAX);

    if newCost < *nextCost {
        println!("Found a cheaper way to get to {:?} with cost {}", next, newCost);
        visited.insert(next, newCost);
        return Some(next);
    }

    None
}


fn next(current:GridPositionAndDirection, row_offset:i32, col_offset:i32, visited:&mut HashMap<GridPositionAndDirection, i32>, map: &mut Vec<Vec<char>>) -> Option<GridPositionAndDirection> {
    if current.pos.col as i32 + col_offset >= 0 &&
        current.pos.col as i32  + col_offset < map[0].len()as i32 &&
        current.pos.row  as i32 + row_offset >= 0 &&
        current.pos.row as i32 + row_offset < map.len() as i32 &&
        map[(current.pos.row as i32 + row_offset) as usize][(current.pos.col as i32 + col_offset) as usize] != '#' {
        let mut cost = 0;
        if visited.contains_key(&current) {
            cost = visited[&current];
        }
        let mut newCost = cost + 1;

        let mut next = GridPositionAndDirection { pos: GridPosition { row: (current.pos.row as i32 + row_offset) as usize, col: (current.pos.col as i32 + col_offset) as usize }, dir: current.dir };
        let mut nextCost = visited.entry(next).or_insert(i32::MAX);

        if newCost < *nextCost {
            println!("Found a cheaper way to get to {:?} with cost {}", next, newCost);
            visited.insert(next, newCost);
            return Some(next);
        }
    }
    None
}

