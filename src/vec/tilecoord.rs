use crate::vec::{WorldCoord, WORLD_FACTOR};

use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TileCoord(i32);

impl Add for TileCoord {
	type Output = TileCoord;

	fn add(self, other: Self) -> Self {
		Self(self.0 + other.0)
	}
}

impl Mul for TileCoord {
	type Output = TileCoord;

	fn mul(self, other: Self) -> Self {
		Self(self.0 * other.0)
	}
}

impl Sub for TileCoord {
	type Output = TileCoord;

	fn sub(self, other: Self) -> Self {
		Self(self.0 - other.0)
	}
}

impl TileCoord {
	pub fn new(x: i32) -> TileCoord { Self(x) }

	fn to_world_coord(self) -> WorldCoord {
		WorldCoord::new(self.0 * WORLD_FACTOR)
	}
}

