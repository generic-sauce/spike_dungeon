extern crate sfml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;

mod app;
mod vec;
mod world;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
