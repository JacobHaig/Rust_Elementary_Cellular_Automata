// Create by Jacob Haig
// Written in Rust
// Updated 9/10/2019
// https://github.com/JacobHaig/Rust-Elementary-Cellular-Automata/

mod array;
mod imgui_wrapper;
mod state;

extern crate ggez;
use ggez::event::*;
use ggez::*;

const RULE: i32 = 57;
const ARRAY_LENGTH: usize = 81;
const ITERATIONS: usize = 80;

impl EventHandler for state::GameState {
    // Update code here...
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    // Draw code here...
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //let window = graphics::window(ctx);

        // IDK why 600 and 800 are the factors. Maybe its the default and im scaling it when I change the resolution?
        let block_height = 600.0 / ITERATIONS as f32;
        let block_width = 800.0 / ARRAY_LENGTH as f32;

        // Set screen to white
        ggez::graphics::clear(ctx, ggez::graphics::WHITE);

        for (row, array) in self.array.iter().enumerate() {
            for (count, block) in array.iter().enumerate() {
                if *block {
                    // Create a mesh with a Rectangle. Set it to fill.
                    let mesh = ggez::graphics::Mesh::new_rectangle(
                        ctx,
                        ggez::graphics::DrawMode::fill(),
                        ggez::graphics::Rect::new(
                            count as f32 * block_width,
                            row as f32 * block_height,
                            block_width,
                            block_height,
                        ),
                        ggez::graphics::BLACK,
                    )?;

                    // Pass the mesh to be drawn
                    ggez::graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::default())?;
                }
            }
        }

        // Render game ui
        {
            self.imgui_wrapper.render(ctx);
        }
        // Display screen
        ggez::graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::P => {
                self.imgui_wrapper.open_popup();
            }
            _ => (),
        }
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
    ggez::graphics::set_window_title(&ctx, "Rust Elementary Cellular Automata");
    ggez::graphics::set_mode(&mut ctx, win_mode).expect("Can't set window mode.");

    // State
    let mut game_state = state::GameState::new(&mut ctx, RULE);
    game_state.create_array();

    // Run Events
    match ggez::event::run(&mut ctx, &mut event_loop, &mut game_state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
