use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Debug, Error, Formatter};
use std::cmp::{min, max};

use crate::world::TILEMAP_SIZE;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileVec {
	pub x: i32,
	pub y: i32,
}

impl TileVec {
	pub const fn new(x: i32, y: i32) -> TileVec {
		TileVec { x, y }
	}

	pub const fn with(xy: i32) -> TileVec {
		TileVec { x: xy, y: xy }
	}

	pub fn max(self, v: TileVec) -> TileVec {
		TileVec { x: max(v.x, self.x), y: max(v.y, self.y) }
	}

	pub fn min(self, v: TileVec) -> TileVec {
		TileVec { x: min(v.x, self.x), y: min(v.y, self.y) }
	}

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

impl Default for TileVec {
	fn default() -> Self {
		Self { x: 0, y: 0, }
	}
}

impl Add<TileVec> for TileVec {
	type Output = TileVec;

	fn add(self, other: TileVec) -> TileVec {
		TileVec::new (
			self.x + other.x,
			self.y + other.y,
		)
	}
}

impl Add<i32> for TileVec {
	type Output = TileVec;

	fn add(self, other: i32) -> TileVec {
		TileVec::new (
			self.x + other,
			self.y + other,
		)
	}
}

impl Sub<TileVec> for TileVec {
	type Output = TileVec;

	fn sub(self, other: TileVec) -> TileVec {
		TileVec::new (
			self.x - other.x,
			self.y - other.y,
		)
	}
}

impl Sub<i32> for TileVec {
	type Output = TileVec;

	fn sub(self, other: i32) -> TileVec {
		TileVec::new (
			self.x - other,
			self.y - other,
		)
	}
}

impl Mul<TileVec> for TileVec {
	type Output = TileVec;

	fn mul(self, other: TileVec) -> TileVec {
		TileVec::new (
			self.x * other.x,
			self.y * other.y,
		)
	}
}

impl Mul<i32> for TileVec {
	type Output = TileVec;

	fn mul(self, other: i32) -> TileVec {
		TileVec::new (
			self.x * other,
			self.y * other,
		)
	}
}

impl Div<TileVec> for TileVec {
	type Output = TileVec;

	fn div(self, other: TileVec) -> TileVec {
		TileVec::new (
			self.x / other.x,
			self.y / other.y,
		)
	}
}

impl Div<i32> for TileVec {
	type Output = TileVec;

	fn div(self, other: i32) -> TileVec {
		TileVec::new (
			self.x / other,
			self.y / other,
		)
	}
}

impl Display for TileVec {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("TileVec({}, {})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl Debug for TileVec {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("TileVec({:?}, {:?})", self.x, self.y);
		fmt.write_str(&*s)
	}
}
