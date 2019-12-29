use sfml::system::Vector2f;

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
