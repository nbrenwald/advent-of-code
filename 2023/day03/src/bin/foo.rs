use std::fs::read_to_string;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    const rows: usize = 143;
    const cols: usize = 141;

    let mut touches = [[false; cols] ; rows];

    let mut l = 0;
    for line in read_to_string("sample.txt").unwrap().lines() {
       let line_str = line.to_string();
       let mut c = 0;
        for i in line_str.chars() {
            if i != '.' {
                //println!("found symbol {}", i);
                if !i.is_numeric() {
                    touches[l][c] = true;
                    if l>0 {
                        if c >0 {
                           touches[l-1][c-1] = true;
                        }//diagnoal top left
                    touches[l-1][c] = true; //above                          //t
                    touches[l-1][c+1] = true;
                    }
                    
                    if c >0 {
                    touches[l][c-1] = true; //diagnoal top left
                    }
                    touches[l][c] = true; //above                          //t
                    touches[l][c+1] = true;

                    if c > 0 {
                        touches[l+1][c-1] = true; //diagnoal top left
                    }
                        touches[l+1][c] = true; //above                          //t
                    touches[l+1][c+1] = true;
                }
            }
            c += 1;
        }
        l += 1;
    }

    let mut l = 0;
    let mut buf:String = "".to_string();
    let mut touchesNumber = false;
    let mut total: usize = 0;
    for line in read_to_string("sample.txt").unwrap().lines() {
       let line_str = line.to_string();
       let mut c = 0;

                  if touchesNumber == true {
                     let v = buf.parse::<usize>().unwrap();
                      total += v;
                      println!("adding {}", v);
                      buf = "".to_string();
                      touchesNumber=false;
                  }
                  //println!("Adding {}, total now {}", v, total);
                      


        for i in line_str.chars() {
            if i.is_numeric() {
                //println!("checking row={} col={} touches[{}][{}]={}", l, c, l, c, touches[l][c]);
                if touches[l][c] == true {
                    touchesNumber = true;
                }
                buf.push(i);
            }
            else {
                if buf.len() != 0 {

                 // println!("{} and touchesNumber={}", buf, touchesNumber);
                  if touchesNumber == true {
                     let v = buf.parse::<usize>().unwrap();
                      total += v;
                      println!("adding {}", v);
                  //println!("Adding {}, total now {}", v, total);
                      
                  }
                  buf = "".to_string();
                  touchesNumber = false;
                }
            }
            c += 1;
        }
        l += 1;
    }
    //touches.iter().for_each(|it| {
    //         println!("{:#?}", it);
    //});
    println!("{}", total);
Ok(())
}
