// Create by Jacob Haig
// Written in Rust
// Date 9/8/2019
// https://github.com/JacobHaig/Rust-Elementary-Cellular-Automata/

#![allow(non_snake_case)]

extern crate rand;
use rand::Rng;

use ggez::event::{self, EventHandler};
use ggez::*;
use ggez::graphics::*;

const CONSOLE: bool = false;
const ARRAY_LENGTH: usize = 31;
const ITERATIONS: usize = 15;

// Print a single char which represents the "cell"
#[allow(dead_code)]
fn print_char(block: bool) {
    print!(
        "{}",
        if CONSOLE {
            if block {"█"} else {" "}
        } else
            if block {"⬛"} else {"⬜"}
    )
    // "█" or "⬛" and " " or "⬜"
}

// Loop over the array and print each cell
#[allow(dead_code)]
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
#[allow(dead_code)]
fn transfer(array: &mut [bool], array2: &[bool]) {
    for i in 0..ARRAY_LENGTH {
        array[i as usize] = array2[i as usize];
    }
}

// This allows the grid to wrap. This is more accurate than constant-value edge cells
fn wrap(mut n: isize) -> usize {
    if n < 0 { n += ARRAY_LENGTH as isize; }
    if n > ARRAY_LENGTH as isize - 1 { n -= ARRAY_LENGTH as isize; }

    n as usize
}

// Logic for creating new array.
fn next_line( array: &[bool], rules: &[bool]) -> Vec<bool>{
    let mut array2 = vec![false; ARRAY_LENGTH];
    //let mut array2: [bool; ARRAY_LENGTH] = [false; ARRAY_LENGTH];

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

    //transfer(&mut array, &array2);
    array2
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
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    let mut game_state = GameState::new(&mut ctx, 225);
    game_state.create_array();

    match event::run(&mut ctx, &mut event_loop, &mut game_state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct GameState {
    rules: [bool; 8],
    array: Vec<Vec<bool>>,
}

// Load/create resources such as images here.
impl GameState {
    pub fn new(_ctx: &mut Context, rule: i32) -> GameState {
        GameState {
            rules: rule_to_bin(rule),
            array: Vec::new(),
        }
    }

    pub fn create_array(&mut self) {
        let mut first_array: Vec<bool> = vec![false; ARRAY_LENGTH];
        first_array[ARRAY_LENGTH / 2] = true;

        self.array.push(first_array);

        for _i in 0..ITERATIONS {
            let new_array = next_line(& self.array.last().unwrap(), & self.rules);
            self.array.push(new_array);
        }
    }
}

impl EventHandler for GameState {
    // Update code here...
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

     // Draw code here...
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let window = graphics::window(ctx).get_inner_size().unwrap();
        let block_size = window.width as f32 / ARRAY_LENGTH as f32;

        graphics::clear(ctx, graphics::WHITE);


        for (row, array) in self.array.iter().enumerate() {
            for (count,block) in array.iter().enumerate() {
                if *block {

                    let rect = graphics::Rect::new(count as f32 * block_size, row as f32 * block_size, block_size, block_size);
                    let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(),rect, graphics::BLACK)?;
                    graphics::draw(ctx, &mesh,DrawParam::default())?;
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}