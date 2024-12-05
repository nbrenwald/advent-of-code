use std::fs::{read_to_string, File};
use std::io;
use regex::Regex;

pub(crate) fn part1(path: &str) -> io::Result<()> {

    let mut result:i64 = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path)?.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }
    println!("{:?}", matrix);

    for (i,row) in matrix.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if ch.to_owned() == 'X' {
                println!("Starting search at positon ({},{})", i,j);
                if search(i, j, 0, 1,&matrix) { //left
                    result += 1;
                }
                if search(i, j, 0, -1,&matrix) { //right
                    result += 1;
                }
                if search(i, j, -1, 0,&matrix) { //up
                    result += 1;
                }
                if search(i, j, 1, 0,&matrix) { //down
                    result += 1;
                }
                if search(i, j, -1, -1,&matrix) { //diag up left
                    result += 1;
                }
                if search(i, j, -1, 1,&matrix) { //diag up rioght
                    result += 1;
                }
                if search(i, j, 1, -1,&matrix) { //diag down left
                    result += 1;
                }
                if search(i, j, 1, 1,&matrix) { //diag up rioght
                    result += 1;
                }
            }
        }
    }

    println!("{:?}", result);
    Ok(())
}

fn search(x: usize, y: usize, xOffset: i64, yOffset: i64, p3: &Vec<Vec<char>>) -> bool {

    if p3[x][y] != 'X' {
        return false;
    }

    let mut nextX = x as i64 + xOffset;
    let mut nextY = y as i64  + yOffset;

    if nextX < 0 || nextX > (p3.len() as i64 -1 ) || nextY < 0 || nextY > p3[0].len() as i64 -1 || p3[nextX as usize][nextY as usize] != 'M' {
        return false;
    }

    nextX = nextX + xOffset;
    nextY = nextY + yOffset;

    if nextX < 0 || nextX > p3.len() as i64 -1 || nextY < 0 || nextY > p3[0].len() as i64 -1 || p3[nextX as usize][nextY as usize] != 'A' {
        return false;
    }

    nextX = nextX + xOffset;
    nextY = nextY + yOffset;

    if nextX < 0 || nextX > p3.len() as i64 -1 || nextY < 0 || nextY > p3[0].len() as i64 -1 || p3[nextX as usize][nextY as usize] != 'S' {
        return false;
    }
    true
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_search() {
        //assert_eq!(search(0,0, "xmas", ), 19);
    }
}