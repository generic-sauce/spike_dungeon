use sfml::system::{Vector2f};
use sfml::graphics::{RenderTarget, RenderWindow, Color, View, FloatRect};
use sfml::window::{VideoMode, Style, Event};

use crate::world::World;
use crate::app::controller::Controller;
use crate::vec::TileVec;
use crate::vec::TileCoord;
use crate::world::WORLD_SIZE;

mod graphics;
pub mod controller;

pub struct App {
    window: RenderWindow,
	#[allow(dead_code)]
	world: World,
    controller: Controller
}

impl App {
    pub fn new() -> App {
        App {
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "spike dungeon", Style::DEFAULT | Style::CLOSE, &Default::default()),
			world: World::new(),
            controller: Controller::new(),
		}
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                if event == Event::Closed {
                    self.window.close();
                    break;
                }
            }

            let window_size = Vector2f::new(self.window.size().x as f32, self.window.size().y as f32);
            let aspect_ratio = window_size.x / window_size.y;
            let border_width_x = 0f32.max(window_size.x - window_size.y) / 2.0;
            let border_width_y = 0f32.max(window_size.y - window_size.x) / 2.0;
            let scale = window_size.y / WORLD_SIZE.y.0 as f32;
            self.window.set_view(
                &View::from_rect(
                    &FloatRect::new(
                        -border_width_x / scale,
                        -border_width_y / scale,
                        WORLD_SIZE.x.0 as f32 + border_width_x * 2.0 / scale,
                        WORLD_SIZE.y.0 as f32 + border_width_y * 2.0 / scale)));

            self.controller.update();
            self.world.update(&self.controller);

            self.render();

            self.window.display();
            self.window.clear(Color::rgb(0, 0, 0));

            std::thread::sleep(std::time::Duration::from_millis(40));
        }
    }
}
