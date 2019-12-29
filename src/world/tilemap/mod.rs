use sfml::system::{Vector2f};
use sfml::graphics::{Image, Color};

use crate::vec::{TileVec, TileCoord};

pub const TILEMAP_SIZE: TileVec = TileVec::new(TileCoord::new(32), TileCoord::new(32));
lazy_static! { static ref TILEMAP_SIZE_F: Vector2f = Vector2f::new(TILEMAP_SIZE.x.0 as f32, TILEMAP_SIZE.y.0 as f32); }

static TILES: [Tile; 3] = [Tile::GROUND, Tile::WALL, Tile::SPIKE];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
	GROUND,
	WALL,
	SPIKE
}

impl Tile {
	pub fn get_color(&self) -> Color {
		match self {
			Tile::GROUND => Color::rgb(100, 100, 0),
			Tile::WALL => Color::rgb(0, 0, 0),
			Tile::SPIKE => Color::rgb(255, 255, 255),
		}
	}
}

pub struct TileMap {
	pub tiles: [[Tile; TILEMAP_SIZE.y.0 as usize]; TILEMAP_SIZE.x.0 as usize],
}

impl TileMap {
	pub fn new() -> TileMap {
        let tiles = [[Tile::GROUND; 32]; 32];
		TileMap { tiles }
	}

	pub fn from_filename(filename: &str) -> TileMap {
		let img = Image::from_file(filename).unwrap();
		assert!(img.size().x == 32);
		assert!(img.size().y == 32);

        let mut tiles = [[Tile::GROUND; 32]; 32];
		
		for p in iter_tiles() {
			let c = img.pixel_at(p.x.0 as u32, p.y.0 as u32);
			let t = TILES.iter().find(|t| t.get_color() == c).unwrap();
			tiles[p.x.0 as usize][p.y.0 as usize] = *t;
		}
		TileMap { tiles }
	}

	pub fn set_tile(&mut self, pos: TileVec, t: Tile) {
		self.tiles[pos.x.0 as usize][pos.y.0 as usize] = t;
	}
}

pub fn iter_tiles() -> impl Iterator<Item=TileVec> {
	iproduct!(0..32, 0..32)
		.map(|(x, y)| TileVec::new(TileCoord(x), TileCoord(y)))
}

