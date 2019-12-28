use sfml::graphics::{RenderTarget, RenderWindow, Color};
use sfml::window::{VideoMode, Style, Event};

pub struct App {
    window: RenderWindow
}

impl App {
    pub fn new() -> App {
        App { window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Toa client", Style::FULLSCREEN | Style::CLOSE, &Default::default()) }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                if event == Event::Closed {
                    self.window.close();
                }
            }

            self.window.display();
            self.window.clear(Color::rgb(0, 0, 0));

            std::thread::sleep(std::time::Duration::from_millis(40));
        }
    }
}
