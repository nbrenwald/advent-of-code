use std::collections::{HashMap, HashSet, VecDeque};
use utils::{GridPosition, Position};

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
        let mut total_perimiter = 0;
        for grid in &region {
            total_perimiter += perimeters.get(&grid).unwrap();
        }
        println!("Region {:?} area={}, totalPerimiter={}",region,region.len(),total_perimiter );
        result += total_perimiter* region.len() as u64;
    }



    println!("Result: {}", result);

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
