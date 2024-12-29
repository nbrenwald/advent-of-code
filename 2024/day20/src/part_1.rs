use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(map: &mut Vec<Vec<char>>, start: &GridPosition, end :&GridPosition, max:i32) {
    let mut result = 0;

    let mut result_map: HashMap<usize, usize> = HashMap::new();
    /* let mut visited :HashSet<GridPosition> = HashSet::new();

     let mut i = start.row;
     let mut j = start.col;
     while !visited.contains(&GridPosition{row:i, col:j}) {
         println!("Checking {},{}", i, j);

             if i > 1 && map[i-1][j] == '#' && map[i-2][j] !='#' {
                 let first = map[i-1][j];
                 let second = map[i-2][j];
                 map[i-1][j] = '1';
                 map[i-2][j] = '2';

                 let min = shortest_path(map, start, end);
                 if max as usize - min >= 1 {

                     let saving = max as usize - min;
                     if saving == 100 {result +=1;};
                     println!("result: {}",saving);
                     let mut count: usize = 0;
                     if result_map.contains_key(&saving) {
                         count = *result_map.get(&saving).unwrap();
                     }
                     result_map.insert(saving, count+1);

                }
                 map[i-1][j] = first;
                 map[i-2][j] = second

             }
             if j > 1 && map[i][j-1] == '#' && map[i][j-2] != '#'{
                 let first = map[i][j-1];
                 let second = map[i][j-2];
                 map[i][j-1] = '1';
                 map[i][j-2] = '2';

                 let min = shortest_path(map, start, end);
                 if max as usize - min >= 1 {
                     let saving = max as usize - min;
                     if saving == 100 {result +=1;};
                     println!("result: {}",saving);
                     let mut count: usize = 0;
                     if result_map.contains_key(&saving) {
                         count = *result_map.get(&saving).unwrap();
                     }
                     result_map.insert(saving, count+1);
                 }
                 map[i][j-1] = first;
                 map[i][j-2] = second

             }
             if j < map.len() -2 && map[i][j+1] == '.' && map[i][j+2] != '#'{
                 let first = map[i][j+1];
                 let second = map[i][j+2];
                 map[i][j+1] = '1';
                 map[i][j+2] = '2';
                 let min = shortest_path(map, start, end);
                 if max as usize - min >= 1 {
                     let saving = max as usize - min;
                     if saving == 100 {result +=1;};
                     println!("result: {}",saving);
                     let mut count: usize = 0;
                     if result_map.contains_key(&saving) {
                         count = *result_map.get(&saving).unwrap();
                     }
                     result_map.insert(saving, count+1);
                 }
                 map[i][j+1] = first;
                 map[i][j+2] = second

             }

             if j < map[0].len()-2 && map[i][j+1] == '.' && map[i][j+2] != '#'{
                 let first = map[i][j+1];
                 let second = map[i][j+2];
                 map[i][j+1] = '1';
                 map[i][j+2] = '2';

                 let min = shortest_path(map, start, end);
                 if max as usize - min >= 1 {
                     let saving = max as usize - min;
                     if saving == 100 {result +=1;};
                     println!("result: {}",saving);
                     let mut count: usize = 0;
                     if result_map.contains_key(&saving) {
                         count = *result_map.get(&saving).unwrap();
                     }
                     result_map.insert(saving, count+1);
                 }
                 map[i][j+1] = first;
                 map[i][j+2] = second

             }
         visited.insert(GridPosition{row: i, col: j});
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

     }*/
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '#' {
                map[i][j] = '.';
                let min = shortest_path(map, start, end);
                let saving = max as usize - min;
                if saving >= 100 { result += 1;
                    println!("result: {}", saving);};

                let mut count: usize = 0;
                if result_map.contains_key(&saving) {
                    count = *result_map.get(&saving).unwrap();
                }
                result_map.insert(saving, count + 1);
                map[i][j] = '#';
            }
        }

        println!("result: {:?}", result_map);
        println!("result: {:?}", result);
        /* map[8][8] = '1';
         map[9][8] = '2';
         print_grid(map);
         let min = shortest_path(map, start, end);
         print!("result: {}", max as usize - min);*/
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

}
