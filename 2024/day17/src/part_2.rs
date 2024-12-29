use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, GridPosition, GridPositionAndDirection, Position};


pub(crate) fn run(a:&mut u64, b: &mut u64, c: &mut u64, program:Vec<u64>) -> i32{
    let mut result:i32 = i32::MAX;

    let program_string: String = format!(",{}",program.iter().map(|x|format!("{}", x)).collect::<Vec<_>>().join(","));

    let mut i:u64 = 74280637;
    let mut j = 0;

    let deltas: [u64; 5] = [256, 4194048, 256, 2427384, 1766664];
    for x in 0..1000000000 as u64 {

        i = (i + deltas[j]);
        j = (j + 1) % deltas.len();

        //println!("{}",i);
        let mut out: String = String::new();

        *a = i;
        let mut instruction_ptr: usize = 0;
        while instruction_ptr < program.len() - 1 {
            //println!("Processing {}", program[instruction_ptr]);

            match program[instruction_ptr] {
                0 => { *a = *a / (u64::pow(2, get_combo_operand(program[instruction_ptr + 1], *a, *b, *c) as u32)); }
                1 => { *b = *b ^ program[instruction_ptr + 1]; }
                2 => { *b = get_combo_operand(program[instruction_ptr + 1], *a, *b, *c) % 8; }
                3 => {
                    if *a != 0 {
                        instruction_ptr = program[instruction_ptr + 1] as usize;
                        continue;
                    }
                }
                4 => { *b = *b ^ *c }
                5 => { out = format!("{},{}", out, get_combo_operand(program[instruction_ptr + 1], *a, *b, *c) % 8); }
                7 => { *c = *a / (u64::pow(2, get_combo_operand(program[instruction_ptr + 1], *a, *b, *c) as u32)); }
                _ => {}
            }

            instruction_ptr += 2;
            //println!("Register A : {:?}", a);
            //println!("Register B : {:?}", b);
            //println!("Register C : {:?}", c);

        }
        //println!("out={}", out);
        //println!("Result = {}", out);
        if out.len() > 27 && out[0..27] == program_string[0..27] {
            println!("{}", i);
            //println!("out={}", out);
            //println!("Result = {}", out);
            //break;
        }
        //println!("out={}", out);
        //println!("Result = {}", out);
        if out == program_string {
            println!("Result: {}", i);
            break;
        }

    }
result
}





fn get_combo_operand(combo_operand:u64, a:u64, b: u64, c:u64) -> u64 {
    if combo_operand >= 0 && combo_operand <= 3 {return combo_operand;}
    if combo_operand == 4 { return  a; }
    if combo_operand == 5 { return  b; }
    if combo_operand == 6 { return  c; }
    return 0;
}

