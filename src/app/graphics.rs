use sfml::system::Vector2f;
use sfml::graphics::{RenderTarget, RenderWindow, Color, RectangleShape, Shape, Transformable, Sprite, CircleShape};
use sfml::window::{VideoMode, Style, Event};

use crate::app::App;
use crate::world::World;
use crate::vec::{TileVec, WorldVec};

const PLAYER_RADIUS: f32 = 32.0;

use crate::world::{TILEMAP_SIZE, TILESIZE};

impl App {
    pub fn render_tile(&mut self, pos: TileVec, color: Color) {
        let mut tile = RectangleShape::with_size(Vector2f::new(TILESIZE as f32, TILESIZE as f32));
        tile.set_fill_color(color);
        tile.set_position(Vector2f::from(WorldVec::from(pos)));
        self.window.draw(&tile);
    }

    pub fn render_tiles(&mut self) {
		for p in TileVec::iter_all() {
			let c = self.world.tilemap.get_tile(p).get_color().clone();
            self.render_tile(p, c);
        }
    }

    pub fn render(&mut self) -> () {
        self.render_tiles();
        self.render_players();
        self.render_bullets();
    }

    pub fn render_players(&mut self) {
        for player in self.world.players.iter() {
            let mut player_circle: CircleShape = CircleShape::new(PLAYER_RADIUS, 16);
            player_circle.set_position(Vector2f::from(WorldVec::from(player.position)));
            player_circle.set_origin(Vector2f::new(PLAYER_RADIUS, PLAYER_RADIUS));
            self.window.draw(&player_circle);
        }
    }

    pub fn render_bullets(&mut self) {
        for bullet in self.world.bullets.iter() {
            let mut bullet_circle: CircleShape = CircleShape::new(bullet.radius as f32, 8);
            bullet_circle.set_position(Vector2f::from(WorldVec::from(bullet.position)));
            bullet_circle.set_origin(Vector2f::new(bullet.radius as f32, bullet.radius as f32));
            self.window.draw(&bullet_circle);
        }
    }
}
