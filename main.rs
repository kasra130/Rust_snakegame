extern crate piston_window;
extern crate rand;

mod shapes1;
mod The_snake;
mod rule;

use piston_window::*;
use piston_window::types::Color;

use crate::rule::Rule;
use crate::shapes1::to_coord_u32;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 0.0];



fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow =       //call window for Rule  
        WindowSettings::new("Snake_game_rust", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut rule = Rule::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            rule.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            rule.make(&c, g);
        });

        event.update(|arg| {
            rule.update(arg.dt);
        });
    }
}
