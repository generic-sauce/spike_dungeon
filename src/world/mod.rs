mod player;
mod tilemap;

use player::Player;
use tilemap::TileMap;

pub struct World {
	players: [Player; 2],
	tilemap: TileMap,
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(), Player::new()],
			tilemap: TileMap::new(),
		}
	}
}
