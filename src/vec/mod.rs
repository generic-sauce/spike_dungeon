use sfml::system::Vector2f;
use std::cmp::{max, min};
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Debug, Error, Formatter};

pub const TILESIZE: i32 = 32;

mod tilevec;
mod worldvec;

pub use self::tilevec::TileVec;
pub use self::worldvec::WorldVec;

impl From<TileVec> for WorldVec {
	fn from(v: TileVec) -> WorldVec {
		WorldVec {
			x: v.x * TILESIZE,
			y: v.y * TILESIZE,
		}
	}
}

impl From<WorldVec> for TileVec {
	fn from(v: WorldVec) -> TileVec {
		TileVec {
			x: v.x / TILESIZE,
			y: v.y / TILESIZE,
		}
	}
}

impl From<WorldVec> for Vector2f {
	fn from(v: WorldVec) -> Vector2f {
		Vector2f::new(v.x as f32, v.y as f32)
	}
}

macro_rules! init_vec {
	($name:ident) => {
		impl $name {
			pub const fn new(x: i32, y: i32) -> $name {
				$name { x, y }
			}

			pub const fn with(xy: i32) -> $name {
				$name { x: xy, y: xy }
			}

			pub fn max(self, v: $name) -> $name {
				$name { x: max(v.x, self.x), y: max(v.y, self.y) }
			}

			pub fn min(self, v: $name) -> $name {
				$name { x: min(v.x, self.x), y: min(v.y, self.y) }
			}

			pub fn length(self) -> i32 {
				((self.x * self.x + self.y * self.y) as f32).sqrt() as i32
			}
		}

		impl Default for $name {
			fn default() -> Self {
				Self { x: 0, y: 0, }
			}
		}

		impl Add<$name> for $name {
			type Output = $name;

			fn add(self, other: $name) -> $name {
				$name::new (
					self.x + other.x,
					self.y + other.y,
				)
			}
		}

		impl Add<i32> for $name {
			type Output = $name;

			fn add(self, other: i32) -> $name {
				$name::new (
					self.x + other,
					self.y + other,
				)
			}
		}

		impl Sub<$name> for $name {
			type Output = $name;

			fn sub(self, other: $name) -> $name {
				$name::new (
					self.x - other.x,
					self.y - other.y,
				)
			}
		}

		impl Sub<i32> for $name {
			type Output = $name;

			fn sub(self, other: i32) -> $name {
				$name::new (
					self.x - other,
					self.y - other,
				)
			}
		}

		impl Mul<$name> for $name {
			type Output = $name;

			fn mul(self, other: $name) -> $name {
				$name::new (
					self.x * other.x,
					self.y * other.y,
				)
			}
		}

		impl Mul<i32> for $name {
			type Output = $name;

			fn mul(self, other: i32) -> $name {
				$name::new (
					self.x * other,
					self.y * other,
				)
			}
		}

		impl Div<$name> for $name {
			type Output = $name;

			fn div(self, other: $name) -> $name {
				$name::new (
					self.x / other.x,
					self.y / other.y,
				)
			}
		}

		impl Div<i32> for $name {
			type Output = $name;

			fn div(self, other: i32) -> $name {
				$name::new (
					self.x / other,
					self.y / other,
				)
			}
		}

		impl Display for $name {
			fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
				let s = format!("$name({}, {})", self.x, self.y);
				fmt.write_str(&*s)
			}
		}

		impl Debug for $name {
			fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
				let s = format!("$name({:?}, {:?})", self.x, self.y);
				fmt.write_str(&*s)
			}
		}

	}
}

init_vec!(WorldVec);
init_vec!(TileVec);
