use std::fs::read_to_string;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

fn processMatch(gears: &mut HashMap<usize, Vec<usize>>, v: usize, gearId: usize) {

    let vec = gears.entry(gearId).or_insert(Vec::new());
    vec.push(v);
    println!("adding {}", v);
}

fn main() -> io::Result<()> {

    const rows: usize = 143;
    const cols: usize = 141;

    let mut touches = [[0usize; cols] ; rows];

    let mut l = 0;
    let mut gearId = 1;
    for line in read_to_string("sample.txt").unwrap().lines() {
       let line_str = line.to_string();
       let mut c = 0;
        for i in line_str.chars() {
            if i == '*' {
                //println!("found symbol {}", i);
                if !i.is_numeric() {
                    touches[l][c] = gearId;
                    if l>0 {
                        if c >0 {
                           touches[l-1][c-1] = gearId;
                        }//diagnoal top left
                    touches[l-1][c] = gearId; //above                          //t
                    touches[l-1][c+1] = gearId;
                    }
                    
                    if c >0 {
                    touches[l][c-1] = gearId; //diagnoal top left
                    }
                    touches[l][c] = gearId; //above                          //t
                    touches[l][c+1] = gearId;

                    if c > 0 {
                        touches[l+1][c-1] = gearId; //diagnoal top left
                    }
                        touches[l+1][c] = gearId; //above                          //t
                    touches[l+1][c+1] = gearId;
                    gearId += 1;
                }
            }
            c += 1;
        }
        l += 1;
    }
    
    let mut gears: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut l = 0;
    let mut buf:String = "".to_string();
    let mut touchesNumber = false;
    let mut total: usize = 0;
    for line in read_to_string("sample.txt").unwrap().lines() {
       let line_str = line.to_string();
       let mut c = 0;

                  if touchesNumber == true { //have a number from a previous row
                     let v = buf.parse::<usize>().unwrap();
                     processMatch(&mut gears, v, gearId); //total += v;
                      println!("adding {}", v);
                      buf = "".to_string();
                      touchesNumber=false;
                  }
                  //println!("Adding {}, total now {}", v, total);
                      

        for i in line_str.chars() {
            if i.is_numeric() {
                //println!("checking row={} col={} touches[{}][{}]={}", l, c, l, c, touches[l][c]);
                if touches[l][c] != 0 {
                    touchesNumber = true;
                    gearId = touches[l][c];
                }
                buf.push(i);
            }
            else {
                if buf.len() != 0 {

                 //s!("{} and touchesNumber={}", buf, touchesNumber);
                  if touchesNumber == true {
                     let v = buf.parse::<usize>().unwrap();
                     processMatch(&mut gears, v, gearId)
                  //println!("Adding {}, total now {}", v, total);
                  }
                  buf = "".to_string();
                  touchesNumber = false;
                  gearId = 0;
                }
            }
            c += 1;
        }
        l += 1;
    }
    //touches.iter().for_each(|it| {
    //         println!("{:#?}", it);
    //});
    for (key, value) in gears {
        if value.len() > 1 {
        total += value[0] * value[1];
    }}
    println!("{}", total);
Ok(())
}
