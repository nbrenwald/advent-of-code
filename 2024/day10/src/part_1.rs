use std::collections::HashSet;
use utils::Position;

pub(crate) fn run(map: &Vec<Vec<char>>, trail_heads: &Vec<Position>){

    let mut result:u32 = 0;

    for (i,trail_head) in trail_heads.iter().enumerate() {
        if true {
            println!("Starting {:?}", trail_head);
            let mut positions: HashSet<Position> = HashSet::new();
            let r = backtrack(map, trail_head, -1, &mut positions);
            println!("HashSet Size: {:?}", positions.len());
            println!("R: {:?}", r);
           result += positions.len() as u32;
    }

    println!("Result: {}", result);

}}

fn backtrack(map: &Vec<Vec<char>>, p: &Position, x: i32, positions: &mut HashSet<Position>) -> u32 {

    println!("Checking position P={:?}", p);

    if p.x < 0 || p.y < 0 || p.x >= map[0].len() as i32 || p.y >= map.len() as i32 {
        println!("HALT - P IS OUT OF BOUNDS");
        return 0;
    }


    let vChar = *map.get(p.y as usize).unwrap().get(p.x as usize).unwrap();
    if vChar == '.' {
        println!("HALT - P IS NOT NUMBER");
        return 0;}
    let v = vChar.to_digit(10).unwrap() as i32;

    println!("V of P is {}", v);
    println!("delta is {}", v as i32 -x);
    if v as i32 - x != 1 {
        println!("HALT - NOT GOING UP BY 1");
        return 0;}
    if v == 9 {
        println!("SUCESS - GOT TO 9");
        positions.insert(p.clone());
        return 1;}

    backtrack(map, &Position {x: p.x+1, y: p.y}, v, positions ) +
        backtrack(map, &Position {x: p.x-1, y: p.y}, v, positions) +
        backtrack(map, &Position {x: p.x, y: p.y+1}, v, positions) +
        backtrack(map,
                  &Position {x: p.x, y: p.y-1}, v,
                  positions)
}