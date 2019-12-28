use std::mem::MaybeUninit;

#[derive(Clone, Copy)]
pub enum Tile {
	GROUND,
	WALL,
	SPIKE
}

pub struct TileMap {
	tiles: [[Tile; 32]; 32],
}

impl TileMap {
	pub fn new() -> TileMap {
        let tiles = [[Tile::GROUND; 32]; 32];
		TileMap { tiles }
	}
}
