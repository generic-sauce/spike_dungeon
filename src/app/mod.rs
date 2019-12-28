use sfml::graphics::{RenderTarget, RenderWindow, Color};
use sfml::window::{VideoMode, Style, Event};

use crate::world::World;
use crate::app::controller::Controller;

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
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "spike dungeon", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
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

            self.controller.update();
            self.world.update(&self.controller);

            self.render_tiles();

            self.window.display();
            self.window.clear(Color::rgb(0, 0, 0));

            std::thread::sleep(std::time::Duration::from_millis(40));
        }
    }
}
