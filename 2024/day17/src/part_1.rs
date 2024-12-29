use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use utils::{direction, GridPosition, GridPositionAndDirection, Position};


pub(crate) fn run(a:&mut u32, b: &mut u32, c: &mut u32, program:Vec<u32>) -> i32{
    let mut result:i32 = i32::MAX;
    let mut out: String = String::new();

    let mut instruction_ptr:usize = 0;
    while instruction_ptr < program.len()-1 {
        println!("Processing {}", program[instruction_ptr]);

        match program[instruction_ptr] {
            0 => {*a = *a / (u32::pow(2,get_combo_operand(program[instruction_ptr + 1], *a, *b, *c))); }
            1 => {*b = *b ^ program[instruction_ptr + 1];}
            2 => { *b = get_combo_operand(program[instruction_ptr + 1], *a, *b, *c) % 8; }
            3 => {if *a != 0 {instruction_ptr = program[instruction_ptr + 1] as usize; continue;}}
            4 => {*b = *b ^ *c}
            5 => { out  = format!("{},{}",out, get_combo_operand(program[instruction_ptr + 1], *a, *b, *c) % 8);}
            7 => {*c = *a / (u32::pow(2,get_combo_operand(program[instruction_ptr + 1], *a, *b, *c))); }
            _ => {}
        }

        instruction_ptr += 2;
        println!("Register A : {:?}", a);
        println!("Register B : {:?}", b);
        println!("Register C : {:?}", c);

    }
println!("Result = {}", out);
result
}

/*fn bst(combo_operand: u32, a:u32, b:&mut u32, c:u32) {
    let literal_operand = get_combo_operand(combo_operand, a, b, c);
    *b =  literal_operand % 8;

}

fn out(combo_operand: u32, a:u32, b:&mut u32, c:u32) {
    let literal_operand = get_combo_operand(combo_operand, a, b, c);
    literal_operand % 8;

}*/



fn get_combo_operand(combo_operand:u32, a:u32, b: u32, c:u32) -> u32 {
    if combo_operand >= 0 && combo_operand <= 3 {return combo_operand;}
    if combo_operand == 4 { return  a; }
    if combo_operand == 5 { return  b; }
    if combo_operand == 6 { return  c; }
    return 0;
}

