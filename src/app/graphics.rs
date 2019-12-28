use sfml::system::{Vector2f};
use sfml::graphics::{RenderTarget, RenderWindow, Color, RectangleShape, Shape, Transformable, Sprite, CircleShape};
use sfml::window::{VideoMode, Style, Event};

use crate::app::App;
use crate::world::World;
use crate::vec::WorldVec;
use crate::vec::WorldCoord;
use crate::vec::TileVec;
use crate::vec::TileCoord;

use crate::world::tilemap::SIZE;
use crate::world::TILE_SIZE_F;

impl App {
    pub fn render_tile(&mut self, pos: TileVec, color: Color) {
        let mut tile = RectangleShape::with_size(Vector2f::new(TILE_SIZE_F, TILE_SIZE_F));
        tile.set_fill_color(color);
        tile.set_position(Vector2f::new(pos.x.0 as f32 * TILE_SIZE_F, pos.y.0 as f32 * TILE_SIZE_F));
        self.window.draw(&tile);
    }

    pub fn render_tiles(&mut self) {

        for y in 0..SIZE.x.0 as usize {
            for x in 0..SIZE.y.0 as usize {
				let c = self.world.tilemap.tiles[x][y].get_color().clone();
                self.render_tile(TileVec::new(TileCoord::new(x as i32), TileCoord::new(y as i32)), c);
            }
        }
    }

    pub fn render(&mut self) -> () {
        self.render_tiles();
        self.render_players();
    }

    pub fn render_players(&mut self) {
        for player in self.world.players.iter() {
            let mut player_circle: CircleShape = CircleShape::new(32.0, 16);
            player_circle.set_position(Vector2f::new(player.position.x.0 as f32, player.position.y.0 as f32));
            self.window.draw(&player_circle);
        }
    }
}
