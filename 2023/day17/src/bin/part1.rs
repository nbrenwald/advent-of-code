use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::cmp;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    vertex: Vertex,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            //.then_with(|| self.vertex.cmp(&other.vertex))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Above,
    Below,
    Left,
    Right
}

const DIRECTION_VALUES: [Direction; 4] = [Direction::Above, Direction::Below, Direction::Left, Direction::Right];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vertex {
    pos: (usize, usize),
    d: Direction,
    straight: usize,
}

fn group_text(file: &str) -> Vec<Vec<String>> {
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


fn main() -> io::Result<()> {
    let mut total = 0;
    let mut result = 0;
    let mut map: Vec<Vec<usize>> = Vec::new();
    
    for group in group_text("sample.txt").iter(){

        for line in group.iter() {
            let mut row: Vec<usize> = Vec::new();
            for c in line.chars(){
                println!("{}", c);
                row.push(c.to_digit(10).unwrap().try_into().unwrap());
            }
            map.push(row);
        }
    }


    let mut graph: HashMap<Vertex, Vec<(Vertex, usize)>> = build_graph(&map);
    println!("graph = {:?} ", graph);
    println!("{}" , shortest_path(&graph));
    Ok(())
}

fn shortest_path(graph: &HashMap<Vertex, Vec<(Vertex, usize)>>) -> usize {

    let mut current_vertex = Vertex{pos: (0,0), d:Direction::Left, straight: 0};
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut shortest: HashMap<Vertex, usize> = HashMap::new();

    let mut best = BinaryHeap::new();
    best.push(State{cost:0, vertex:current_vertex});

    while !best.is_empty() {

    let state = best.pop().unwrap();
    println!("Checking {:?}", state.vertex);
    if visited.contains(&state.vertex) {continue;}
        println!("{:?}", state.vertex);
        let adjacency_list = graph.get(&state.vertex).unwrap();
        for e in adjacency_list.iter() {
            best.push(State{cost: e.1 + state.cost, vertex:e.0});
        }
        visited.insert(state.vertex);
        shortest.insert(state.vertex, state.cost);
    }

    let mut x = 0;
    let mut y = 0 ;
    for (key, value) in shortest.iter() {
       x = cmp::max(x, key.pos.0);
       y = cmp::max(y, key.pos.1);
    }
    for (key, value) in shortest.iter() {
       if key.pos == (x, y) {
          println!("{:?} {}", key, value)
       }
    }
    println!("{} {}",graph.keys().len(), shortest.keys().len());
    0
} 

    fn build_graph(map: &Vec<Vec<usize>>) -> HashMap<Vertex, Vec<(Vertex, usize)>> {
            let mut graph: HashMap<Vertex, Vec<(Vertex, usize)>> = HashMap::new();
            let mut row = 0;
            while row < map.len() {
                let mut col = 0;
                while col < map[0].len() {
                    for d in DIRECTION_VALUES.iter() {
                        let mut straight_counter = 0;
                        while straight_counter < 3 {
                            println!("Create Vertex {} {} {:?} {}", row, col, d, straight_counter);
                            let adjacency_list = build_adjacency_list(&map, (row, col), *d, straight_counter);
                            println!("Edges {:?}", adjacency_list);
                            let v:Vertex = Vertex{pos:(row, col), d:*d, straight:straight_counter};
                            graph.insert(v, adjacency_list);
                            straight_counter += 1;
                        }
                        

                    }
                    col += 1;
                }
                row += 1;
            }
            graph
    }

   fn build_adjacency_list(map: &Vec<Vec<usize>>, pos:(usize,usize), dir:Direction, straight: usize) -> Vec<(Vertex, usize)> {
       let mut result: Vec<(Vertex, usize)> = Vec::new();

       println!("{:?} {:?} {}", pos, dir, straight);
    if dir == Direction::Left {
        if pos.0 > 0 {
            result.push( (Vertex{pos:(pos.0-1,pos.1), d:Direction::Below, straight:0 }, map[pos.0-1][pos.1]) )
            //cmp::min(result, dfs(map, (pos.0-1, pos.1), 0, Direction::Below, visited, score+map[pos.0][pos.1]));
        }
        if pos.0 < map.len() - 1 {
            result.push( (Vertex{pos:(pos.0+1,pos.1), d:Direction::Above, straight:0 }, map[pos.0+1][pos.1]) )
            //result.push( (Vertex{pos:(pos.0+1, pos.1), 0, Direction::Above}, map[pos.0][pos.1])); 
        }
        if straight < 2 && pos.1 < map[0].len() - 1 {
            
            result.push( (Vertex{pos:(pos.0,pos.1+1), d:dir, straight: straight + 1 }, map[pos.0][pos.1+1]) )
            //result.push( (Vertex{pos:(pos.0, pos.1 + 1), straight + 1, dir}, map[pos.0][pos.1])); 
        }
    }
    if dir == Direction::Right {
        if pos.0 > 0 {
            result.push( (Vertex{pos:(pos.0-1, pos.1), straight:0, d:Direction::Below}, map[pos.0-1][pos.1])); 
        }
        if pos.0 < map.len() - 1 {
            result.push( (Vertex{pos:(pos.0+1, pos.1), straight:0, d: Direction::Above}, map[pos.0+1][pos.1])); 
        }
        if straight < 2 && pos.1 > 0 {
            result.push( (Vertex{pos:(pos.0, pos.1 - 1), straight: straight + 1, d: dir}, map[pos.0][pos.1-1])); 
        }
    }
    if dir == Direction::Above {
        if pos.1 > 0 {
            result.push( (Vertex{pos:(pos.0, pos.1-1), straight:0, d: Direction::Right}, map[pos.0][pos.1-1])); 
        }
        if pos.1 < map[0].len() - 1 {
            result.push( (Vertex{pos:(pos.0, pos.1 + 1), straight:0, d: Direction::Left}, map[pos.0][pos.1+1])); 
        }
        if straight < 2 && pos.0 < map.len() -1 {
            result.push( (Vertex{pos:(pos.0 + 1, pos.1), straight: straight + 1, d: dir}, map[pos.0+1][pos.1])); 
        }
    }
    if dir == Direction::Below {
        if pos.1 > 0 {
            result.push( (Vertex{pos:(pos.0, pos.1-1), straight: 0, d: Direction::Right}, map[pos.0][pos.1-1])); 
        }
        if pos.1 < map[0].len() - 1 {
            result.push( (Vertex{pos:(pos.0, pos.1 + 1), straight: 0, d: Direction::Left}, map[pos.0][pos.1+1] )); 
        }
        if straight < 2 && pos.0 > 0 {
            result.push( (Vertex{pos:(pos.0 - 1, pos.1), straight: straight + 1, d: dir}, map[pos.0-1][pos.1])); 
        }
    }


    result 
}

