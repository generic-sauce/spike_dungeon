pub mod player;
pub mod tilemap;

use player::Player;
use tilemap::TileMap;
use crate::app::controller::Controller;

pub const TILE_SIZE: i32 = 32;
pub const TILE_SIZE_F: f32 = TILE_SIZE as f32;

pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(), Player::new()],
			tilemap: TileMap::new(),
		}
	}

    pub fn update(&mut self, controller: &Controller) {
        for (i, player) in self.players.iter_mut().enumerate() {
            player.position = player.position + controller.get_direction(i);
        }
    }
}
