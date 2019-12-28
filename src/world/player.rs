use crate::vec::{WorldCoord, WorldVec};

pub struct Player {
	pos: WorldVec,
}

impl Player {
	pub fn new() -> Player {
		Player {
			pos: WorldVec::with(WorldCoord::new(0))
		}
	}
}
