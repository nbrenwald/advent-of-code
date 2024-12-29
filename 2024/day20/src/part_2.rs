use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(map: &mut Vec<Vec<char>>, start: &GridPosition, end :&GridPosition, max:i32) {
    let mut result = 0;

    let mut result_map: HashMap<usize, usize> = HashMap::new();
    let path: Vec<GridPosition> = get_path(map, start, end);
    println!("{:?}", path);

    for (i,p1) in path.iter().enumerate() {
       // println!("Looking for a shortcut from {:?}", p1);
        for j in  i .. path.len() {
            let p2 = path.get(j).unwrap();
            let distance = j-i;
           // println!("Checking {:?} which is {:?} moves ahead", path.get(j).unwrap(), j-i);

            
            let manhattan_distance = get_manhattan_distance (p1, p2);
            //println!("Checking Shortcut {:?} which is {:?} moves ahead", path.get(j).unwrap(), manhattan_distance);
            if manhattan_distance >= 2  && manhattan_distance <= 20 {
                let cheat  = distance -manhattan_distance;
                if cheat >= 100 {
                    result +=1;
                }
            }
            
        }
    }

println!("{:?}", result);
    }

    fn shortest_path(map: &mut Vec<Vec<char>>, start: &GridPosition, end: &GridPosition) -> usize {
        let mut queue = BinaryHeap::new();
        queue.push(State { position: start.clone(), cost: 0 });
        for (i, row) in map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if map[i][j] == '.' {
                    queue.push(State { position: GridPosition { row: i, col: j }, cost: usize::MAX });
                }
            }
        }

        let mut visited = HashSet::new();
        let mut costs: HashMap<GridPosition, usize> = HashMap::new();
        while !queue.is_empty() {
            let current = queue.pop().unwrap();


            if visited.contains(&current.position) {
                continue;
            }

            visited.insert(current.position);
            costs.insert(current.position, current.cost);


            if current.position.row > 0 && map[current.position.row - 1][current.position.col] != '#' {
                let mut next_cost = usize::MAX;
                if costs.contains_key(&GridPosition { row: current.position.row - 1, col: current.position.col }) {
                    next_cost = *costs.get(&GridPosition { row: current.position.row - 1, col: current.position.col }).unwrap();
                }

                if current.cost + 1 < next_cost {
                    queue.push(State { position: GridPosition { row: current.position.row - 1, col: current.position.col }, cost: current.cost + 1 });
                }
            }

            if current.position.row < map.len() - 1 && map[current.position.row + 1][current.position.col] != '#' {
                let mut next_cost = usize::MAX;
                if costs.contains_key(&GridPosition { row: current.position.row + 1, col: current.position.col }) {
                    next_cost = *costs.get(&GridPosition { row: current.position.row + 1, col: current.position.col }).unwrap();
                }

                if current.cost + 1 < next_cost {
                    queue.push(State { position: GridPosition { row: current.position.row + 1, col: current.position.col }, cost: current.cost + 1 });
                }
            }

            if current.position.col > 0 && map[current.position.row][current.position.col - 1] != '#' {
                let mut next_cost = usize::MAX;
                if costs.contains_key(&GridPosition { row: current.position.row, col: current.position.col - 1 }) {
                    next_cost = *costs.get(&GridPosition { row: current.position.row, col: current.position.col - 1 }).unwrap();
                }

                if current.cost + 1 < next_cost {
                    queue.push(State { position: GridPosition { row: current.position.row, col: current.position.col - 1 }, cost: current.cost + 1 });
                }
            }

            if current.position.col < map[0].len() - 1 && map[current.position.row][current.position.col + 1] != '#' {
                let mut next_cost = usize::MAX;
                if costs.contains_key(&GridPosition { row: current.position.row, col: current.position.col + 1 }) {
                    next_cost = *costs.get(&GridPosition{row:current.position.row, col:current.position.col+1}).unwrap();
                }

                if current.cost +1 < next_cost {
                    queue.push(State {position: GridPosition{row:current.position.row, col:current.position.col+1}, cost: current.cost +1});
                }
            }


        }

        *costs.get(&end).unwrap()

}

fn get_manhattan_distance(p1: &GridPosition, p2: &GridPosition) -> usize {
    usize::abs_diff(p1.row, p2.row) + usize::abs_diff(p1.col, p2.col)
}

fn get_path(map: &mut Vec<Vec<char>>, start: &GridPosition, end :&GridPosition) -> Vec<GridPosition> {

    let mut path: Vec<GridPosition> = Vec::new();

    let mut visited :HashSet<GridPosition> = HashSet::new();

     let mut i = start.row;
     let mut j = start.col;
     while !visited.contains(&GridPosition{row:i, col:j}) {
         println!("Checking {},{}", i, j);

         visited.insert(GridPosition{row: i, col: j});
         path.push(GridPosition{row: i, col: j});
         if i> 0 && map[i-1][j] == '.' && !visited.contains(&GridPosition{row: i-1, col: j}) {
             i = i-1;

         }
         else if j> 0 && map[i][j-1] == '.' && !visited.contains(&GridPosition{row: i, col: j-1}) {
             j = j-1;
         }
         else if i < map.len() -1 && map[i+1][j] == '.' && !visited.contains(&GridPosition{row: i+1, col: j}) {
            i = i +1;
         }
         else if j < map[0].len() -1 && map[i][j+1] == '.' && !visited.contains(&GridPosition{row: i, col: j+1}) {
             j = j +1;
         }
         else {
             break;
         }

     }
    path.push(end.clone());


    path
}
