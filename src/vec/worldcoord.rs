use crate::vec::{TileCoord, WORLD_FACTOR};

use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct WorldCoord(pub i32);

impl Add for WorldCoord {
	type Output = WorldCoord;

	fn add(self, other: Self) -> Self {
		Self(self.0 + other.0)
	}
}

impl Mul for WorldCoord {
	type Output = WorldCoord;

	fn mul(self, other: Self) -> Self {
		Self(self.0 * other.0)
	}
}

impl Sub for WorldCoord {
	type Output = WorldCoord;

	fn sub(self, other: Self) -> Self {
		Self(self.0 - other.0)
	}
}

impl WorldCoord {
	pub const fn new(x: i32) -> WorldCoord { Self(x) }

	fn round_tile_coord(self) -> TileCoord {
		TileCoord::new(self.0 / WORLD_FACTOR)
	}
}
