use crate::vec::{WorldCoord, WorldVec};

pub struct Player {
	pub position: WorldVec,
}

impl Player {
	pub fn new() -> Player {
		Player {
			position: WorldVec::with(WorldCoord::new(0))
		}
	}
}
