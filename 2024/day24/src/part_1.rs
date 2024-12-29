use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(wires: &mut BTreeMap<String, usize>, calcs:&Vec<(String, String, String, String)>, total: usize) {
    let mut result = 0;

    while (wires.len() != total)  {
        //for i in 0..10 {
        //println!("result {}:", result);

        for calc in calcs.iter() {
            //can i resolve calc
            println!("Checking {:?}", calc);
            if !wires.contains_key(&calc.3) && wires.contains_key(&calc.0) && wires.contains_key(&calc.1) {

                let in1 = wires[&calc.0];
                let in2 = wires[&calc.1];
                let op = &calc.2;
                let out = &calc.3;
                let mut out_result: usize  = 0;
                println!("Calculating {}", out);

                if *op == String::from("OR") {
                    if in1 == 1 || in2 == 1 { out_result = 1; } else { out_result=0; }
                }
                else if *op == String::from("XOR") {
                    println!("{}{}", in1, in2);
                    if in1 != in2 { out_result = 1; } else { out_result = 0; }
                    println!("{} {} {}", in1, in2, out_result);
                }
                else if *op == String::from("AND")  {
                    if in1 == 1 && in2 == 1
                    { out_result = 1; } else { out_result = 0; }
                }
                println!("Found an answer for {}", out);
                wires.insert(out.to_string(), out_result);
            }
        }
    }
    println!("result {:?}:", wires);
    let mut bstr:VecDeque<usize> = VecDeque::new();
    for (k, v) in wires.iter() {
        if k.starts_with('z') {
            println!("{} {}", k, v);
            bstr.push_front(*v);

        }
    }
    println!("result {:?} = {}",bstr,  result);
    result = i64::from_str_radix(bstr.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join("").as_str(), 2).expect("Not a binary number!");
    println!("result {:?} = {}",bstr,  result);
    //6409 too low

}
