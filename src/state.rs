extern crate ggez;

use crate::*;
use array;

pub struct GameState {
    pub rules: [bool; 8],
    pub vect: Vec<[bool; ARRAY_LENGTH]>,
}

impl GameState {
    pub fn new(mut ctx: &mut Context, rule: i32) -> GameState {
        GameState {
            rules: array::rule_to_bin(rule), // Generate the Rules
            vect: Vec::new(),
        }
    }

    // Create all the rows need to display
    pub fn create_array(&mut self) {
        let mut first_array: [bool; ARRAY_LENGTH] = [false; ARRAY_LENGTH];

        first_array[first_array.len() / 2] = true; // Set the middle element to true
        self.vect.push(first_array);

        // Grow ARRAY to set size
        for _i in 0..ITERATIONS {
            array::next_line(self);
        }
    }
}

impl EventHandler for state::GameState {
    // Update code here...
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        println!("{}", ggez::timer::fps(ctx));
        Ok(())
    }

    // Draw code here...
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Set screen to white
        ggez::graphics::clear(ctx, ggez::graphics::WHITE);


        // IDK why 600 and 800 are the factors. Maybe its the default and im scaling it when I change the resolution?
        let block_height = 600.0 / ITERATIONS as f32;
        let block_width = 800.0 / ARRAY_LENGTH as f32;

        // Create mesh_builder as it is much faster than creating many Meshs
        let mut mesh_builder = graphics::MeshBuilder::new();

        for (row, array) in self.vect.iter().enumerate() {
            for (count, block) in array.iter().enumerate() {
                if *block {
                    // Create a mesh with a Rectangle. Set it to fill.
                    let rect = ggez::graphics::Rect::new(
                        count as f32 * block_width,
                        row as f32 * block_height,
                        block_width,
                        block_height,
                    );

                    // Append a rectangle to mesh_builder
                    mesh_builder.rectangle(ggez::graphics::DrawMode::fill(), rect, graphics::BLACK);
                }
            }
        }

        // Convert all the meshes to a single mesh
        let meshes = mesh_builder.build(ctx)?;
        // Pass the meshes to be drawn
        ggez::graphics::draw(ctx, &meshes, ggez::graphics::DrawParam::default())?;

        // Display screen
        ggez::graphics::present(ctx)?;
        Ok(())
    }
}
