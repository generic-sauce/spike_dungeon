use sfml::system::{Vector2f};
use sfml::graphics::{RenderTarget, RenderWindow, Color, RectangleShape, Shape, Transformable, Sprite, CircleShape};
use sfml::window::{VideoMode, Style, Event};

use crate::app::App;
use crate::world::World;
use crate::vec::WorldVec;
use crate::vec::WorldCoord;
use crate::vec::TileVec;
use crate::vec::TileCoord;

const PLAYER_RADIUS: f32 = 32.0;

use crate::world::tilemap::TILEMAP_SIZE;
use crate::world::TILESIZE_F;

impl App {
    pub fn render_tile(&mut self, pos: TileVec, color: Color) {
        let mut tile = RectangleShape::with_size(Vector2f::new(TILESIZE_F, TILESIZE_F));
        tile.set_fill_color(color);
        tile.set_position(Vector2f::new(pos.x.to_world_coord().0 as f32, pos.y.to_world_coord().0 as f32));
        self.window.draw(&tile);
    }

    pub fn render_tiles(&mut self) {
        for y in 0..TILEMAP_SIZE.x.0 as usize {
            for x in 0..TILEMAP_SIZE.y.0 as usize {
				let c = self.world.tilemap.tiles[x][y].get_color().clone();
                self.render_tile(TileVec::new(TileCoord::new(x as i32), TileCoord::new(y as i32)), c);
            }
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
            player_circle.set_position(Vector2f::new(player.position.x.0 as f32, player.position.y.0 as f32));
            player_circle.set_origin(Vector2f::new(PLAYER_RADIUS, PLAYER_RADIUS));
            self.window.draw(&player_circle);
        }
    }

    pub fn render_bullets(&mut self) {
        for bullet in self.world.bullets.iter() {
            let mut bullet_circle: CircleShape = CircleShape::new(bullet.radius as f32, 8);
            bullet_circle.set_position(Vector2f::new(bullet.position.x.0 as f32, bullet.position.y.0 as f32));
            bullet_circle.set_origin(Vector2f::new(bullet.radius as f32, bullet.radius as f32));
            self.window.draw(&bullet_circle);
        }
    }
}
