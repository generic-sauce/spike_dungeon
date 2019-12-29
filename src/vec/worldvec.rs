use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Debug, Error, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldVec {
	pub x: i32,
	pub y: i32,
}

impl WorldVec {
	pub const fn new(x: i32, y: i32) -> WorldVec {
		WorldVec { x, y }
	}

	pub const fn with(xy: i32) -> WorldVec {
		WorldVec { x: xy, y: xy }
	}
}

impl Default for WorldVec {
	fn default() -> Self {
		Self { x: 0, y: 0, }
	}
}

impl Add<WorldVec> for WorldVec {
	type Output = WorldVec;

	fn add(self, other: WorldVec) -> WorldVec {
		WorldVec::new (
			self.x + other.x,
			self.y + other.y,
		)
	}
}

impl Add<i32> for WorldVec {
	type Output = WorldVec;

	fn add(self, other: i32) -> WorldVec {
		WorldVec::new (
			self.x + other,
			self.y + other,
		)
	}
}

impl Sub<WorldVec> for WorldVec {
	type Output = WorldVec;

	fn sub(self, other: WorldVec) -> WorldVec {
		WorldVec::new (
			self.x - other.x,
			self.y - other.y,
		)
	}
}

impl Sub<i32> for WorldVec {
	type Output = WorldVec;

	fn sub(self, other: i32) -> WorldVec {
		WorldVec::new (
			self.x - other,
			self.y - other,
		)
	}
}

impl Mul<WorldVec> for WorldVec {
	type Output = WorldVec;

	fn mul(self, other: WorldVec) -> WorldVec {
		WorldVec::new (
			self.x * other.x,
			self.y * other.y,
		)
	}
}

impl Mul<i32> for WorldVec {
	type Output = WorldVec;

	fn mul(self, other: i32) -> WorldVec {
		WorldVec::new (
			self.x * other,
			self.y * other,
		)
	}
}

impl Div<WorldVec> for WorldVec {
	type Output = WorldVec;

	fn div(self, other: WorldVec) -> WorldVec {
		WorldVec::new (
			self.x / other.x,
			self.y / other.y,
		)
	}
}

impl Div<i32> for WorldVec {
	type Output = WorldVec;

	fn div(self, other: i32) -> WorldVec {
		WorldVec::new (
			self.x / other,
			self.y / other,
		)
	}
}

impl Display for WorldVec {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("WorldVec({}, {})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl Debug for WorldVec {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("WorldVec({:?}, {:?})", self.x, self.y);
		fmt.write_str(&*s)
	}
}
