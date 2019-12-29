use crate::vec::{WorldCoord, TILESIZE};

use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TileCoord(pub i32);

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
	pub const fn new(x: i32) -> TileCoord { Self(x) }

	pub fn to_world_coord(self) -> WorldCoord {
		WorldCoord::new(self.0 * TILESIZE)
	}
}

