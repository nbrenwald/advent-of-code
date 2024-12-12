use std::collections::{HashMap, HashSet, VecDeque};
use utils::{direction, GridPosition, Position};
use utils::direction::{EAST, NORTH, SOUTH, WEST};

pub(crate) fn run(map: &Vec<Vec<char>>){

    let mut result:u64 = 0;

    // keep track of grids already in a region
    let mut visited_grids : HashSet<GridPosition> = HashSet::new();

    let mut regions : Vec<HashSet<GridPosition>> = Vec::new();

    let mut perimeters : HashMap<GridPosition, u64> = HashMap::new();

    // iterate over all cells
    for (row, line) in map.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            let grid_position = GridPosition {col:col, row:row};
            println!("Checking {:?}", grid_position);
            if !visited_grids.contains(&grid_position) {
                println!("Region Search at {:?}", grid_position);

                let mut region : HashSet<GridPosition> = HashSet::new();
                let mut queue : VecDeque<GridPosition> = VecDeque::new();
                queue.push_back(grid_position);
                bfs( &mut queue, &mut region, &mut visited_grids, &map, &mut perimeters);

                if region.len() > 0 {
                    println!("New Region Found {:?}", region);
                    regions.push(region);
                }
            }
        }
    }

    println!("regions: {:?}", regions);
    for region in regions {

        let z:u64 = get_edges2(&region, map);
        let mut total_perimiter = 0;
        for grid in &region {
            total_perimiter += perimeters.get(&grid).unwrap();
        }
        println!("Region {:?} area={}, totalCorner={}",region,region.len(),z );
        result += z* region.len() as u64;
    }



    println!("Result: {}", result);

}

fn get_edges2(region: &HashSet<GridPosition>, map: &Vec<Vec<char>>) -> u64 {

    let mut largeMap : Vec<Vec<char>>= Vec::new() ;

   for row in  0..map.len() +2 {
       let mut largeRow : Vec<char> = Vec::new();
       for col in 0..map[0].len() +2 {
           largeRow.push('.');
       }
       largeMap.push(largeRow);
   }

    for p in region {
        largeMap[p.row +1][p.col +1] = 'X';
    }

    let mut corner: u64 = 0;
    for (i,row) in largeMap.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '.' {
                if j>0 && largeMap[i][j-1] == 'X' && i> 0 && largeMap[i-1][j] == 'X' {
                    corner +=1;
                }
                if j>0 && largeMap[i][j-1] == 'X' && i< largeMap.len() -1 && largeMap[i+1][j] == 'X' {
                    corner +=1;
                }
                if j < largeMap[0].len() -1 && largeMap[i][j+1] == 'X' && largeMap[i-1][j] == 'X' {
                    corner +=1;
                }
                if j < largeMap[0].len() -1 && largeMap[i][j+1] == 'X' && i < largeMap.len()-1 && largeMap[i+1][j] == 'X' {
                    corner +=1;
                }
                //
                if j > 0 && largeMap[i][j-1] == '.' && i> 0 && largeMap[i-1][j] == '.'  && largeMap[i-1][j-1] == 'X' {
                    corner +=1;
                }
                if j>0 && i < largeMap.len() -1 && largeMap[i][j-1] == '.' && largeMap[i+1][j] == '.' && largeMap[i+1][j-1] == 'X' {
                    corner +=1;
                }
                if j< largeMap[0].len()-1 && largeMap[i][j+1] == '.' && i > 0 && largeMap[i-1][j] == '.' && largeMap[i-1][j+1] == 'X' {
                    corner +=1;
                }
                if j< largeMap[0].len()-1 && i< largeMap.len() -1 && largeMap[i][j+1] == '.' && largeMap[i+1][j] == '.' && largeMap[i+1][j+1] == 'X'{
                    corner +=1;
                }
            }
        }
    }



    return corner;
}

