extern crate ggez;

use crate::imgui::*;
use crate::*;
use array;

pub struct GameState {
    pub imgui_wrapper: ImGuiWrapper,
    pub rules: [bool; 8],
    pub vect: Vec<[bool; ARRAY_LENGTH]>,
}

impl GameState {
    pub fn new(mut ctx: &mut Context, rule: i32) -> GameState {
        GameState {
            rules: array::rule_to_bin(rule), // Generate the Rules
            vect: Vec::new(),
            imgui_wrapper: ImGuiWrapper::new(&mut ctx),
        }
    }

    // Create all the rows need to display
    pub fn create_array(&mut self) {
        let mut first_array: [bool; ARRAY_LENGTH] = [false; ARRAY_LENGTH];

        first_array[first_array.len() / 2] = true; // Set the middle element to true
        self.vect.push(first_array);

        // Grow ARRAY
        for _i in 0..ITERATIONS {
            array::next_line(self);
        }
    }
}

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

        for (row, array) in self.vect.iter().enumerate() {
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
