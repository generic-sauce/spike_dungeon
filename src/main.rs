extern crate sfml;
#[macro_use]
extern crate lazy_static;

mod app;
mod vec;
mod world;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