fn get_edges(region: &HashSet<GridPosition>, map: &Vec<Vec<char>>) -> u64 {


    // Make a big map of '.' with two extra rows and two extra columns.
    // populate with my region with everything shifted 1 across an 1 down
    //Go over every position. if a . touches a single non . its on a diagnola, its an outside corner.
    // if a . touches a pair of non dots its an inner corner

    // Pick any node to start
    // Go round clock wise.
    // Record the starting direction.
    // If we ever get back, then we finished. Each time we change direction, we add 1.
    let start = region.iter().collect::<Vec<_>>()[0];
    let c = map[start.row][start.col];

    let mut nextDirection;
    let mut next: GridPosition;

    if start.col < map[0].len() - 1  && map[start.row][start.col + 1] == c {
        nextDirection = EAST;
        next = GridPosition {row: start.row, col: start.col + 1};
    }
    else if start.row < map.len() - 1  && map[start.row + 1][start.col] == c {
        nextDirection = SOUTH;
        next = GridPosition {row: start.row + 1, col: start.col};
    }
    else if start.col > 0  && map[start.row][start.col - 1] == c {
        nextDirection = WEST;
        next = GridPosition {row: start.row, col: start.col - 1};
    }
    else if start.row > 0  && map[start.row - 1][start.col] == c{
        nextDirection = NORTH;
        next = GridPosition {row: start.row, col: start.col - 1};
    }
    else {return 4;}

    let startDirection = nextDirection;
    println!("Started at {:?} direction {:?}, currently at {:?} heading {:?}", start, startDirection, next, nextDirection );

    let mut changes:u64 = 0;
    while ! (*start == next && startDirection == nextDirection)  {
        println!("Started at {:?} direction {:?}, currently at {:?} heading {:?}", start, startDirection, next, nextDirection );
        if nextDirection == EAST {
            if next.col < map[0].len() - 1  && map[next.row][next.col + 1] == c {
                next.col = next.col +1; // step left, keep same direction
            }
            else {
                nextDirection = EAST;
                changes +=1;
            }
        }
        else if nextDirection == SOUTH {
            if next.row < map.len() - 1  && map[next.row+1][next.col] == c {
                next.row = next.row +1; // step down, keep same direction
            }
            else {
                nextDirection = WEST;
                changes +=1;
            }
        }
        else if nextDirection == WEST {
            if next.col > 0  && map[next.row][next.col-1] == c {
                next.col = next.col - 1; // step down, keep same direction
            }
            else {
                nextDirection = NORTH;
                changes +=1;
            }
        }
        else if nextDirection == NORTH {
            if next.row > 0  && map[next.row - 1][next.col] == c {
                next.row = next.row - 1; // step down, keep same direction
            }
            else {
                nextDirection = EAST;
                changes +=1;
            }
        }

    }

    // Determine start direction

    return changes;
}

fn bfs(queue: &mut VecDeque<GridPosition>, region: &mut HashSet<GridPosition>, visited_grids: &mut HashSet<GridPosition>, map: &&Vec<Vec<char>>, perimeters: &mut HashMap<GridPosition, u64>){
    // Check if position is already in a region


    while queue.len() > 0 {
        let mut perimiter = 4;
        let mut position = queue.pop_front().unwrap();
        if !visited_grids.contains(&position) {

            let c = map[position.row][position.col];

            if position.row > 0 {
                let above: GridPosition = GridPosition { row: position.row - 1, col: position.col };
                if !visited_grids.contains(&above) && map[above.row][above.col] == c {
                    queue.push_back(above.clone());
                    perimiter -= 1;
                } else if map[above.row][above.col] == c {
                    perimiter -= 1;
                }
            }

            if position.col > 0 {
                let left: GridPosition = GridPosition { row: position.row, col: position.col - 1 };
                if !visited_grids.contains(&left) && map[left.row][left.col] == c {
                    queue.push_back(left.clone());
                    perimiter -= 1;
                } else if map[left.row][left.col] == c {
                    perimiter -= 1;
                }
            }

            if position.row < map.len() - 1 {
                let below: GridPosition = GridPosition { row: position.row + 1, col: position.col };
                if !visited_grids.contains(&below) && map[below.row][below.col] == c {
                    queue.push_back(below.clone());
                    perimiter -= 1;
                } else if map[below.row][below.col] == c {
                    perimiter -= 1;
                }
            }

            if position.col < map[0].len() - 1 {
                let right: GridPosition = GridPosition { row: position.row, col: position.col + 1 };
                if !visited_grids.contains(&right) && map[right.row][right.col] == c {
                    queue.push_back(right.clone());
                    perimiter -= 1;
                } else if map[right.row][right.col] == c {
                    perimiter -= 1;
                }
            }

            println!("Finished searning {:?}, perimiter={}", position, perimiter);
            //println!("Queue {:?}", queue);
            region.insert(position.clone());
            visited_grids.insert(position.clone());
            perimeters.insert(position.clone(), perimiter);
        }}

}
