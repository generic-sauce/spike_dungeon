use sfml::system::{Vector2f};
use sfml::graphics::{RenderTarget, RenderWindow, Color, CircleShape, Transformable};
use sfml::window::{VideoMode, Style, Event};

use crate::app::App;

impl App {
    pub fn render(&mut self) -> () {
        let mut circle: CircleShape = CircleShape::new(64.0, 32);
        circle.set_position(Vector2f::new(200.0, 200.0));
        self.window.draw(&circle);
    }
}
