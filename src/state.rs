extern crate ggez;

use crate::imgui_wrapper::*;
use crate::*;
use array;

pub struct GameState {
    pub imgui_wrapper: ImGuiWrapper,
    pub rules: [bool; 8],
    pub array: Vec<Vec<bool>>,
}

impl GameState {
    pub fn new(mut ctx: &mut Context, rule: i32) -> GameState {
        GameState {
            rules: array::rule_to_bin(rule), // Generate the Rules
            array: Vec::new(),
            imgui_wrapper: ImGuiWrapper::new(&mut ctx),
        }
    }

    // Create all the rows need to display
    pub fn create_array(&mut self) {
        let mut first_array: Vec<bool> = vec![false; ARRAY_LENGTH];
        first_array[ARRAY_LENGTH / 2] = true; // Set the middle element to true
        self.array.push(first_array);

        for _i in 0..ITERATIONS {
            let new_array = array::next_line(&self.array.last().unwrap(), &self.rules);
            self.array.push(new_array);
        }
    }
}
