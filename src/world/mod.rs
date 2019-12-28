mod player;

use player::Player;

pub struct World {
	players: [Player; 2]
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(), Player::new()]
		}
	}
}
