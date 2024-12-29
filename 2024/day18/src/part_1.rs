use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

use utils::{direction, GridPosition, GridPositionAndDirection, Position};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    cost: usize,
    position: GridPosition,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.row.cmp(&other.position.row))
            .then_with(|| self.position.col.cmp(&other.position.col))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub(crate) fn run(map:&Vec<Vec<char>>, start:GridPosition, end:GridPosition) {
    let mut result:i32 = i32::MAX;

    let mut unvisited: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashSet<GridPosition> = HashSet::new();
    let mut costs: HashMap<GridPosition, usize> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len(){
            unvisited.push(State {cost: usize::MAX, position:GridPosition{row:i, col:j}});
        }
    }

    unvisited.push(State {cost:0, position:start});

    while !unvisited.is_empty() {
        let current: State = unvisited.pop().unwrap();
        println!("Visiting {:?}", current);
        if visited.contains(&current.position) {
            break;
        }
        costs.insert(current.position, current.cost);

            // work out the neighbours. See if the cost is lower
            if current.position.row > 0 {
                if map[current.position.row - 1][current.position.col] != '#' {
                    let mut nextCost = usize::MAX;
                    if costs.contains_key(&GridPosition{row:current.position.row-1, col:current.position.col}) {
                        nextCost = *costs.get(&GridPosition { row: current.position.row - 1, col: current.position.col }).unwrap();
                    }
                    if current.cost + 1 < nextCost {
                        costs.insert(GridPosition{row:current.position.row-1, col:current.position.col}, current.cost + 1);
                        unvisited.push(State {cost:current.cost + 1, position:GridPosition{row:current.position.row-1, col:current.position.col}});
                    }
                }
                //check above
            }
            if current.position.col > 0 {
                if map[current.position.row][current.position.col-1] != '#' {
                    let mut nextCost = usize::MAX;
                    if costs.contains_key(&GridPosition{row:current.position.row, col:current.position.col-1}) {
                        nextCost = *costs.get(&GridPosition { row: current.position.row, col: current.position.col-1}).unwrap();
                    }
                    if current.cost + 1 < nextCost {
                        costs.insert(GridPosition{row:current.position.row, col:current.position.col-1}, current.cost + 1);
                        unvisited.push(State {cost:current.cost + 1, position:GridPosition{row:current.position.row, col:current.position.col-1}});
                    }
                } //check above
            }
            if current.position.row < map.len() -1 {
                if map[current.position.row +1][current.position.col] != '#' {
                    let mut nextCost = usize::MAX;
                    if costs.contains_key(&GridPosition{row:current.position.row+1, col:current.position.col}) {
                        nextCost = *costs.get(&GridPosition { row: current.position.row+1, col: current.position.col}).unwrap();
                    }
                    if current.cost + 1 < nextCost {
                        costs.insert(GridPosition{row:current.position.row+1, col:current.position.col}, current.cost + 1);
                        unvisited.push(State {cost:current.cost + 1, position:GridPosition{row:current.position.row+1, col:current.position.col}});
                    }
                }
                //check above
            }
            if current.position.col < map[0].len() -1 {
                if map[current.position.row][current.position.col+1] != '#' {
                    let mut nextCost = usize::MAX;
                    if costs.contains_key(&GridPosition{row:current.position.row, col:current.position.col+1}) {
                        nextCost = *costs.get(&GridPosition { row: current.position.row, col: current.position.col+1}).unwrap();
                    }
                    if current.cost + 1 < nextCost {
                        costs.insert(GridPosition{row:current.position.row, col:current.position.col+1}, current.cost + 1);
                        unvisited.push(State {cost:current.cost + 1, position:GridPosition{row:current.position.row, col:current.position.col+1}});
                    }
                } //check above
            }
            visited.insert(current.position);

            if current.position == end {
                //return
            }




    }

    println!("{:?}", costs.get(&end));
}

