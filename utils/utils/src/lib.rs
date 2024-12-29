use std::cmp::Ordering;
use std::fs::read_to_string;

pub fn group_text(file: &str) -> Vec<Vec<String>> {
    let mut group:Vec<String> = Vec::new();
    let mut text:Vec<Vec<String>> = Vec::new();
    for line in read_to_string(file).unwrap().lines() {
        if line == "" {
            if group.len() != 0 {
                text.push(group);
                group = Vec::new();
            }
        }
        else {
            group.push(line.to_string());
        }
    }
    if group.len() != 0 {
        text.push(group);
    }
    text
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct State {
    pub cost: usize,
    pub position: GridPosition,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GridPositionAndDirection {
    pub pos: GridPosition,
    pub dir: direction
}

#[derive(Debug, Copy, Clone)]
pub struct Segment {
    pub first: Position,
    pub second: Position
}

pub fn print_grid<T: std::fmt::Display>(map: &Vec<Vec<T>>) {
    for (i, r) in map.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            print!("{}", c);
        }
        println!();
    }
}

pub fn in_grid<T>(row: i32, col:i32, map: &Vec<Vec<T>>) -> bool {
    row > 0 && col > 0 && row  < map.len() as i32 && col < map[0].len() as i32
}

pub fn on_line(p:Position, s:&Segment) -> bool {

    let changeY:i32 = s.second.y - s.first.y;
    let changeX:i32 = s.second.x - s.first.x;
    let m:f32 = changeY as f32/changeX as f32;

    let c = s.first.y as f32 - (m * s.first.x as f32);
    let calcY = (p.x as f32 * m) + c;

    if (((p.x as f32 * m) + c) - p.y as f32).abs() < 0.001 {
        return true;
    }
    false
}


/*
check if a discrete place on a grid is on the same line as a given segment
*/
pub fn on_grid_line(p:Position, s:&Segment) -> bool {

    let changeY:i32 = s.second.y - s.first.y;
    let changeX:i32 = s.second.x - s.first.x;

    let multiplier = (s.first.x - p.x) / changeX;
    if (s.first.x - p.x) % changeX == 0 {
        if p.y + (changeY * multiplier) == s.first.y {
            return true;
        }
    }
    false
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
