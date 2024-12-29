use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::i32::MIN;
use utils::{direction, GridPosition, GridPositionAndDirection, Position};
use utils::direction::NORTH;
use crate::Robot;

pub(crate) fn run(map: &mut Vec<Vec<char>>, start: GridPositionAndDirection, end: GridPosition) -> i32{
    let mut result:i32 = i32::MAX;

    let mut queue:VecDeque<GridPositionAndDirection> = VecDeque::new();
    let mut visited: HashMap<GridPositionAndDirection, (i32, HashSet<GridPosition>)> = HashMap::new();

    queue.push_back(start);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if (current.pos.row ==7 && current.pos.col ==5)  {
            println!("HERE");
        }
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
            println!(" {:?} {:?}",visited.get(key).unwrap().0, visited.get(key).unwrap().1.len());
            result = min(result, visited.get(key).unwrap().0);
            for (i,row) in map.iter().enumerate(){
                for (j, c) in row.iter().enumerate(){
                    if visited.get(key).unwrap().1.contains(&GridPosition{row: i, col: j}){
                        print!("O");
                    }
                    else {print!("{}", c);
                    }
                }
                println!();

            }
        }
    }


    println!("{:?}", result);



    return result;
}

fn rotate_clockwise(current:GridPositionAndDirection, visited:&mut HashMap<GridPositionAndDirection, (i32, HashSet<GridPosition>)>, map: &mut Vec<Vec<char>>) -> Option<GridPositionAndDirection> {


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
    let mut parents:HashSet<GridPosition> = HashSet::new();
    if visited.contains_key(&current) {
        cost = visited[&current].0;
        parents = visited[&current].1.clone();
    }

    let mut newCost = cost + 1000;
    parents.insert(current.pos);

    let mut nextCost = i32::MAX;
    let mut nextParents = std::collections::HashSet::new();
    if visited.contains_key(&next) {
        nextCost = visited.get(&next).unwrap().0;
        nextParents = visited.get(&next).unwrap().1.clone();
    }

    if newCost < nextCost {
        println!("Found a cheaper way to get to {:?} with cost {}", next, newCost);
        visited.insert(next, (newCost,parents.clone()));
        return Some(next);
    }

    if newCost == nextCost {
        //println!("Found a same way to get to {:?} with cost {}", next, newCost);
        for p in nextParents {
            parents.insert(p.clone());
        }
        visited.insert(next, (newCost, parents.clone()));
        return Some(next);
    }

    None
}

fn rotate_anti_lockwise(current:GridPositionAndDirection, visited:&mut HashMap<GridPositionAndDirection, (i32, HashSet<GridPosition>)>, map: &mut Vec<Vec<char>>) -> Option<GridPositionAndDirection> {
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
    let mut parents:HashSet<GridPosition> = HashSet::new();
    if visited.contains_key(&current) {
        cost = visited[&current].0;
        parents = visited[&current].1.clone();
    }

    let mut newCost = cost + 1000;
    parents.insert(current.pos);


    let mut nextCost = i32::MAX;
    let mut nextParents = HashSet::new();
    if visited.contains_key(&next) {
        nextCost = visited.get(&next).unwrap().0;
        nextParents = visited.get(&next).unwrap().1.clone();
    }

    if newCost < nextCost {
        //println!("Found a cheaper way to get to {:?} with cost {}", next, newCost);
        visited.insert(next, (newCost,parents.clone()));
        return Some(next);
    }

    if newCost == nextCost {
        //println!("Found a same way to get to {:?} with cost {}", next, newCost);
        for p in nextParents {
            parents.insert(p.clone());
        }
        visited.insert(next, (newCost, parents.clone()));
        return Some(next);
    }

    None
}


fn next(current:GridPositionAndDirection, row_offset:i32, col_offset:i32, visited:&mut HashMap<GridPositionAndDirection, (i32, HashSet<GridPosition>)>, map: &mut Vec<Vec<char>>) -> Option<GridPositionAndDirection> {
    if current.pos.col as i32 + col_offset >= 0 &&
        current.pos.col as i32  + col_offset < map[0].len()as i32 &&
        current.pos.row  as i32 + row_offset >= 0 &&
        current.pos.row as i32 + row_offset < map.len() as i32 &&
        map[(current.pos.row as i32 + row_offset) as usize][(current.pos.col as i32 + col_offset) as usize] != '#' {
        let mut cost = 0;
        let mut parents:HashSet<GridPosition> = HashSet::new();
        if visited.contains_key(&current) {
            cost = visited[&current].0;
            parents = visited[&current].1.clone();
        }

        let mut newCost = cost + 1;
        parents.insert(current.pos);

        let mut next = GridPositionAndDirection { pos: GridPosition { row: (current.pos.row as i32 + row_offset) as usize, col: (current.pos.col as i32 + col_offset) as usize }, dir: current.dir };
        let mut nextCost = i32::MAX;
        let mut nextParents = HashSet::new();
        if visited.contains_key(&next) {
           nextCost = visited.get(&next).unwrap().0;
            nextParents = visited.get(&next).unwrap().1.clone();
        }

        if newCost < nextCost {
            println!("Found a cheaper way to get to {:?} with cost {}", next, newCost);
            visited.insert(next, (newCost, parents.clone()));
            return Some(next);
        }
        if newCost == nextCost {
            println!("Found a same way to get to {:?} with cost {}", next, newCost);
            for p in nextParents {
                parents.insert(p.clone());
            }
            visited.insert(next, (newCost, parents.clone()));
            return Some(next);
        }
    }
    None
}

