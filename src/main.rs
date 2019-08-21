// Create by Jacob Haig
// Written in Rust
// Date 8/5/2019
// https://github.com/JacobHaig/Rust-Elementary-Cellular-Automata/

#![allow(non_snake_case)]
extern crate rand;
use rand::Rng;

const TEXT_TYPE     : TextType = TextType::UnicodeBlock; // Set true if working in the console
const ARRAY_LENGTH  : usize = 31;
const ITERATIONS    : usize = 15;

// This enum allows for differant types of text to be printed
#[allow(dead_code)]
enum TextType {
    AsciiBlock,
    UnicodeBlock,
    UnicodeAnimals,
}

// This function selects the text based of selected ENUM
fn select_block(text: TextType, is_solid: bool) -> &'static str {
    match text {
        TextType::AsciiBlock    => if is_solid { "█" } else { " " },
        TextType::UnicodeBlock  => if is_solid { "⬛" } else { "⬜" },
        TextType::UnicodeAnimals  => if is_solid { "🐶" } else { "😺" },
    }
}

// Print a single char which represents the "cell"
fn print_char(is_solid: bool) {
    print!(
        "{}",
        select_block(TEXT_TYPE, is_solid)
    )
}

// Loop over the array and print each cell
fn print_array(array: &[bool]) {
    for b in array {
        print_char(*b);
    }
    println!();
}

// This is for generating interesting patterns
#[allow(dead_code)]
fn rand_array(array: &mut [bool], p: f64) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    for b in array.iter_mut() {
        *b = rng.gen_bool(p);
    }
}

// Transfer the temp array to the first
fn transfer(array: &mut [bool], array2: &[bool]) {
    for i in 0..ARRAY_LENGTH {
        array[i as usize] = array2[i as usize];
    }
}

// This allows the grid to wrap. This is more accurate than constant-value edge cells
fn wrap(mut n: isize) -> usize {
    if n < 0 {
        n += ARRAY_LENGTH as isize;
    }
    if n > ARRAY_LENGTH  as isize - 1 {
        n -= ARRAY_LENGTH as isize;
    }

    n as usize
}

// Logic for creating new array.
fn next_line(mut array: &mut [bool], rules: &[bool]) {
    let mut array2: [bool; ARRAY_LENGTH] = [false; ARRAY_LENGTH];

    for (i, _b) in array.iter().enumerate() {
        let left_cell = (array[wrap(i as isize - 1)]) as usize * 4;
        let mid_cell = (array[wrap(i as isize)]) as usize * 2;
        let right_cell = (array[wrap(i as isize + 1)]) as usize;

        // Binary 4, 2, 1. Adding them gives index into rules. ie "⬜⬛⬜" is 2
        let index: usize = left_cell + mid_cell + right_cell;
        //print!("{}", index); //Print the rule's index

        // Depending on the arrangement of cell values, the rules decide the next value.
        array2[i] = rules[index];
    }

    transfer(&mut array, &array2);
}

// Create rule array for comparing later
fn rule_to_bin(mut n: i32) -> [bool; 8] {
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

fn main() {
    for rule_number in 0..256 {
        print!("\nPrinting Rule {}\n", rule_number);
        let rules = rule_to_bin(rule_number);

        let mut array: [bool; ARRAY_LENGTH] = [false; ARRAY_LENGTH];
        array[ARRAY_LENGTH / 2] = true;
        //rand_array(&mut array, 0.5);

        for _i in 0..ITERATIONS {
            print_array(&array);
            next_line(&mut array, &rules);
        }
    }
}
