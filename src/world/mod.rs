mod player;
mod tilemap;

use player::Player;
use tilemap::TileMap;
use crate::app::controller::Controller;

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

    pub fn update(&mut self, controller: &Controller) {
        self.players[0].position = self.players[0].position + controller.direction;
    }
}
