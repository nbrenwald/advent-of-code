use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, print_grid, GridPosition, GridPositionAndDirection, Position, State};


pub(crate) fn run(wires: &mut BTreeMap<String, usize>, old_calcs:&Vec<(String, String, String, String)>, total: usize) {
let mut calcs: Vec<(String, String, String, String)> = Vec::new();
    for c in old_calcs {
        if c.3 == "nnt" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "gws".to_string()));
        }
        else if c.3 == "gws" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "nnt".to_string()));
        }
        else if c.3 == "npf" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "z13".to_string()));
        }
        else if c.3 == "z13" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "npf".to_string()));
        }
        else if c.3 == "z19" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "cph".to_string()));
        }
        else if c.3 == "cph" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "z19".to_string()));
        }
        else if c.3 == "hgj" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "z33".to_string()));
        }
        else if c.3 == "z33" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "hgj".to_string()));
        }/*
        else if c.3 == "wtm" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "wgq".to_string()));
        }
        else if c.3 == "wgq" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "wtm".to_string()));
        }
        else if c.3 == "z19" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "fnq".to_string()));
        }
        else if c.3 == "fnq" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "z19".to_string()));
        }
        else if c.3 == "z13" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "kvr".to_string()));
        }
        else if c.3 == "kvr" {
            calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), "z13".to_string()));
        }*/
        else {calcs.push((c.0.clone(), c.1.clone(), c.2.clone(), c.3.clone()));}

    }

    for index in 0 ..45 {
        println!("Checking bit {}", index);
        for i in 0..2 {
            for j in 0..2 {
                let mut new_wires = BTreeMap::new();

                for (k, v) in &mut *wires {

                    if k[1..].parse::<usize>().unwrap() == index && k.starts_with('x') {

                        new_wires.insert(k.clone(), i);
                    }
                    else if k[1..].parse::<usize>().unwrap() == index && k.starts_with('y') {

                        new_wires.insert(k.clone(), j);
                    }
                    else { new_wires.insert(k.clone(), 0);
                    }
                }

               let result =  run2(&mut new_wires, &calcs, total);
                if (i==0 && j ==0){
                    if *result.get(result.len()-1-index).unwrap() != 0 {
                        println!("Broken bit={} x={} y={}", index, i, j);
                    }
                }
                else if (i==1 && j ==1){
                    if *result.get(result.len()-1-index).unwrap() != 0 ||  *result.get(result.len()-2-index).unwrap() != 1 {
                        println!("Broken bit={} x={} y={}", index, i, j);
                    }
                }
                else if (i==1 && j ==0) || (i==0 && j==1){
                    if *result.get(result.len()-1-index).unwrap() != 1 {
                        println!("Broken bit={} x={} y={}", index, i, j);
                    }
                }
            }
        }
    }

    print_calcs(&calcs);
}

pub(crate) fn run2(wires: &mut BTreeMap<String, usize>, calcs:&Vec<(String, String, String, String)>, total: usize) -> VecDeque<usize>{

        let mut result = 0;

        while (wires.len() != total) {
            //for i in 0..10 {
            //println!("result {}:", result);

            for calc in calcs.iter() {
                //can i resolve calc
                //println!("Checking {:?}", calc);
                if !wires.contains_key(&calc.3) && wires.contains_key(&calc.0) && wires.contains_key(&calc.1) {
                    let in1 = wires[&calc.0];
                    let in2 = wires[&calc.1];
                    let op = &calc.2;
                    let out = &calc.3;
                    let mut out_result: usize = 0;
                    //println!("Calculating {}", out);

                    if *op == String::from("OR") {
                        if in1 == 1 || in2 == 1 { out_result = 1; } else { out_result = 0; }
                    } else if *op == String::from("XOR") {
                        //println!("{}{}", in1, in2);
                        if in1 != in2 { out_result = 1; } else { out_result = 0; }
                        // println!("{} {} {}", in1, in2, out_result);
                    } else if *op == String::from("AND") {
                        if in1 == 1 && in2 == 1
                        { out_result = 1; } else { out_result = 0; }
                    }
                    if out.starts_with("z") {
                        //println!("Found an answer for {}", out);
                    }
                    wires.insert(out.to_string(), out_result);
                }
            }
        }
        //println!("result {:?}:", wires);
        let mut xbstr: VecDeque<usize> = VecDeque::new();
        for (k, v) in wires.iter() {
            if k.starts_with('x') {
                //println!("{} {}", k, v);
                xbstr.push_front(*v);
            }
        }
        xbstr.push_front(0);
        let mut ybstr: VecDeque<usize> = VecDeque::new();
        for (k, v) in wires.iter() {
            if k.starts_with('y') {
                //println!("{} {}", k, v);
                ybstr.push_front(*v);
            }
        }
        ybstr.push_front(0);
        let mut zbstr: VecDeque<usize> = VecDeque::new();
        for (k, v) in wires.iter() {
            if k.starts_with('z') {
                //println!("{} {}", k, v);
                zbstr.push_front(*v);
            }
        }
        println!("result {:?} = {}", xbstr, result);
        println!("result {:?} = {}",ybstr,  result);
        println!("result {:?} = {}",zbstr,  result);
        result = i64::from_str_radix(zbstr.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join("").as_str(), 2).expect("Not a binary number!");
        //println!("result {:?} = {}",zbstr,  result);
        //6409 too low
    zbstr

}

fn print_calcs(calcs:&Vec<(String, String, String, String)>) {
for (i, calc) in calcs.iter().enumerate() {
    print!("{} -> {};", calc.0, i);
    print!("{} -> {};", calc.1, i);
    print!("{} -> {};", i, calc.3);
    print!("{} [label=\"{}\"];", i, calc.2);
}
}