use std::mem::MaybeUninit;
use sfml::system::{Vector2f};

use crate::vec::TileCoord;
use crate::vec::TileVec;

pub const SIZE: TileVec = TileVec::new(TileCoord::new(32), TileCoord::new(32));
lazy_static! { static ref SIZE_F: Vector2f = Vector2f::new(SIZE.x.0 as f32, SIZE.y.0 as f32); }

#[derive(Clone, Copy)]
pub enum Tile {
	GROUND,
	WALL,
	SPIKE
}

pub struct TileMap {
	pub tiles: [[Tile; SIZE.y.0 as usize]; SIZE.x.0 as usize],
}

impl TileMap {
	pub fn new() -> TileMap {
        let tiles = [[Tile::GROUND; 32]; 32];
		TileMap { tiles }
	}
}
