// Create by Jacob Haig
// Written in Rust
// Updated 9/10/2019
// https://github.com/JacobHaig/Rust-Elementary-Cellular-Automata/

mod array;
mod state;

extern crate ggez;
use ggez::event::*;
use ggez::*;

const RULE: i32 = 57;
const ARRAY_LENGTH: usize = 201;
const ITERATIONS: usize = 400;

// MAIN!
fn main() {
    // Configuration
    let win_mode = ggez::conf::WindowMode {
        height: 1200.0,
        width: 600.0,
        resizable: true,

        min_height: 100.0,
        min_width: 100.0,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        ..Default::default()
    };

    // Context
    let cb = ContextBuilder::new("Rust Elementary Cellular Automata", "Wisward");
    let (mut ctx, event_loop) = cb.build().expect("Could not create context!");
    ggez::graphics::set_window_title(&ctx, "Rust Elementary Cellular Automata");
    ggez::graphics::set_mode(&mut ctx, win_mode).expect("Can't set window mode.");

    // State
    let mut game_state = state::GameState::new(&mut ctx, RULE);
    game_state.create_array();

    // Run Events
    ggez::event::run(ctx, event_loop, game_state);
}
