extern crate rand;
use crate::state::GameState;
use crate::*;
use rand::Rng;

// This is for generating interesting patterns
#[allow(dead_code)]
pub fn rand_array(array: &mut [bool], p: f64) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    for b in array.iter_mut() {
        *b = rng.gen_bool(p);
    }
}

// This allows the grid to wrap. This is more accurate than constant-value edge cells
#[rustfmt::skip]
pub fn wrap(mut n: isize) -> usize {
    const LEN: isize = ARRAY_LENGTH as isize;

    if n < 0       { n += LEN; }
    if n > LEN - 1 { n -= LEN; }

    n as usize
}

pub fn get_rule_index(array: &[bool], index: usize) -> usize {
    let left_cell = (array[wrap(index as isize - 1)]) as usize * 4;
    let mid_cell = (array[wrap(index as isize)]) as usize * 2;
    let right_cell = (array[wrap(index as isize + 1)]) as usize;

    // Binary 4, 2, 1. Adding them gives index into rules. ie "⬜⬛⬜" is 2
    left_cell + mid_cell + right_cell
}

// Logic for creating new vect.
pub fn next_line(state: &mut GameState) {
    let mut array: [bool; ARRAY_LENGTH] = [false; ARRAY_LENGTH];
    let last = state.vect.last().unwrap();

    for (i, _b) in last.iter().enumerate() {
        let index: usize = get_rule_index(last, i);

        // Depending on the arrangement of cell values, the rules decide the next value.
        array[i] = state.rules[index];
    }

    state.vect.push(array);
}

// Create rule array for comparing later
#[rustfmt::skip]
pub fn rule_to_bin(mut n: i32) -> [bool; 8] {
    let mut rules: [bool; 8] = [false; 8];

    // Decode int in to binary array.
    if n >= 128 { n -= 128; rules[7] = true; }
    if n >= 64  { n -= 64;  rules[6] = true; }
    if n >= 32  { n -= 32;  rules[5] = true; }
    if n >= 16  { n -= 16;  rules[4] = true; }
    if n >= 8   { n -= 8;   rules[3] = true; }
    if n >= 4   { n -= 4;   rules[2] = true; }
    if n >= 2   { n -= 2;   rules[1] = true; }
    if n >= 1   { n -= 1;   rules[0] = true; }

    if n != 0 { panic!("Error: rules not set correctly! Rule number must be from 0 to 255"); }

    rules
}

#[allow(dead_code)]
fn print_array(array: &[bool]) {
    for b in array {
        print!("{}", if *b { "⬛" } else { "⬜" });
    }
    println!();
}
