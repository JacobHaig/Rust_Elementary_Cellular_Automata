// Create by Jacob Haig
// Written in Rust
// Updated 9/8/2019
// https://github.com/JacobHaig/Rust-Elementary-Cellular-Automata/

extern crate rand;
use rand::Rng;

use ggez::event::{self, EventHandler};
use ggez::graphics::*;
use ggez::*;

const RULE: i32 = 57;
const ARRAY_LENGTH: usize = 81;
const ITERATIONS: usize = 80;

struct GameState {
    rules: [bool; 8],
    array: Vec<Vec<bool>>,
}

// This is for generating interesting patterns
#[allow(dead_code)]
fn rand_array(array: &mut [bool], p: f64) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    for b in array.iter_mut() {
        *b = rng.gen_bool(p);
    }
}

// This allows the grid to wrap. This is more accurate than constant-value edge cells
fn wrap(mut n: isize) -> usize {
    const LEN: isize = ARRAY_LENGTH as isize;

    if n < 0 {
        n += LEN;
    }
    if n > LEN - 1 {
        n -= LEN;
    }

    n as usize
}

fn get_rule_index(array: &[bool], index: usize) -> usize {
    let left_cell = (array[wrap(index as isize - 1)]) as usize * 4;
    let mid_cell = (array[wrap(index as isize)]) as usize * 2;
    let right_cell = (array[wrap(index as isize + 1)]) as usize;

    // Binary 4, 2, 1. Adding them gives index into rules. ie "⬜⬛⬜" is 2
    left_cell + mid_cell + right_cell
}

// Logic for creating new array.
fn next_line(array: &[bool], rules: &[bool]) -> Vec<bool> {
    let mut array2 = vec![false; ARRAY_LENGTH];

    for (i, _b) in array.iter().enumerate() {
        let index: usize = get_rule_index(array, i);

        // Depending on the arrangement of cell values, the rules decide the next value.
        array2[i] = rules[index];
    }

    array2
}

// Create rule array for comparing later
#[rustfmt::skip]
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

impl GameState {
    pub fn new(_ctx: &mut Context, rule: i32) -> GameState {
        GameState {
            rules: rule_to_bin(rule), // Generate the Rules
            array: Vec::new(),
        }
    }

    // Create all the rows need to display
    pub fn create_array(&mut self) {
        let mut first_array: Vec<bool> = vec![false; ARRAY_LENGTH];
        first_array[ARRAY_LENGTH / 2] = true; // Set the middle element to true
        self.array.push(first_array);

        for _i in 0..ITERATIONS {
            let new_array = next_line(&self.array.last().unwrap(), &self.rules);
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
        //let window = graphics::window(ctx);

        // IDK why 600 and 800 are the factors. Maybe its the default and im scaling it when I change the resolution?
        let  block_height = 600.0 / ITERATIONS as f32;
        let  block_width = 800.0 / ARRAY_LENGTH as f32;

        // Set screen to white
        graphics::clear(ctx, graphics::WHITE);

        for (row, array) in self.array.iter().enumerate() {
            for (count, block) in array.iter().enumerate() {
                if *block {
                    // Create a mesh with a Rectangle. Set it to fill.
                    let mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        graphics::Rect::new(
                            count as f32 * block_width,
                            row as f32 * block_height,
                            block_width,
                            block_height,
                        ),
                        graphics::BLACK,
                    )?;

                    // Pass the mesh to be drawn
                    graphics::draw(ctx, &mesh, DrawParam::default())?;
                }
            }
        }

        // Display screen
        graphics::present(ctx)?;
        Ok(())
    }
}

// MAIN!
fn main() {
    // Configuration
    let win_mode = ggez::conf::WindowMode {
        width: 800.0,
        height: 600.0,
        maximized: false,
        resizable: true,
        borderless: false,

        min_width: 100.0,
        min_height: 100.0,
        max_width: 0.0,
        max_height: 0.0,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
    };

    // Context
    let cb = ContextBuilder::new("Rust Elementary Cellular Automata", "Wisward");
    let (mut ctx, mut event_loop) = cb.build().expect("Could not create context!");
    graphics::set_window_title(&ctx, "Rust Elementary Cellular Automata");
    graphics::set_mode(&mut ctx, win_mode).expect("Can't set window mode.");

    // State
    let mut game_state = GameState::new(&mut ctx, RULE);
    game_state.create_array();

    // Run Events
    match event::run(&mut ctx, &mut event_loop, &mut game_state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
