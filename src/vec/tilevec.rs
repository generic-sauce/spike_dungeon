use std::cmp::{min, max};

use crate::world::TILEMAP_SIZE;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileVec {
	pub x: i32,
	pub y: i32,
}

impl TileVec {
	pub fn iter_all() -> impl Iterator<Item=TileVec> {
		iproduct!(0..TILEMAP_SIZE.x, 0..TILEMAP_SIZE.y)
			.map(|(x, y)| TileVec::new(x, y))
	}

	pub fn iter_around(self, range: i32) -> impl Iterator<Item=TileVec> {
		let xmin = max(0, self.x - range);
		let ymin = max(0, self.y - range);

		let xmax = min(TILEMAP_SIZE.x, self.x + range + 1);
		let ymax = min(TILEMAP_SIZE.y, self.y + range + 1);

		iproduct!(xmin..xmax, ymin..ymax)
			.map(|(x, y)| TileVec::new(x, y))
	}
}
