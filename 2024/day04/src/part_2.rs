use std::fs::{read_to_string, File};
use std::io;
use regex::Regex;

pub(crate) fn part2(path: &str) -> io::Result<()> {

    let mut result:i64 = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path)?.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }
    println!("{:?}", matrix);

    for (i,row) in matrix.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if ch.to_owned() == 'A' {
                println!("Starting search at positon ({},{})", i,j);
                if (
                    (i > 0 && j > 0 && search(i-1, j-1, 1, 1,&matrix) )
                        ||
                        search(i+1, j+1, -1, -1,&matrix)
                )
                    &&
                    (
                        (i > 0 && search(i-1, j+1, 1, -1,&matrix) )
                            ||
                            (j > 0 &&  search(i+1, j-1, -1, 1,&matrix))
                    ){ //left
                    println!("Found a valid XMAS");
                    result += 1;
                }
            }
        }
    }

    println!("{:?}", result);
    Ok(())
}

fn search(x: usize, y: usize, xOffset: i64, yOffset: i64, p3: &Vec<Vec<char>>) -> bool {

    let mut nextX = x as i64 ;
    let mut nextY = y as i64 ;

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