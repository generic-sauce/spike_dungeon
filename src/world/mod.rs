use std::cmp::{max, min};

use crate::world::tilemap::Tile;
pub use crate::vec::TILESIZE;

pub mod player;
pub mod tilemap;

use player::Player;
use tilemap::TileMap;
use crate::app::controller::Controller;

pub const TILESIZE_F: f32 = TILESIZE as f32;

pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(), Player::new()],
			tilemap: TileMap::from_filename("./map.png"),
		}
	}

    pub fn update(&mut self, controller: &Controller) {
        for (i, player) in self.players.iter_mut().enumerate() {
            player.position = player.position + controller.get_direction(i);
        }
		self.deglitch_players();
    }

	fn deglitch_players(&mut self) {
        for (i, player) in self.players.iter_mut().enumerate() {
			for x in (player.position.x.round_tile_coord().0-4)..(player.position.x.round_tile_coord().0+4) {
				for y in (player.position.y.round_tile_coord().0-4)..(player.position.y.round_tile_coord().0+4) {
					if x < 0 || x >= 32 { continue; } // TODO un-hardcode
					if y < 0 || y >= 32 { continue; }
					if self.tilemap.tiles[x as usize][y as usize] != Tile::WALL { continue; }
					let closest_tile_x = min((x+1) * TILESIZE, max(x * TILESIZE, player.position.x.0));
					let closest_tile_y = min((y+1) * TILESIZE, max(y * TILESIZE, player.position.y.0));
					let distx = closest_tile_x - player.position.x.0;
					let disty = closest_tile_y - player.position.y.0;
					let dist_sqr = (distx * distx) + (disty * disty);

					if dist_sqr <= 32*32 {
						player.position.x.0 = player.position.x.0 - distx;
						player.position.y.0 = player.position.y.0 - disty;
					}
					
				}
			}
        }
	}
}
